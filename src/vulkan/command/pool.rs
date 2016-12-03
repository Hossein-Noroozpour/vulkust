use super::super::super::system::vulkan::{
    VkResult,
    VkCommandPool,
    VkStructureType,
    vkCreateCommandPool,
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
    pub vk_cmd_pool: VkCommandPool,
}

impl Pool {
    pub fn new(device: Arc<RwLock<Device>>, queue_family_index: u32) -> Self {
        let vk_cmd_pool_info = VkCommandPoolCreateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
            queueFamilyIndex: queue_family_index,
            flags: VkCommandPoolCreateFlagBits::VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT as u32,
            ..VkCommandPoolCreateInfo::default()
        };
        let mut vk_cmd_pool = 0 as VkCommandPool;
        {
            let dev = device.read().unwrap();
            vulkan_check!(vkCreateCommandPool(
                dev.vk_device, &vk_cmd_pool_info, 0 as *const VkAllocationCallbacks,
                &mut vk_cmd_pool));
        }
        Pool {
            device: device,
            vk_cmd_pool: vk_cmd_pool,
        }
    }
}

// TODO: write drop function for it