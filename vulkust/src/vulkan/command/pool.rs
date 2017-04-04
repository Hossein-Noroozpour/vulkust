use super::super::super::system::vulkan as vk;
use super::super::device::logical::Logical as LogicalDevice;
use std::default::Default;
use std::sync::{
    Arc,
    RwLock,
};

pub struct Pool {
    pub logical_device: Arc<LogicalDevice>,
    pub vk_data: vk::VkCommandPool,
}

impl Pool {
    pub fn new(logical_device: Arc<LogicalDevice>, queue_family_index: u32) -> Self {
        let vk_cmd_pool_info = vk::VkCommandPoolCreateInfo::default();
        vk_cmd_pool_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO;
        vk_cmd_pool_info.queueFamilyIndex = queue_family_index;
        vk_cmd_pool_info.flags =
            vk::VkCommandPoolCreateFlagBits::VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT as u32;
        let mut vk_cmd_pool = 0 as VkCommandPool;
        vulkan_check!(vkCreateCommandPool(
            device.vk_device, &vk_cmd_pool_info, 0 as *const VkAllocationCallbacks, &mut vk_cmd_pool));
        Pool {
            device: device,
            vk_pool: vk_cmd_pool,
        }
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        unsafe {
            vkDestroyCommandPool(self.device.vk_device, self.vk_pool, 0 as *const VkAllocationCallbacks);
        }
    }
}
