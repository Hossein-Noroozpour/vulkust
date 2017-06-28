use super::super::super::system::vulkan as vk;
use super::super::device::logical::Logical as LogicalDevice;
use std::default::Default;
use std::sync::Arc;
use std::ptr::{copy, null};
use std::os::raw::c_void;
use std::mem::transmute;

pub struct Uniform {
    pub logical_device: Arc<LogicalDevice>,
    pub buffer: vk::VkBuffer,
    pub memory: vk::VkDeviceMemory,
    pub descriptor: vk::VkDescriptorBufferInfo,
    pub buffer_size: vk::VkDeviceSize,
}

impl Uniform {
    pub fn new(logical_device: Arc<LogicalDevice>, buffer_size: u32) -> Self {
        let mut buffer = 0 as vk::VkBuffer;
        let mut memory = 0 as vk::VkDeviceMemory;
        let mut mem_reqs = vk::VkMemoryRequirements::default();
        let mut buffer_info = vk::VkBufferCreateInfo::default();
        let mut alloc_info = vk::VkMemoryAllocateInfo::default();
        let mut descriptor = vk::VkDescriptorBufferInfo::default();
        let buffer_size = buffer_size as vk::VkDeviceSize;
        alloc_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
        buffer_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO;
        buffer_info.size = buffer_size;
        buffer_info.usage = vk::VkBufferUsageFlagBits::VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT as u32;
        vulkan_check!(vk::vkCreateBuffer(
            logical_device.vk_data,
            &buffer_info,
            null(),
            &mut buffer,
        ));
        // Get memory requirements including size, alignment and memory type
        unsafe {
            vk::vkGetBufferMemoryRequirements(logical_device.vk_data, buffer, &mut mem_reqs);
        }
        alloc_info.allocationSize = mem_reqs.size;
        // TODO:take care of this comment
        // Get the memory type index that supports host visibile memory access
        // Most implementations offer multiple memory types and selecting the correct one to
        // allocate memory from is crucial We also want the buffer to be host coherent so we don't
        // have to flush (or sync after every update.
        // Note: This may affect performance so you might not want to do this in a real world
        // application that updates buffers on a regular base
        alloc_info.memoryTypeIndex = logical_device.physical_device.get_memory_type_index(
            mem_reqs.memoryTypeBits,
            vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT as u32 |
                vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT as
                    u32,
        );
        vulkan_check!(vk::vkAllocateMemory(
            logical_device.vk_data,
            &alloc_info,
            null(),
            &mut memory,
        ));
        vulkan_check!(vk::vkBindBufferMemory(
            logical_device.vk_data,
            buffer,
            memory,
            0,
        ));
        descriptor.buffer = buffer;
        descriptor.range = buffer_size;
        Uniform {
            logical_device: logical_device,
            buffer: buffer,
            memory: memory,
            descriptor: descriptor,
            buffer_size: buffer_size,
        }
    }
    pub fn update(&self, buffer_data: *const u8) {
        let mut data = 0 as *mut c_void;
        vulkan_check!(vk::vkMapMemory(
            self.logical_device.vk_data,
            self.memory,
            0,
            self.buffer_size,
            0,
            &mut data,
        ));
        unsafe {
            copy(buffer_data, transmute(data), self.buffer_size as usize);
            // Unmap after data has been copied
            // Note: Since we requested a host coherent memory type for the uniform buffer,
            // the write is instantly visible to the GPU
            vk::vkUnmapMemory(self.logical_device.vk_data, self.memory);
        }
    }
}

impl Drop for Uniform {
    fn drop(&mut self) {
        unsafe {
            vk::vkDestroyBuffer(self.logical_device.vk_data, self.buffer, null());
            vk::vkFreeMemory(self.logical_device.vk_data, self.memory, null());
        }
    }
}
