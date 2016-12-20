use std::os::raw::{
    memcpy,
};

use super::device::Device;
use super::super::system::vulkan::{
    VkBuffer,
    VkResult,
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
use std::os::raw::c_void;
use std::mem::size_of;
use std::default::Default;

pub struct Buffer {
    pub device: Arc<Device>,
    pub vk_buffer: VkBuffer,
}

impl Buffer {
    pub fn new_in_vram<T>(device: Arc<Device>, buffer_data: Vec<T>) -> Self {
        let mut vk_staging_buffer = 0 as VkBuffer;
        let mut vk_staging_memory = 0 as VkDeviceMemory;
        let mut mem_alloc = VkMemoryAllocateInfo::default();
        let mut mem_rs = VkMemoryRequirements::default();
        let mut data = 0 as *mut c_void;
        let mut buffer_info = VkBufferCreateInfo::default();
        buffer_info.sType = VkStructureType::VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO;
        buffer_info.size = size_of::<T>() as u64 * buffer_data.len() as u64;
        buffer_info.usage = VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_SRC_BIT as u32;
        vulkan_check!(vkCreateBuffer(
            device.vk_device, &buffer_info as *const VkBufferCreateInfo,
            0 as *const VkAllocationCallbacks, &mut vk_staging_buffer as *mut VkBuffer));
        vkGetBufferMemoryRequirements(
            device.vk_device, vk_staging_buffer, &mut mem_rs as *mut VkMemoryRequirements);
        mem_alloc.sType = VkStructureType::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
        mem_alloc.allocationSize = mem_rs.size;
        mem_alloc.memoryTypeIndex = device.get_memory_type_index(
            mem_rs.memoryTypeBits,
            VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT as u32 |
                VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_HOST_COHERENT_BIT as u32);
        vulkan_check!(vkAllocateMemory(
            device.vk_device, &mem_alloc as *const VkMemoryAllocateInfo,
            0 as *const VkAllocationCallbacks, &mut vk_staging_memory as *mut VkDeviceMemory));
        vulkan_check!(vkMapMemory(
            device.vk_device, vk_staging_memory, 0, mem_alloc.allocationSize, 0,
            &mut data as *mut *mut c_void));
        unsafe { memcpy(data, vertex_buffer.as_ptr(), vertex_buffer_size) };
        vkUnmapMemory(device.vk_device, vk_staging_memory);
        vulkan_check!(vkBindBufferMemory(device.vk_device, staging_buffers[0].buffer, staging_buffers[0].memory, 0));

        buffer_info.usage = VkBufferUsageFlagBits::VK_BUFFER_USAGE_VERTEX_BUFFER_BIT as u32 | VkBufferUsageFlagBits::VK_BUFFER_USAGE_TRANSFER_DST_BIT as u32;
        vulkan_check!(vkCreateBuffer(dev.vk_device, &vertex_buffer_info, 0 as ptr, &vertices.buffer));
        vkGetBufferMemoryRequirements(device.vk_device, vertices.buffer, &memReqs);
        memAlloc.allocationSize = memReqs.size;
        memAlloc.memoryTypeIndex = dev.get_memory_type_index(mem_rs.memoryTypeBits, VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT);
        vulkan_check!(vkAllocateMemory(device.vk_device, &memAlloc, nullptr, &vertices.memory));
        vulkan_check!(vkBindBufferMemory(device.vk_device, vertices.buffer, vertices.memory, 0));
        Buffer {
            device: device,
            vk_buffer: vk_staging_buffer,
        }
    }
}