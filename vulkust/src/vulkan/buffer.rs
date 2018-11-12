use super::super::core::allocate as alc;
use super::super::core::allocate::{Allocator, Object};
use super::command::{Buffer as CmdBuffer, Pool as CmdPool};
use super::device::logical::Logical as LogicalDevice;
use super::image::Image;
use super::memory::{Location as MemoryLocation, Manager as MemoryManager, Memory};
use super::vulkan as vk;
use libc;
use std::mem::{size_of, transmute};
use std::os::raw::c_void;
use std::ptr::null;
use std::sync::{Arc, RwLock};

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Buffer {
    memory_offset: isize,
    info: alc::Container,
    vk_data: vk::VkBuffer,
}

impl Buffer {
    pub fn new(size: isize, memory_offset: isize, vk_data: vk::VkBuffer, alignment: isize) -> Self {
        let info = alc::Container::new(size, alignment);
        Buffer {
            memory_offset,
            info,
            vk_data,
        }
    }

    pub fn allocate(&mut self, size: isize) -> Arc<RwLock<Buffer>> {
        let buffer = Arc::new(RwLock::new(Buffer::new(
            size,
            self.memory_offset,
            self.vk_data,
            self.info.get_allocated_memory().get_offset_alignment(),
        )));
        let obj: Arc<RwLock<Object>> = buffer.clone();
        self.info.allocate(&obj);
        return buffer;
    }

    pub(super) fn get_data(&self) -> vk::VkBuffer {
        return self.vk_data;
    }
}

impl Object for Buffer {
    fn get_allocated_memory(&self) -> &alc::Memory {
        return self.info.get_allocated_memory();
    }

    fn place(&mut self, offset: isize) {
        self.info.place(offset);
    }
}

impl Allocator for Buffer {
    fn allocate(&mut self, obj: &Arc<RwLock<Object>>) {
        self.info.allocate(obj);
        vxunimplemented!();
    }

    fn clean(&mut self) {
        self.info.clean();
        vxunimplemented!();
    }
}

unsafe impl Send for Buffer {}

unsafe impl Sync for Buffer {}

#[cfg_attr(debug_mode, derive(Debug))]
pub enum Location {
    CPU,
    GPU,
}

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct RootBuffer {
    logical_device: Arc<LogicalDevice>,
    memory: Arc<RwLock<Memory>>,
    vk_data: vk::VkBuffer,
    container: alc::Container,
    location: Location,
}

impl RootBuffer {
    pub(crate) fn new(size: isize, location: Location, memmgr: &Arc<RwLock<MemoryManager>>) -> Self {
        let (memloc, usage) = match location {
            Location::CPU => (
                MemoryLocation::CPU,
                vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_SRC_BIT as u32
                    | vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_VERTEX_BUFFER_BIT as u32
                    | vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_INDEX_BUFFER_BIT as u32
                    | vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT as u32,
            ),
            Location::GPU => (
                MemoryLocation::GPU,
                vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_VERTEX_BUFFER_BIT as u32
                    | vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_INDEX_BUFFER_BIT as u32
                    | vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT as u32
                    | vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_DST_BIT as u32,
            ),
        };
        let logical_device = vxresult!(memmgr.read()).get_device().clone();
        let mut buffer_info = vk::VkBufferCreateInfo::default();
        buffer_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO;
        buffer_info.size = size as vk::VkDeviceSize;
        buffer_info.usage = usage;
        let mut vk_data = 0 as vk::VkBuffer;
        vulkan_check!(vk::vkCreateBuffer(
            logical_device.vk_data,
            &buffer_info,
            null(),
            &mut vk_data,
        ));
        let mut mem_reqs = vk::VkMemoryRequirements::default();
        unsafe {
            vk::vkGetBufferMemoryRequirements(logical_device.vk_data, vk_data, &mut mem_reqs);
        }
        let memory = vxresult!(memmgr.write()).allocate(&mem_reqs, memloc);
        {
            let mem = vxresult!(memory.read());
            vulkan_check!(vk::vkBindBufferMemory(
                logical_device.vk_data,
                vk_data,
                mem.get_data(),
                mem.get_allocated_memory().get_offset() as vk::VkDeviceSize,
            ));
        }
        let container = alc::Container::new(size, logical_device.get_uniform_buffer_alignment());
        RootBuffer {
            logical_device,
            memory,
            vk_data,
            container,
            location,
        }
    }

