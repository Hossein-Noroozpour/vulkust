extern crate libc;

pub mod uniform;

use std::default::Default;
use std::sync::Arc;
use std::ptr::{null, null_mut};
use std::os::raw::c_void;
use std::mem::transmute;
use super::super::system::vulkan as vk;
use super::command::pool::Pool as CmdPool;
use super::command::buffer::Buffer as CmdBuff;

pub enum Usage {
    Vertex,
    Index,
}

pub struct Buffer {
    pub cmd_pool: Arc<CmdPool>,
    pub vk_data: vk::VkBuffer,
    pub memory: vk::VkDeviceMemory,
    pub offset: u64,
    pub size: u64,
}

impl Buffer {
    fn create(&mut self, size: u64, usage: u32) {
        self.size = size;
        let mut buffer_info = vk::VkBufferCreateInfo::default();
        buffer_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO;
        buffer_info.size = size as vk::VkDeviceSize;
        buffer_info.usage =
            usage | vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_DST_BIT as u32;
        vulkan_check!(vk::vkCreateBuffer(
            self.cmd_pool.logical_device.vk_data,
            &buffer_info,
            null(),
            &mut self.vk_data,
        ));
        let mut mem_reqs = vk::VkMemoryRequirements::default();
        unsafe {
            vk::vkGetBufferMemoryRequirements(
                self.cmd_pool.logical_device.vk_data,
                self.vk_data,
                &mut mem_reqs,
            );
        }
        let mut mem_alloc = vk::VkMemoryAllocateInfo::default();
        mem_alloc.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
        mem_alloc.allocationSize = mem_reqs.size;
        mem_alloc.memoryTypeIndex = self.cmd_pool
            .logical_device
            .physical_device
            .get_memory_type_index(
                mem_reqs.memoryTypeBits,
                vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as u32,
            );
        vulkan_check!(vk::vkAllocateMemory(
            self.cmd_pool.logical_device.vk_data,
            &mem_alloc,
            null(),
            &mut self.memory,
        ));
        vulkan_check!(vk::vkBindBufferMemory(
            self.cmd_pool.logical_device.vk_data,
            self.vk_data,
            self.memory,
            0,
        ));
    }

    pub fn new(cmd_pool: Arc<CmdPool>, size: u64, usage: Usage) -> Self {
        let mut b = Buffer {
            cmd_pool: cmd_pool,
            vk_data: null_mut(),
            memory: null_mut(),
            offset: 0,
            size: size,
        };
        let usage = match usage {
            Usage::Vertex => vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_VERTEX_BUFFER_BIT as u32,
            Usage::Index => vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_INDEX_BUFFER_BIT as u32,
        };
        b.create(size, usage);
        return b;
    }

    pub fn seek(&mut self, offset: u64) {
        self.offset = offset;
    }

    pub fn write(&mut self, data: *const c_void, size: u64) {
        if self.offset + size > self.size {
            logf!(
                "{}{} {}{} {}{}",
                "Your data reached to the maximum size: ", self.size,
                "please specify a better size for buffer current offset is: ", self.offset,
                "data you want to write has size: ", size);
        }
        let mut mem_alloc = vk::VkMemoryAllocateInfo::default();
        mem_alloc.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
        let mut mem_reqs = vk::VkMemoryRequirements::default();
        let mut buffer_data = 0 as *mut c_void;
        let mut staging_memory = 0 as vk::VkDeviceMemory;
        let mut staging_buffer = 0 as vk::VkBuffer;
        let mut buffer_info = vk::VkBufferCreateInfo::default();
        buffer_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO;
        buffer_info.size = size as vk::VkDeviceSize;
        buffer_info.usage = vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_SRC_BIT as u32;
        vulkan_check!(vk::vkCreateBuffer(
            self.cmd_pool.logical_device.vk_data,
            &buffer_info,
            null(),
            &mut staging_buffer,
        ));
        unsafe {
            vk::vkGetBufferMemoryRequirements(
                self.cmd_pool.logical_device.vk_data,
                staging_buffer,
                &mut mem_reqs,
            );
        }
        mem_alloc.allocationSize = mem_reqs.size;
        mem_alloc.memoryTypeIndex = self.cmd_pool
            .logical_device
            .physical_device
            .get_memory_type_index(
                mem_reqs.memoryTypeBits,
                vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT as u32 |
                    vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT as u32,
            );
        vulkan_check!(vk::vkAllocateMemory(
            self.cmd_pool.logical_device.vk_data,
            &mem_alloc,
            null(),
            &mut staging_memory,
        ));
        vulkan_check!(vk::vkMapMemory(
            self.cmd_pool.logical_device.vk_data,
            staging_memory,
            0,
            mem_alloc.allocationSize,
            0,
            &mut buffer_data,
        ));
        unsafe {
            libc::memcpy(
                transmute(buffer_data),
                transmute(data),
                size as libc::size_t,
            );
            vk::vkUnmapMemory(self.cmd_pool.logical_device.vk_data, staging_memory);
        }
        vulkan_check!(vk::vkBindBufferMemory(
            self.cmd_pool.logical_device.vk_data,
            staging_buffer,
            staging_memory,
            0,
        ));
        let copy_cmd = CmdBuff::new(self.cmd_pool.clone());
        let mut copy_region = vk::VkBufferCopy::default();
        copy_region.dstOffset = self.offset as vk::VkDeviceSize;
        copy_region.size = size as vk::VkDeviceSize;
        unsafe {
            vk::vkCmdCopyBuffer(
                copy_cmd.vk_data,
                staging_buffer,
                self.vk_data,
                1,
                &copy_region,
            );
        }
        copy_cmd.flush();
        unsafe {
            vk::vkDestroyBuffer(self.cmd_pool.logical_device.vk_data, staging_buffer, null());
            vk::vkFreeMemory(self.cmd_pool.logical_device.vk_data, staging_memory, null());
        }
        self.offset += size;
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        if self.vk_data == null_mut() {
            return;
        }
        unsafe {
            vk::vkDestroyBuffer(self.cmd_pool.logical_device.vk_data, self.vk_data, null());
            vk::vkFreeMemory(self.cmd_pool.logical_device.vk_data, self.memory, null());
        }
    }
}
