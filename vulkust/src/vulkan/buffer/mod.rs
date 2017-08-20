extern crate libc;

use std::default::Default;
use std::mem::{transmute, size_of};
use std::os::raw::c_void;
use std::ptr::{null, null_mut};
use std::sync::Arc;
use super::super::system::vulkan as vk;
use super::command::buffer::Buffer as CmdBuff;
use super::device::logical::Logical as LogicalDevice;

struct Region {
    pub logical_device: Arc<LogicalDevice>,
    pub buffer: vk::VkBuffer,
    pub memory: vk::VkDeviceMemory,
    pub alignment: usize,
    pub start: usize,
    pub offset: usize,
    pub size: usize,
}

impl Region {
    pub fn new(logical_device: Arc<LogicalDevice>, size: usize) -> Self {
        let mut buffer_info = vk::VkBufferCreateInfo::default();
        buffer_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO;
        buffer_info.size = size as vk::VkDeviceSize;
        buffer_info.usage = vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_SRC_BIT as u32;
        let mut buffer = 0 as vk::VkBuffer;
        vulkan_check!(vk::vkCreateBuffer(
            logical_device.vk_data,
            &buffer_info,
            null(),
            &mut buffer,
        ));
        let mut mem_reqs = vk::VkMemoryRequirements::default();
        unsafe {
            vk::vkGetBufferMemoryRequirements(logical_device.vk_data, buffer, &mut mem_reqs);
        }
        let mut mem_alloc = vk::VkMemoryAllocateInfo::default();
        mem_alloc.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
        mem_alloc.allocationSize = mem_reqs.size;
        mem_alloc.memoryTypeIndex = logical_device.physical_device.get_memory_type_index(
            mem_reqs.memoryTypeBits,
            vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT as u32 |
                vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT as u32,
        );
        let mut memory = 0 as vk::VkDeviceMemory;
        vulkan_check!(vk::vkAllocateMemory(
            logical_device.vk_data,
            &mem_alloc,
            null(),
            &mut memory,
        ));
        let mut start = 0;
        vulkan_check!(vk::vkMapMemory(
            logical_device.vk_data,
            memory,
            0,
            mem_alloc.allocationSize,
            0,
            transmute(&mut start),
        ));
        vulkan_check!(vk::vkBindBufferMemory(
            logical_device.vk_data,
            buffer,
            memory,
            0,
        ));
        let alignment = logical_device.physical_device.get_max_min_alignment() as usize;
        Region {
            logical_device: logical_device,
            buffer: buffer,
            memory: memory,
            alignment: alignment,
            start: start,
            offset: 0,
            size: size,
        }
    }

    pub fn write(&mut self, data: *const c_void, size: usize) -> (usize, usize) {
        let begin = self.offset;
        if self.offset + size > self.size {
            logf!(
                "{}{} {}{} {}{}",
                "Your data reached to the maximum size: ",
                self.size,
                "please specify a better size for buffer current offset is: ",
                self.offset,
                "data you want to write has size: ",
                size
            );
        }
        unsafe {
            libc::memcpy(
                transmute(self.start + self.offset),
                transmute(data),
                size as libc::size_t,
            );
        }
        self.offset += size;
        let flag = self.alignment - 1;
        let rem = self.offset & flag;
        if rem != 0 {
            self.offset += self.alignment - rem;
        }
        (begin, self.offset)
    }

    pub fn push(&self, cmd: &mut CmdBuff, start: usize, dst: vk::VkBuffer) {
        let mut regions = vec![vk::VkBufferCopy::default(); 1];
        regions[0].dstOffset = start as vk::VkDeviceSize;
        regions[0].size = self.size as vk::VkDeviceSize;
        cmd.copy_buffer(self.buffer, dst, &regions);
    }
}

impl Drop for Region {
    fn drop(&mut self) {
        if self.buffer == null_mut() {
            logf!("Unexpected!");
        }
        unsafe {
            vk::vkDestroyBuffer(self.logical_device.vk_data, self.buffer, null());
            vk::vkFreeMemory(self.logical_device.vk_data, self.memory, null());
        }
    }
}

pub struct Manager {
    vk_data: vk::VkBuffer,
    memory: vk::VkDeviceMemory,
    vertices_indices: Region,
    uniforms: Region,
    uniforms_align: usize,
    frames_count: usize,
}