    pub(crate) fn allocate(&mut self, size: isize) -> Arc<RwLock<Buffer>> {
        let memoff = vxresult!(self.memory.read()).get_allocated_memory().get_offset();
        let buffer = Arc::new(RwLock::new(Buffer::new(size, memoff, self.vk_data, 
            match self.location {
                Location::CPU => self.logical_device.get_uniform_buffer_alignment(),
                _ => 1,
            })));
        let obj: Arc<RwLock<Object>> = buffer.clone();
        self.container.allocate(&obj);
        return buffer;
    }

    pub(crate) fn get_memory(&self) -> &Arc<RwLock<Memory>> {
        return &self.memory;
    }
}

impl Drop for RootBuffer {
    fn drop(&mut self) {
        unsafe {
            vk::vkDestroyBuffer(self.logical_device.vk_data, self.vk_data, null());
        }
    }
}

unsafe impl Send for RootBuffer {}

unsafe impl Sync for RootBuffer {}

#[derive(Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct StaticBuffer {
    buffer: Arc<RwLock<Buffer>>,
}

impl StaticBuffer {
    pub(crate) fn new(buffer: Arc<RwLock<Buffer>>) -> Self {
        StaticBuffer { buffer }
    }

    pub(crate) fn get_buffer(&self) -> &Arc<RwLock<Buffer>> {
        return &self.buffer;
    }
}

#[derive(Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct DynamicBuffer {
    buffers: Vec<(Arc<RwLock<Buffer>>, isize)>,
    actual_size: isize,
}

impl DynamicBuffer {
    pub(crate) fn new(buffers: Vec<(Arc<RwLock<Buffer>>, isize)>, actual_size: isize) -> Self {
        DynamicBuffer {
            buffers,
            actual_size,
        }
    }

    pub(crate) fn update_with_ptr(&mut self, data: *const c_void, frame_number: usize) {
        let ptr = self.buffers[frame_number].1;
        unsafe {
            libc::memcpy(transmute(ptr), transmute(data), self.actual_size as usize);
        }
    }

    pub(crate) fn update<T>(&mut self, data: &T, frame_number: usize)
    where
        T: Sized,
    {
        #[cfg(debug_mode)]
        {
            if size_of::<T>() != self.actual_size as usize {
                vxlogf!("Data must have same size of buffer.");
            }
        }
        self.update_with_ptr(unsafe { transmute(data) }, frame_number);
    }

