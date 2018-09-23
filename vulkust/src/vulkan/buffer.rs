use super::super::core::allocate as alc;
use super::super::core::allocate::{Allocator, Object};
use super::command::buffer::Buffer as CmdBuffer;
use super::command::pool::Pool as CmdPool;
use super::device::logical::Logical as LogicalDevice;
use super::image::Image;
use super::memory::{Location as MemoryLocation, Manager as MemoryManager, Memory};
use super::vulkan as vk;
use libc;
use std::mem::{size_of, transmute};
use std::os::raw::c_void;
use std::ptr::null;
use std::sync::{Arc, RwLock};

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Buffer {
    pub memory_offset: isize,
    pub info: alc::Container,
    pub vk_data: vk::VkBuffer,
}

impl Buffer {
    pub fn new(size: isize, memory_offset: isize, vk_data: vk::VkBuffer) -> Self {
        let info = alc::Container::new(size, 2);
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
        )));
        let obj: Arc<RwLock<Object>> = buffer.clone();
        self.info.allocate(&obj);
        return buffer;
    }
}

impl Object for Buffer {
    fn get_size(&self) -> isize {
        self.info.base.size
    }

    fn get_offset(&self) -> isize {
        self.info.base.offset
    }

    fn get_offset_alignment(&self) -> isize {
        self.info.base.offset_alignment
    }

    fn place(&mut self, offset: isize) {
        self.info.place(offset);
    }
}

impl Allocator for Buffer {
    fn increase_size(&mut self, size: isize) {
        self.info.increase_size(size);
        vxunimplemented!();
    }

    fn allocate(&mut self, obj: &Arc<RwLock<Object>>) {
        self.info.allocate(obj);
        vxunimplemented!();
    }

    fn clean(&mut self) {
        self.info.clean();
        vxunimplemented!();
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub enum Location {
    CPU,
    GPU,
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct RootBuffer {
    pub logical_device: Arc<LogicalDevice>,
    pub memory: Arc<RwLock<Memory>>,
    pub vk_data: vk::VkBuffer,
    pub container: alc::Container,
}

impl RootBuffer {
    pub fn new(size: isize, location: Location, memmgr: &Arc<RwLock<MemoryManager>>) -> Self {
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
        let mut buffer_info = vk::VkBufferCreateInfo::default();
        let logical_device = vxresult!(memmgr.read()).logical_device.clone();
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
            let memroot = vxresult!(mem.root_memory.read());
            vulkan_check!(vk::vkBindBufferMemory(
                logical_device.vk_data,
                vk_data,
                memroot.vk_data,
                mem.info.offset as vk::VkDeviceSize,
            ));
        }
        let container = alc::Container::new(size, 2);
        RootBuffer {
            logical_device,
            memory,
            vk_data,
            container,
        }
    }

    pub fn allocate(&mut self, size: isize) -> Arc<RwLock<Buffer>> {
        let memoff = vxresult!(self.memory.read()).info.offset;
        let buffer = Arc::new(RwLock::new(Buffer::new(size, memoff, self.vk_data)));
        let obj: Arc<RwLock<Object>> = buffer.clone();
        self.container.allocate(&obj);
        return buffer;
    }
}

impl Drop for RootBuffer {
    fn drop(&mut self) {
        unsafe {
            vk::vkDestroyBuffer(self.logical_device.vk_data, self.vk_data, null());
        }
    }
}

#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct StaticBuffer {
    pub buffer: Arc<RwLock<Buffer>>,
}

impl StaticBuffer {
    pub fn new(buffer: Arc<RwLock<Buffer>>) -> Self {
        StaticBuffer { buffer }
    }
}

#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct DynamicBuffer {
    pub buffers: Vec<(Arc<RwLock<Buffer>>, isize)>,
    pub frame_number: Arc<RwLock<u32>>,
    pub actual_size: isize,
}

impl DynamicBuffer {
    pub fn new(
        buffers: Vec<(Arc<RwLock<Buffer>>, isize)>,
        frame_number: Arc<RwLock<u32>>,
        actual_size: isize,
    ) -> Self {
        DynamicBuffer {
            buffers,
            frame_number,
            actual_size,
        }
    }

    pub fn update_with_ptr(&mut self, data: *const c_void) {
        let ptr = self.buffers[*vxresult!(self.frame_number.read()) as usize].1;
        unsafe {
            libc::memcpy(transmute(ptr), transmute(data), self.actual_size as usize);
        }
    }

