use libc;
use super::super::core::allocate as alc;
use super::super::core::allocate::{Allocator, Object};
use super::command::pool::Pool as CmdPool;
use super::command::buffer::Buffer as CmdBuffer;
use super::memory::{
    Manager as MemoryManager, 
    Location as MemoryLocation, 
    Memory
};
use super::vulkan as vk;
use std::ptr::null;
use std::sync::{Arc, RwLock};
use std::os::raw::c_void;
use std::mem::transmute;

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

pub struct StaticBuffer {
    pub buffer: Arc<RwLock<Buffer>>,
}

impl StaticBuffer {
    pub fn new(buffer: Arc<RwLock<Buffer>>) -> Self {
        StaticBuffer {
            buffer,
        }
    }
}

pub struct Manager {
    pub alignment: isize,
    pub cpu_buffer: RootBuffer,
    pub gpu_buffer: RootBuffer,
    pub cpu_memory_mapped_ptr: isize,
    pub static_buffer: Arc<RwLock<Buffer>>,
    pub static_uploader_buffer: Arc<RwLock<Buffer>>,
    pub dynamic_buffers: Vec<Arc<RwLock<Buffer>>>,
    pub copy_ranges: Vec<vk::VkBufferCopy>,
    pub cmd_pool: Arc<CmdPool>,
}

impl Manager {
    pub fn new(
        memmgr: &Arc<RwLock<MemoryManager>>,
        cmd_pool: &Arc<CmdPool>,
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
        let vk_memory = vxresult!(cpu_buffer.memory.read()).vk_data;
        let vk_device = vxresult!(memmgr.read()).logical_device.vk_data;
        let memory_size = {
            let cpu_memory = vxresult!(cpu_buffer.memory.read());
            let size = vxresult!(cpu_memory.root_memory.read()).container.size;
            size
        };
        let cpu_memory_mapped_ptr = unsafe {
            let mut data_ptr = 0 as *mut c_void;
            vulkan_check!(vk::vkMapMemory(
                vk_device,
                vk_memory,
                0, memory_size as u64,
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
        let copy_ranges = Vec::new();
        let cmd_pool = cmd_pool.clone();
        Manager {
            alignment,
            cpu_buffer,
            gpu_buffer,
            cpu_memory_mapped_ptr,
            static_buffer,
            static_uploader_buffer,
            dynamic_buffers,
            copy_ranges,
            cmd_pool,
        }
    }

    pub fn create_static_buffer(&mut self, actual_size: isize, data: *const c_void) -> StaticBuffer {
        let size = alc::align(actual_size, self.alignment);
        let buffer = vxresult!(self.static_buffer.write()).allocate(size);
        let upbuffer = vxresult!(self.static_uploader_buffer.write()).allocate(size);
        let upbuffer = vxresult!(upbuffer.read());
        let mut off = upbuffer.memory_offset;
        off += upbuffer.info.offset;
        off += self.cpu_memory_mapped_ptr;
        unsafe {
            let ptr = transmute(off);
            libc::memcpy(ptr, transmute(data), actual_size as usize);
        }
        let mut range = vk::VkBufferCopy::default();
        range.srcOffset = upbuffer.info.offset as vk::VkDeviceSize;
        range.dstOffset = vxresult!(buffer.read()).info.offset as vk::VkDeviceSize;
        range.size = size as vk::VkDeviceSize;
        self.copy_ranges.push(range);
        StaticBuffer::new(buffer)
    }

    pub fn update(&mut self) {
        if self.copy_ranges.len() == 0 {
            return;
        }
        let mut cmd = CmdBuffer::new(self.cmd_pool.clone());
        cmd.copy_buffer(
            self.cpu_buffer.vk_data,
            self.gpu_buffer.vk_data,
            &self.copy_ranges,
        );
        self.copy_ranges.clear();
        cmd.flush();
    }
}