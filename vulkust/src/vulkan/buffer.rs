use super::super::core::allocate as alc;
use super::super::core::allocate::{Allocator, Object};
use super::memory::{
    Manager as MemoryManager, 
    Location as MemoryLocation, 
    Memory
};
use super::vulkan as vk;
use std::ptr::null;
use std::sync::{Arc, RwLock};

pub struct Buffer {
    pub memory_offset: isize,
    pub info: alc::Container,
}

impl Buffer {
    pub fn new(size: isize, memory_offset: isize) -> Self {
        let info = alc::Container::new(size);
        Buffer {
            memory_offset,
            info,
        }
    }

    pub fn allocate(&mut self, size: isize) -> Arc<RwLock<Buffer>> {
        let buffer = Arc::new(RwLock::new(Buffer::new(size, self.memory_offset)));
        let obj: Arc<RwLock<Object>> = buffer.clone();
        self.info.allocate(&obj);
        return buffer;
    }
}

impl Object for Buffer {
    fn size(&self) -> isize {
        self.info.size
    }

    fn offset(&self) -> isize {
        self.info.offset
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

pub enum Location {
    CPU,
    GPU,
}

pub struct RootBuffer {
    pub memory: Arc<RwLock<Memory>>,
    pub vk_data: vk::VkBuffer,
    pub container: alc::Container,
}

impl RootBuffer {
    pub fn new(size: isize, location: Location, memmgr: &Arc<RwLock<MemoryManager>>) -> Self {
        let (memloc, usage) = match location {
            Location::CPU => (
                MemoryLocation::CPU, 
                vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_SRC_BIT as u32),
            Location::GPU => (
                MemoryLocation::GPU,
                vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_VERTEX_BUFFER_BIT as u32 |
                vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_INDEX_BUFFER_BIT as u32 |
                vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT as u32 |
                vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_DST_BIT as u32),
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
        let container = alc::Container::new(size);
        RootBuffer {
            memory,
            vk_data,
            container,
        }
    }

    pub fn allocate(&mut self, size: isize) -> Arc<RwLock<Buffer>> {
        let memoff = vxresult!(self.memory.read()).info.offset;
        let buffer = Arc::new(RwLock::new(Buffer::new(size, memoff)));
        let obj: Arc<RwLock<Object>> = buffer.clone();
        self.container.allocate(&obj);
        return buffer;
    }
}

pub struct Manager {
    pub alignment: isize,
    pub cpu_buffer: RootBuffer,
    pub gpu_buffer: RootBuffer,
    pub static_buffer: Arc<RwLock<Buffer>>,
    pub static_uploader_buffer: Arc<RwLock<Buffer>>,
    pub dynamic_buffers: Vec<Arc<RwLock<Buffer>>>,
    pub dynamic_uploader_buffers: Vec<Arc<RwLock<Buffer>>>,
}

impl Manager {
    pub fn new(
        memmgr: &Arc<RwLock<MemoryManager>>,
        static_size: isize,
        static_uploader_size: isize,
        dynamics_size: isize, 
        frames_count: isize) -> Self {
        let alignment = vxresult!(memmgr.read())
            .logical_device.physical_device.get_max_min_alignment() as isize;
        let static_size = alc::align(static_size, alignment);
        let static_uploader_size = alc::align(static_uploader_size, alignment);
        let dynamics_size = alc::align(dynamics_size, alignment);
        let frames_dynamics_size = dynamics_size * frames_count;
        let mut cpu_buffer = RootBuffer::new(
            frames_dynamics_size + static_uploader_size, Location::CPU, memmgr);
        let mut gpu_buffer = RootBuffer::new(
            frames_dynamics_size + static_size, Location::GPU, memmgr);
        let static_buffer = gpu_buffer.allocate(static_size);
        let static_uploader_buffer = cpu_buffer.allocate(static_uploader_size);
        let mut dynamic_buffers = Vec::new();
        let mut dynamic_uploader_buffers = Vec::new();
        for _ in 0..frames_count {
            dynamic_buffers.push(gpu_buffer.allocate(dynamics_size));
            dynamic_uploader_buffers.push(cpu_buffer.allocate(dynamics_size));
        }
        dynamic_buffers.shrink_to_fit();
        dynamic_uploader_buffers.shrink_to_fit();
        Manager {
            alignment,
            cpu_buffer,
            gpu_buffer,
            static_buffer,
            static_uploader_buffer,
            dynamic_buffers,
            dynamic_uploader_buffers,
        }
    }
}