    pub fn update<T>(&mut self, data: &T)
    where
        T: Sized,
    {
        #[cfg(debug_assertions)]
        {
            if size_of::<T>() != self.actual_size as usize {
                vxlogf!("Data must have same size of buffer.");
            }
        }
        self.update_with_ptr(unsafe { transmute(data) });
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Manager {
    pub alignment: isize,
    pub cpu_buffer: RootBuffer,
    pub gpu_buffer: RootBuffer,
    pub cpu_memory_mapped_ptr: isize,
    pub static_buffer: Arc<RwLock<Buffer>>,
    pub static_uploader_buffer: Arc<RwLock<Buffer>>,
    pub dynamic_buffers: Vec<Arc<RwLock<Buffer>>>,
    pub copy_buffers: Vec<Arc<RwLock<Buffer>>>,
    pub copy_ranges: Vec<vk::VkBufferCopy>,
    pub copy_to_image_ranges: Vec<(vk::VkBufferImageCopy, Arc<RwLock<Image>>)>,
    pub frame_number: Arc<RwLock<u32>>,
    pub cmd_pool: Arc<CmdPool>,
}

impl Manager {
    pub fn new(
        memmgr: &Arc<RwLock<MemoryManager>>,
        cmd_pool: &Arc<CmdPool>,
        frame_number: &Arc<RwLock<u32>>,
        static_size: isize,
        static_uploader_size: isize,
        dynamics_size: isize,
        frames_count: isize,
    ) -> Self {
        let alignment = vxresult!(memmgr.read())
            .logical_device
            .physical_device
            .get_max_min_alignment() as isize;
        let static_size = alc::align(static_size, alignment);
        let static_uploader_size = alc::align(static_uploader_size, alignment);
        let dynamics_size = alc::align(dynamics_size, alignment);
        let frames_dynamics_size = dynamics_size * frames_count;
        let mut cpu_buffer = RootBuffer::new(
            frames_dynamics_size + static_uploader_size,
            Location::CPU,
            memmgr,
        );
        let mut gpu_buffer = RootBuffer::new(static_size, Location::GPU, memmgr);
        let static_buffer = gpu_buffer.allocate(static_size);
        let static_uploader_buffer = cpu_buffer.allocate(static_uploader_size);
        let vk_memory = vxresult!(cpu_buffer.memory.read()).vk_data;
        let vk_device = vxresult!(memmgr.read()).logical_device.vk_data;
        let memory_size = {
            let cpu_memory = vxresult!(cpu_buffer.memory.read());
            let size = vxresult!(cpu_memory.root_memory.read()).container.base.size;
            size
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
        let mut dynamic_buffers = Vec::new();
        for _ in 0..frames_count {
            dynamic_buffers.push(cpu_buffer.allocate(dynamics_size));
        }
        dynamic_buffers.shrink_to_fit();
        let copy_buffers = Vec::new();
        let copy_ranges = Vec::new();
        let copy_to_image_ranges = Vec::new();
        let cmd_pool = cmd_pool.clone();
        let frame_number = frame_number.clone();
        Manager {
            alignment,
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
            frame_number,
        }
    }

    pub fn create_static_buffer_with_ptr(
        &mut self,
        data: *const c_void,
        data_len: usize,
    ) -> StaticBuffer {
        let size = alc::align(data_len as isize, self.alignment);
        let buffer = vxresult!(self.static_buffer.write()).allocate(size);
        let upbuff = self.create_staging_buffer_with_ptr(data, data_len as usize);
        let upbuffer = vxresult!(upbuff.read());
        let mut range = vk::VkBufferCopy::default();
        range.srcOffset = upbuffer.info.base.offset as vk::VkDeviceSize;
        range.dstOffset = vxresult!(buffer.read()).info.base.offset as vk::VkDeviceSize;
        range.size = data_len as vk::VkDeviceSize;
        self.copy_ranges.push(range);
        StaticBuffer::new(buffer)
    }

    pub fn create_static_buffer_with_vec<T>(&mut self, data: &[T]) -> StaticBuffer {
        let data_ptr = unsafe { transmute(data.as_ptr()) };
        let data_len = data.len() * size_of::<T>();
        self.create_static_buffer_with_ptr(data_ptr, data_len)
    }

    pub fn create_staging_buffer_with_ptr(
        &mut self,
        data: *const c_void,
        data_len: usize,
    ) -> Arc<RwLock<Buffer>> {
        let size = alc::align(data_len as isize, self.alignment);
        let upbuffer = vxresult!(self.static_uploader_buffer.write()).allocate(size);
        let off = {
            let upbuff = vxresult!(upbuffer.read());
            let mut off = upbuff.memory_offset;
            off += upbuff.info.base.offset;
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

    pub fn create_staging_buffer_with_vec<T>(&mut self, data: &[T]) -> Arc<RwLock<Buffer>> {
        let data_ptr = unsafe { transmute(data.as_ptr()) };
        let data_len = data.len() * size_of::<T>();
        self.create_staging_buffer_with_ptr(data_ptr, data_len)
    }

    pub fn create_staging_image(
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
        copy_info.bufferOffset = upbuffer.info.base.offset as vk::VkDeviceSize;
        self.copy_to_image_ranges.push((copy_info, image.clone()));
    }

    pub fn create_dynamic_buffer(&mut self, actual_size: isize) -> DynamicBuffer {
        let size = alc::align(actual_size, self.alignment);
        let mut buffers = Vec::new();
        for dynamic_buffer in &self.dynamic_buffers {
            let buffer = vxresult!(dynamic_buffer.write()).allocate(size);
            let ptr = {
                let buffer = vxresult!(buffer.read());
                buffer.memory_offset + buffer.info.base.offset + self.cpu_memory_mapped_ptr
            };
            buffers.push((buffer, ptr));
        }
        buffers.shrink_to_fit();
        DynamicBuffer::new(buffers, self.frame_number.clone(), actual_size)
    }

    pub fn update(&mut self) {
        if self.copy_buffers.len() == 0 {
            return;
        }
        let mut cmd = CmdBuffer::new(self.cmd_pool.clone());
        cmd.begin();
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
                image.change_layout(
                    &mut cmd,
                    vk::VkImageLayout::VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
                );
            }
            for copy_img in &self.copy_to_image_ranges {
                let image = vxresult!(copy_img.1.read());
                cmd.copy_buffer_to_image(self.cpu_buffer.vk_data, image.vk_data, &copy_img.0);
            }
            for copy_img in &self.copy_to_image_ranges {
                let mut image = vxresult!(copy_img.1.write());
                image.change_layout(
                    &mut cmd,
                    vk::VkImageLayout::VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL,
                );
            }
        }
        cmd.flush();
        self.copy_buffers.clear();
        self.copy_to_image_ranges.clear();
        // todo I have to clean the static uploader container in here
    }
}
