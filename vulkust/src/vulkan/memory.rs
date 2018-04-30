use super::device::logical::Logical as LogicalDevice;
use super::vulkan as vk;

use std::ptr::null;
use std::sync::Arc;

pub fn allocate_with_requirements(
    logical_device: &Arc<LogicalDevice>,
    mem_req_s: vk::VkMemoryRequirements,
) -> vk::VkDeviceMemory {
    let mut mem_alloc = vk::VkMemoryAllocateInfo::default();
    mem_alloc.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
    mem_alloc.allocationSize = mem_req_s.size;
    let ref memory_prop = logical_device.physical_device.memory_properties;
    let mut type_bits = mem_req_s.memoryTypeBits;
    let mut memory_type_not_found = true;
    for index in 0..memory_prop.memoryTypeCount {
        if (type_bits & 1) == 1
            && ((memory_prop.memoryTypes[index as usize].propertyFlags as u32)
                & (vk::VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as u32))
                != 0
        {
            mem_alloc.memoryTypeIndex = index;
            memory_type_not_found = false;
            break;
        }
        type_bits >>= 1;
    }
    if memory_type_not_found {
        vxlogf!("Error memory type not found.");
    }
    let mut memory = 0 as vk::VkDeviceMemory;
    vulkan_check!(vk::vkAllocateMemory(
        logical_device.vk_data,
        &mem_alloc,
        null(),
        &mut memory,
    ));
    memory
}

pub struct Manager {}
