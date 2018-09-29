use super::super::device::logical::Logical as LogicalDevice;
use super::super::vulkan as vk;
use std::default::Default;
use std::ptr::null;
use std::sync::Arc;

#[cfg_attr(debug_mode, derive(Debug))]
pub enum Type {
    Graphic,
    Compute,
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Pool {
    pub pool_type: Type,
    pub logical_device: Arc<LogicalDevice>,
    pub vk_data: vk::VkCommandPool,
}

impl Pool {
    pub fn new(logical_device: &Arc<LogicalDevice>, pool_type: Type, flags: u32) -> Self {
        let mut vk_cmd_pool_info = vk::VkCommandPoolCreateInfo::default();
        vk_cmd_pool_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO;
        match pool_type {
            Type::Graphic => {
                vk_cmd_pool_info.queueFamilyIndex =
                    logical_device.physical_device.graphics_queue_node_index;
            }
            Type::Compute => {
                vk_cmd_pool_info.queueFamilyIndex =
                    logical_device.physical_device.compute_queue_node_index;
            }
        }
        vk_cmd_pool_info.flags =
            vk::VkCommandPoolCreateFlagBits::VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT as u32
                | flags;
        let mut vk_data = 0 as vk::VkCommandPool;
        vulkan_check!(vk::vkCreateCommandPool(
            logical_device.vk_data,
            &vk_cmd_pool_info,
            null(),
            &mut vk_data,
        ));
        Pool {
            pool_type,
            logical_device: logical_device.clone(),
            vk_data: vk_data,
        }
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        unsafe {
            vk::vkDestroyCommandPool(self.logical_device.vk_data, self.vk_data, null());
        }
    }
}