    pub(crate) fn get_buffer(&self, frame_number: usize) -> &Arc<RwLock<Buffer>> {
        return &self.buffers[frame_number].0;
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct Manager {
    cpu_buffer: RootBuffer,
    gpu_buffer: RootBuffer,
    cpu_memory_mapped_ptr: isize,
    static_buffer: Arc<RwLock<Buffer>>,
    static_uploader_buffer: Arc<RwLock<Buffer>>,
    dynamic_buffers: Vec<Arc<RwLock<Buffer>>>,
    copy_ranges: Vec<vk::VkBufferCopy>,
    copy_buffers: Vec<Arc<RwLock<Buffer>>>,
    copy_to_image_ranges: Vec<(vk::VkBufferImageCopy, Arc<RwLock<Image>>)>,
    frame_copy_buffers: Vec<Vec<Arc<RwLock<Buffer>>>>,
    frame_copy_to_image_ranges: Vec<Vec<(vk::VkBufferImageCopy, Arc<RwLock<Image>>)>>,
    cmd_pool: Arc<CmdPool>,
}

impl Manager {
    pub(crate) fn new(
        memmgr: &Arc<RwLock<MemoryManager>>,
        cmd_pool: &Arc<CmdPool>,
        static_size: isize,
        static_uploader_size: isize,
        dynamics_size: isize,
        frames_count: isize,
    ) -> Self {
        let mut cpu_buffer = RootBuffer::new(
            dynamics_size * frames_count + static_uploader_size,
            Location::CPU,
            memmgr,
        );
        let mut gpu_buffer = RootBuffer::new(static_size, Location::GPU, memmgr);
        let static_buffer = gpu_buffer.allocate(static_size);
        let static_uploader_buffer = cpu_buffer.allocate(static_uploader_size);
        
        let vk_device = vxresult!(memmgr.read()).get_device().vk_data;
        let (memory_size, vk_memory) = {
            let cpu_memory = vxresult!(cpu_buffer.memory.read());
            let vk_memory = cpu_memory.get_data();
            let size = vxresult!(cpu_memory.get_root().read()).get_size();
            (size, vk_memory)
        };
        let cpu_memory_mapped_ptr = unsafe {
            let mut data_ptr = 0 as *mut c_void;
            vulkan_check!(vk::vkMapMemory(
                vk_device,
                vk_memory,
                0,
                memory_size as u64,
                0,
                &mut data_ptr
            ));
            transmute(data_ptr)
        };
        let mut dynamic_buffers = Vec::with_capacity(frames_count as usize);
        for _ in 0..frames_count {
            dynamic_buffers.push(cpu_buffer.allocate(dynamics_size));
        }
        let copy_buffers = Vec::new();
        let copy_ranges = Vec::new();
        let copy_to_image_ranges = Vec::new();
        let mut frame_copy_buffers = Vec::new();
        let mut frame_copy_to_image_ranges = Vec::new();
        for _ in 0..frames_count {
            frame_copy_buffers.push(Vec::new());
            frame_copy_to_image_ranges.push(Vec::new());
        }
        frame_copy_buffers.shrink_to_fit();
        frame_copy_to_image_ranges.shrink_to_fit();
        let cmd_pool = cmd_pool.clone();
        Manager {
            cpu_buffer,
            gpu_buffer,
            cpu_memory_mapped_ptr,
            static_buffer,
            static_uploader_buffer,
            dynamic_buffers,
            copy_buffers,
            copy_ranges,
            copy_to_image_ranges,
            cmd_pool,
            frame_copy_buffers,
            frame_copy_to_image_ranges,
        }
    }

    pub(crate) fn create_static_buffer_with_ptr(
        &mut self,
        data: *const c_void,
        data_len: usize,
    ) -> StaticBuffer {
        let buffer = vxresult!(self.static_buffer.write()).allocate(data_len as isize);
        let upbuff = self.create_staging_buffer_with_ptr(data, data_len as usize);
        let upbuffer = vxresult!(upbuff.read());
        let mut range = vk::VkBufferCopy::default();
        range.srcOffset = upbuffer.get_allocated_memory().get_offset() as vk::VkDeviceSize;
        range.dstOffset = vxresult!(buffer.read()).get_allocated_memory().get_offset() 
            as vk::VkDeviceSize;
        range.size = data_len as vk::VkDeviceSize;
        self.copy_ranges.push(range);
        StaticBuffer::new(buffer)
    }

    pub(crate) fn create_static_buffer_with_vec<T>(&mut self, data: &[T]) -> StaticBuffer {
        let data_ptr = unsafe { transmute(data.as_ptr()) };
        let data_len = data.len() * size_of::<T>();
        self.create_static_buffer_with_ptr(data_ptr, data_len)
    }

    pub(crate) fn create_staging_buffer_with_ptr(
        &mut self,
        data: *const c_void,
        data_len: usize,
    ) -> Arc<RwLock<Buffer>> {
        let upbuffer = vxresult!(self.static_uploader_buffer.write()).allocate(data_len as isize);
        let off = {
            let upbuff = vxresult!(upbuffer.read());
            let mut off = upbuff.memory_offset;
            off += upbuff.get_allocated_memory().get_offset();
            off += self.cpu_memory_mapped_ptr;
            off
        };
        unsafe {
            let ptr = transmute(off);
            libc::memcpy(ptr, transmute(data), data_len);
        }
        self.copy_buffers.push(upbuffer.clone());
        upbuffer
    }

    pub(crate) fn create_staging_buffer_with_vec<T>(&mut self, data: &[T]) -> Arc<RwLock<Buffer>> {
        let data_ptr = unsafe { transmute(data.as_ptr()) };
        let data_len = data.len() * size_of::<T>();
        self.create_staging_buffer_with_ptr(data_ptr, data_len)
    }

    pub(crate) fn create_staging_image(
        &mut self,
        image: &Arc<RwLock<Image>>,
        pixels: &[u8],
        img_info: &vk::VkImageCreateInfo,
    ) {
        let upbuff = self.create_staging_buffer_with_vec(pixels);
        let upbuffer = vxresult!(upbuff.read());
        let mut copy_info = vk::VkBufferImageCopy::default();
        copy_info.imageSubresource.aspectMask =
            vk::VkImageAspectFlagBits::VK_IMAGE_ASPECT_COLOR_BIT as u32;
        copy_info.imageSubresource.mipLevel = 0;
        copy_info.imageSubresource.baseArrayLayer = 0;
        copy_info.imageSubresource.layerCount = 1;
        copy_info.imageExtent.width = img_info.extent.width;
        copy_info.imageExtent.height = img_info.extent.height;
        copy_info.imageExtent.depth = img_info.extent.depth;
        copy_info.bufferOffset = upbuffer.get_allocated_memory().get_offset() as vk::VkDeviceSize;
        self.copy_to_image_ranges.push((copy_info, image.clone()));
    }

    pub(crate) fn create_dynamic_buffer(&mut self, actual_size: isize) -> DynamicBuffer {
        let mut buffers = Vec::new();
        for dynamic_buffer in &self.dynamic_buffers {
            let buffer = vxresult!(dynamic_buffer.write()).allocate(actual_size);
            let ptr = {
                let buffer = vxresult!(buffer.read());
                buffer.memory_offset + buffer.get_allocated_memory().get_offset() + self.cpu_memory_mapped_ptr
            };
            buffers.push((buffer, ptr));
        }
        buffers.shrink_to_fit();
        DynamicBuffer::new(buffers, actual_size)
    }

    pub(crate) fn update(&mut self, cmd: &mut CmdBuffer, frame_number: usize) {
        self.frame_copy_buffers[frame_number].clear();
        self.frame_copy_to_image_ranges[frame_number].clear();
        if self.copy_buffers.len() == 0 {
            return;
        }
        if self.copy_ranges.len() != 0 {
            cmd.copy_buffer(
                self.cpu_buffer.vk_data,
                self.gpu_buffer.vk_data,
                &self.copy_ranges,
            );
            self.copy_ranges.clear();
        }
        if self.copy_to_image_ranges.len() != 0 {
            for copy_img in &self.copy_to_image_ranges {
                let mut image = vxresult!(copy_img.1.write());
                image.change_layout(cmd, vk::VkImageLayout::VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL);
            }
            for copy_img in &self.copy_to_image_ranges {
                let image = vxresult!(copy_img.1.read());
                cmd.copy_buffer_to_image(self.cpu_buffer.vk_data, image.get_data(), &copy_img.0);
            }
            for copy_img in &self.copy_to_image_ranges {
                let mut image = vxresult!(copy_img.1.write());
                image.change_layout(
                    cmd,
                    vk::VkImageLayout::VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL,
                );
            }
        }
        self.frame_copy_buffers[frame_number].append(&mut self.copy_buffers);
        self.frame_copy_to_image_ranges[frame_number].append(&mut self.copy_to_image_ranges);
    }

    pub(super) fn get_gpu_root_buffer(&self) -> &RootBuffer {
        return &self.gpu_buffer;
    }
}

unsafe impl Send for Manager {}

unsafe impl Sync for Manager {}
