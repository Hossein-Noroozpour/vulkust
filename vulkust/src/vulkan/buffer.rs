use super::super::core::allocate as alc;
use super::super::core::allocate::{Allocator, Object};
use super::command::{Buffer as CmdBuffer, Pool as CmdPool};
use super::device::Logical as LogicalDevice;
use super::image::Image;
use super::memory::{Location as MemoryLocation, Manager as MemoryManager, Memory};
use ash::version::DeviceV1_0;
use ash::vk;
use libc;
use std::mem::{size_of, transmute};
use std::os::raw::c_void;
use std::sync::{Arc, RwLock};

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct Buffer {
    memory_offset: isize,
    info: alc::Container,
    vk_data: vk::Buffer,
}

impl Buffer {
    pub fn new(size: isize, memory_offset: isize, vk_data: vk::Buffer, alignment: isize) -> Self {
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

    pub(super) fn get_data(&self) -> vk::Buffer {
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
    vk_data: vk::Buffer,
    container: alc::Container,
    location: Location,
}

impl RootBuffer {
    pub(crate) fn new(
        size: isize,
        location: Location,
        memmgr: &Arc<RwLock<MemoryManager>>,
    ) -> Self {
        let (memloc, usage) = match location {
            Location::CPU => (
                MemoryLocation::CPU,
                vk::BufferUsageFlags::TRANSFER_SRC
                    | vk::BufferUsageFlags::VERTEX_BUFFER
                    | vk::BufferUsageFlags::INDEX_BUFFER
                    | vk::BufferUsageFlags::UNIFORM_BUFFER,
            ),
            Location::GPU => (
                MemoryLocation::GPU,
                vk::BufferUsageFlags::VERTEX_BUFFER
                    | vk::BufferUsageFlags::INDEX_BUFFER
                    | vk::BufferUsageFlags::UNIFORM_BUFFER
                    | vk::BufferUsageFlags::TRANSFER_DST,
            ),
        };
        let logical_device = vxresult!(memmgr.read()).get_device().clone();
        let mut buffer_info = vk::BufferCreateInfo::default();
        buffer_info.size = size as vk::DeviceSize;
        buffer_info.usage = usage;
        let vk_data =
            vxresult!(unsafe { logical_device.get_data().create_buffer(&buffer_info, None,) });
        let mem_reqs = unsafe {
            logical_device
                .get_data()
                .get_buffer_memory_requirements(vk_data)
        };
        let memory = vxresult!(memmgr.write()).allocate(&mem_reqs, memloc);
        {
            let mem = vxresult!(memory.read());
            vxresult!(unsafe {
                logical_device.get_data().bind_buffer_memory(
                    vk_data,
                    mem.get_data(),
                    mem.get_allocated_memory().get_offset() as vk::DeviceSize,
                )
            });
        }
        let container = alc::Container::new(size, logical_device.get_uniform_buffer_alignment());
        Self {
            logical_device,
            memory,
            vk_data,
            container,
            location,
        }
    }

    pub(crate) fn allocate(&mut self, size: isize) -> Arc<RwLock<Buffer>> {
        let memoff = vxresult!(self.memory.read())
            .get_allocated_memory()
            .get_offset();
        let buffer = Arc::new(RwLock::new(Buffer::new(
            size,
            memoff,
            self.vk_data,
            match self.location {
                Location::CPU => self.logical_device.get_uniform_buffer_alignment(),
                _ => 1,
            },
        )));
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
            self.logical_device
                .get_data()
                .destroy_buffer(self.vk_data, None);
        }
    }
}

unsafe impl Send for RootBuffer {}

unsafe impl Sync for RootBuffer {}

#[derive(Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct Static {
    buffer: Arc<RwLock<Buffer>>,
}

impl Static {
    pub(crate) fn new(buffer: Arc<RwLock<Buffer>>) -> Self {
        Self { buffer }
    }

    pub(crate) fn get_buffer(&self) -> &Arc<RwLock<Buffer>> {
        return &self.buffer;
    }
}

#[derive(Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct Dynamic {
    buffers: Vec<(Arc<RwLock<Buffer>>, isize)>,
    actual_size: isize,
}