impl Manager {
    fn create(&mut self) {
        let mut buffer_info = vk::VkBufferCreateInfo::default();
        buffer_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO;
        buffer_info.size = (self.vertices_indices.size + self.uniforms.size) as vk::VkDeviceSize;
        buffer_info.usage = vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_VERTEX_BUFFER_BIT as u32 |
            vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_INDEX_BUFFER_BIT as u32 |
            vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT as u32 |
            vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_DST_BIT as u32;
        vulkan_check!(vk::vkCreateBuffer(
            self.uniforms.logical_device.vk_data,
            &buffer_info,
            null(),
            &mut self.vk_data,
        ));
        let mut mem_reqs = vk::VkMemoryRequirements::default();
        unsafe {
            vk::vkGetBufferMemoryRequirements(
                self.uniforms.logical_device.vk_data,
                self.vk_data,
                &mut mem_reqs,
            );
        }
        let mut mem_alloc = vk::VkMemoryAllocateInfo::default();
        mem_alloc.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
        mem_alloc.allocationSize = mem_reqs.size;
        mem_alloc.memoryTypeIndex = self.uniforms
            .logical_device
            .physical_device
            .get_memory_type_index(
                mem_reqs.memoryTypeBits,
                vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as u32,
            );
        vulkan_check!(vk::vkAllocateMemory(
            self.uniforms.logical_device.vk_data,
            &mem_alloc,
            null(),
            &mut self.memory,
        ));
        vulkan_check!(vk::vkBindBufferMemory(
            self.uniforms.logical_device.vk_data,
            self.vk_data,
            self.memory,
            0,
        ));
    }

    pub fn new(logical_device: Arc<LogicalDevice>, vi_size: usize, u_size: usize, frames_count: usize) -> Self {
        let alignment = logical_device.physical_device.get_max_min_alignment() as usize;
        let vertices = Region::new(logical_device.clone(), vi_size);
        let uniforms = Region::new(logical_device, u_size * frames_count);
        let mut b = Manager {
            vk_data: null_mut(),
            memory: null_mut(),
            vertices_indices: vertices,
            uniforms: uniforms,
            uniforms_align: u_size,
            frames_count: frames_count,
        };
        let flag = alignment - 1;
        if vi_size & flag != 0 || u_size & flag != 0 {
            logf!("Buffer sizes must be coefficeint of {}", alignment);
        }
        b.create();
        return b;
    }

    pub fn seek_vi(&mut self, offset: usize) {
        self.vertices_indices.offset = offset;
    }

    pub fn seek_u(&mut self, offset: usize) {
        self.uniforms.offset = offset;
    }

    pub fn add_vi(&mut self, data: *const c_void, size: usize) -> (usize, usize) {
        self.vertices_indices.write(data, size)
    }

    pub fn add_u<T>(&mut self, data: &T) -> (Vec<&'static mut T>, Vec<(usize, usize)>) {
        let (ptrs, rngs) = self.add_u_with_ptr(unsafe { transmute(data) }, size_of::<T>());
        let mut trefs = vec![unsafe { transmute(ptrs[0]) }; self.frames_count];
        for i in 1..self.frames_count {
            trefs[i] = unsafe { transmute(ptrs[i]) };
        }
        (trefs, rngs)
    }

    fn add_u_with_ptr(&mut self, data: *const c_void, size: usize) -> (Vec<*mut c_void>, Vec<(usize, usize)>) {
        let mut res = vec![(0, 0); self.frames_count];
        let mut offset = self.uniforms.offset;
        res[0] = self.uniforms.write(data, size);
        let last_offset = self.uniforms.offset;
        for i in 1..self.frames_count {
            offset += self.uniforms_align;
            self.seek_u(offset);
            res[i] = self.uniforms.write(data, size);
        }
        self.seek_u(last_offset);
        let uniforms_start = self.vertices_indices.size;
        let map_start = self.uniforms.start;
        let mut ptrs = vec![null_mut(); self.frames_count];
        for i in 0..self.frames_count {
            ptrs[i] = unsafe { transmute (res[i].0 + map_start) };
            res[i].0 += uniforms_start;
            res[i].1 += uniforms_start;
        }
        return (ptrs, res);
    }

    pub fn update_u(&mut self, data: *const c_void, size: usize, offset: usize) {
        self.seek_u(offset);
        let _ = self.uniforms.write(data, size);
    }

    pub fn push_vi(&self, cmd: &mut CmdBuff) {
        self.vertices_indices.push(cmd, 0, self.vk_data);
    }

    pub fn push_u(&self, cmd: &mut CmdBuff, frame_index: usize) {
        self.uniforms.push(
            cmd, 
            self.vertices_indices.size + (self.uniforms_align * frame_index), 
            self.vk_data);
    }

    pub fn get_id(&self) -> u64 {
        self.vk_data as u64
    }

    pub fn get_buffer(&self) -> vk::VkBuffer {
        self.vk_data
    }

    pub fn get_device(&self) -> &Arc<LogicalDevice> {
        &self.uniforms.logical_device
    }
}

impl Drop for Manager {
    fn drop(&mut self) {
        if self.vk_data == null_mut() {
            return;
        }
        unsafe {
            vk::vkDestroyBuffer(self.uniforms.logical_device.vk_data, self.vk_data, null());
            vk::vkFreeMemory(self.uniforms.logical_device.vk_data, self.memory, null());
        }
    }
}
