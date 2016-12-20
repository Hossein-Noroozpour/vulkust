use super::device::Device;

use super::super::system::vulkan::{
    VkBuffer,
    vkMapMemory,
    VkBufferCopy,
    VkDeviceSize,
    vkUnmapMemory,
    vkCreateBuffer,
    VkDeviceMemory,
    vkCmdCopyBuffer,
    VkCommandBuffer,
    VkStructureType,
    vkAllocateMemory,
    vkBindBufferMemory,
    VkBufferUsageFlags,
    VkBufferCreateInfo,
    VkMemoryAllocateInfo,
    VkMemoryRequirements,
    VkBufferUsageFlagBits,
    VkAllocationCallbacks,
    VkCommandBufferBeginInfo,
    VkMemoryPropertyFlagBits,
    vkGetBufferMemoryRequirements,
};

use std::sync::{
    Arc,
};
use std::mem::size_of;
use std::default::Default;

pub struct Buffer {
    device: Arc<Devie>,
    vk_buffer: VkBuffer,
}

impl Buffer {
    fn new_in_vram<T>(device: Arc<Device>, data: Vec<T>) -> Self {
        let mut vk_staging_buffer = 0 as VkBuffer;
        let mut mem_alloc = VkMemoryAllocateInfo::default();
        let mut mem_rs = VkMemoryRequirements::default();
        let mut vertex_buffer_info = VkBufferCreateInfo::default();
        vertex_buffer_info.sType = VkStructureType::VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO;
        vertex_buffer_info.size = size_of::<T>() as u32 * data.len() as u32;
        vertex_buffer_info.usage = VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_SRC_BIT;
        vulkan_check!(vkCreateBuffer(device.vk_device, &vertex_buffer_info as *const VkBufferCreateInfo, 0 as *const VkAllocationCallbacks, &mut vk_staging_buffer as *mut VkBuffer));
        vulkan_check!(vkGetBufferMemoryRequirements(device.vk_device, vk_staging_buffer, &mem_rs as *mut VkMemoryRequirements));
        mem_alloc.sType = VkStructureType::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
        mem_alloc.allocationSize = mem_rs.size;
        mem_alloc.memoryTypeIndex = device.get_memory_type_index(mem_rs.memoryTypeBits, VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT);
        vulkan_check!(vkAllocateMemory(dev.vk_device, &mem_alloc, 0, &staging_buffers[0].memory));
        vulkan_check!(vkMapMemory(dev.vk_device, staging_buffers[0].memory, 0, mem_alloc.allocationSize, 0, &data));
        memcpy(data, vertex_buffer.as_ptr(), vertex_buffer_size);
        vulkan_check!(vkUnmapMemory(dev.vk_device, staging_buffers[0].memory));
        vulkan_check!(vkBindBufferMemory(dev.vk_device, staging_buffers[0].buffer, staging_buffers[0].memory, 0));

        vertex_buffer_info.usage = VkBufferUsageFlagBits::VK_BUFFER_USAGE_VERTEX_BUFFER_BIT | VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_DST_BIT;
        vulkan_check!(vkCreateBuffer(dev.vk_device, &vertex_buffer_info, 0, &vertices.buffer));
        vkGetBufferMemoryRequirements(device, vertices.buffer, &memReqs);
        memAlloc.allocationSize = memReqs.size;
        memAlloc.memoryTypeIndex = dev.get_memory_type_index(mem_rs.memoryTypeBits, VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT);
        vulkan_check!(vkAllocateMemory(device, &memAlloc, nullptr, &vertices.memory));
        vulkan_check!(vkBindBufferMemory(device, vertices.buffer, vertices.memory, 0));
        Buffer {
            device: device,
            vk_buffer: vk_staging_buffer,
        }
    }
}