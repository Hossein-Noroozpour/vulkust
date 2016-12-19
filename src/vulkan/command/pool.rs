use super::super::super::system::vulkan::{
    VkResult,
    VkCommandPool,
    VkStructureType,
    vkCreateCommandPool,
    vkDestroyCommandPool,
    VkAllocationCallbacks,
    VkCommandPoolCreateInfo,
    VkCommandPoolCreateFlagBits,
};

use super::super::device::Device;

use std::default::Default;
use std::sync::{
    Arc,
    RwLock,
};

pub struct Pool {
    pub device: Arc<RwLock<Device>>,
    pub vk_pool: VkCommandPool,
}

impl Pool {
    pub fn new(device: Arc<RwLock<Device>>, queue_family_index: u32) -> Self {
        let vk_cmd_pool_info = VkCommandPoolCreateInfo::default();
        vk_cmd_pool_info.sType = VkStructureType::VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO;
        vk_cmd_pool_info.queueFamilyIndex = queue_family_index;
        vk_cmd_pool_info.flags = VkCommandPoolCreateFlagBits::VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT as u32;
        let mut vk_cmd_pool = 0 as VkCommandPool;
        {
            let dev = device.read().unwrap();
            vulkan_check!(vkCreateCommandPool(
                dev.vk_device, &vk_cmd_pool_info, 0 as *const VkAllocationCallbacks,
                &mut vk_cmd_pool));
        }
        Pool {
            device: device,
            vk_pool: vk_cmd_pool,
        }
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        let device = self.device.read().unwrap();
        unsafe {
            vkDestroyCommandPool(device.vk_device, self.vk_pool, 0 as *const VkAllocationCallbacks);
        }
    }
}