impl Dynamic {
    pub(crate) fn new(buffers: Vec<(Arc<RwLock<Buffer>>, isize)>, actual_size: isize) -> Self {
        Self {
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
    copy_ranges: Vec<vk::BufferCopy>,
    copy_buffers: Vec<Arc<RwLock<Buffer>>>,
    copy_to_image_ranges: Vec<(vk::BufferImageCopy, Arc<RwLock<Image>>)>,
    frame_copy_buffers: Vec<Vec<Arc<RwLock<Buffer>>>>,
    frame_copy_to_image_ranges: Vec<Vec<(vk::BufferImageCopy, Arc<RwLock<Image>>)>>,
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
        let memmgr = vxresult!(memmgr.read());
        let vk_device = memmgr.get_device().get_data();
        let (memory_size, vk_memory) = {
            let cpu_memory = vxresult!(cpu_buffer.memory.read());
            let vk_memory = cpu_memory.get_data();
            let size = vxresult!(cpu_memory.get_root().read()).get_size();
            (size, vk_memory)
        };
        let cpu_memory_mapped_ptr = unsafe {
            transmute(vxresult!(vk_device.map_memory(
                vk_memory,
                0 as vk::DeviceSize,
                memory_size as vk::DeviceSize,
                vk::MemoryMapFlags::empty()
            )))
        };
        let mut dynamic_buffers = Vec::with_capacity(frames_count as usize);
        for _ in 0..frames_count {
            dynamic_buffers.push(cpu_buffer.allocate(dynamics_size));
        }
        let copy_buffers = Vec::new();
        let copy_ranges = Vec::new();
        let copy_to_image_ranges = Vec::new();
        let mut frame_copy_buffers = Vec::with_capacity(frames_count as usize);
        let mut frame_copy_to_image_ranges = Vec::with_capacity(frames_count as usize);
        for _ in 0..frames_count {
            frame_copy_buffers.push(Vec::new());
            frame_copy_to_image_ranges.push(Vec::new());
        }
        let cmd_pool = cmd_pool.clone();
        Self {
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
    ) -> Static {
        let buffer = vxresult!(self.static_buffer.write()).allocate(data_len as isize);
        let upbuff = self.create_staging_buffer_with_ptr(data, data_len as usize);
        let upbuffer = vxresult!(upbuff.read());
        let mut range = vk::BufferCopy::default();
        range.src_offset = upbuffer.get_allocated_memory().get_offset() as vk::DeviceSize;
        range.dst_offset =
            vxresult!(buffer.read()).get_allocated_memory().get_offset() as vk::DeviceSize;
        range.size = data_len as vk::DeviceSize;
        self.copy_ranges.push(range);
        Static::new(buffer)
    }

    pub(crate) fn create_static_buffer_with_vec<T>(&mut self, data: &[T]) -> Static {
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
        img_info: &vk::ImageCreateInfo,
        base_array_layer: u32,
    ) {
        let upbuff = self.create_staging_buffer_with_vec(pixels);
        let upbuffer = vxresult!(upbuff.read());
        let mut copy_info = vk::BufferImageCopy::default();
        copy_info.image_subresource.aspect_mask = vk::ImageAspectFlags::COLOR;
        copy_info.image_subresource.mip_level = 0;
        copy_info.image_subresource.base_array_layer = base_array_layer;
        copy_info.image_subresource.layer_count = 1;
        copy_info.image_extent.width = img_info.extent.width;
        copy_info.image_extent.height = img_info.extent.height;
        copy_info.image_extent.depth = img_info.extent.depth;
        copy_info.buffer_offset = upbuffer.get_allocated_memory().get_offset() as vk::DeviceSize;
        self.copy_to_image_ranges.push((copy_info, image.clone()));
    }

    pub(crate) fn create_dynamic_buffer(&mut self, actual_size: isize) -> Dynamic {
        let mut buffers = Vec::with_capacity(self.dynamic_buffers.len());
        for dynamic_buffer in &self.dynamic_buffers {
            let buffer = vxresult!(dynamic_buffer.write()).allocate(actual_size);
            let ptr = {
                let buffer = vxresult!(buffer.read());
                buffer.memory_offset
                    + buffer.get_allocated_memory().get_offset()
                    + self.cpu_memory_mapped_ptr
            };
            buffers.push((buffer, ptr));
        }
        Dynamic::new(buffers, actual_size)
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
                image.set_layout(cmd, vk::ImageLayout::TRANSFER_DST_OPTIMAL);
            }
            for copy_img in &self.copy_to_image_ranges {
                let image = vxresult!(copy_img.1.read());
                cmd.copy_buffer_to_image(self.cpu_buffer.vk_data, image.get_data(), &copy_img.0);
            }
            for copy_img in &self.copy_to_image_ranges {
                let mut image = vxresult!(copy_img.1.write());
                if image.get_mips_count() == 1 {
                    image.set_layout(cmd, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
                }
            }
        }
        self.frame_copy_buffers[frame_number].append(&mut self.copy_buffers);
        self.frame_copy_to_image_ranges[frame_number].append(&mut self.copy_to_image_ranges);
    }

    pub(super) fn secondary_update(&self, cmd: &mut CmdBuffer, frame_number: usize) {
        for ri in &self.frame_copy_to_image_ranges[frame_number] {
            let mut img = vxresult!(ri.1.write());
            if img.get_mips_count() < 2 {
                continue;
            }
            img.generate_mips(cmd);
        }
    }

    pub(super) fn get_gpu_root_buffer(&self) -> &RootBuffer {
        return &self.gpu_buffer;
    }
}

unsafe impl Send for Manager {}

unsafe impl Sync for Manager {}
