use super::super::system::vulkan as vk;
use super::device::logical::Logical as LogicalDevice;
use std::sync::Arc;
use std::default::Default;
use std::ptr::null;

pub struct Fence {
    pub logical_device: Arc<LogicalDevice>,
    pub vk_data: vk::VkFence,
}

impl Fence {
    pub fn new(logical_device: Arc<LogicalDevice>) -> Self {
        let mut fence_create_info = vk::VkFenceCreateInfo::default();
        fence_create_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_FENCE_CREATE_INFO;
        let mut vk_data = 0 as vk::VkFence;
        vulkan_check!(vk::vkCreateFence(
            logical_device.vk_data,
            &fence_create_info,
            null(),
            &mut vk_data,
        ));
        Fence {
            logical_device: logical_device,
            vk_data: vk_data,
        }
    }
    pub fn new_signaled(logical_device: Arc<LogicalDevice>) -> Self {
        let mut fence_create_info = vk::VkFenceCreateInfo::default();
        fence_create_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_FENCE_CREATE_INFO;
        fence_create_info.flags = vk::VkFenceCreateFlagBits::VK_FENCE_CREATE_SIGNALED_BIT as u32;
        let mut vk_data = 0 as vk::VkFence;
        vulkan_check!(vk::vkCreateFence(
            logical_device.vk_data,
            &fence_create_info,
            null(),
            &mut vk_data,
        ));
        Fence {
            logical_device: logical_device,
            vk_data: vk_data,
        }
    }
    pub fn wait(&self) {
        vulkan_check!(vk::vkWaitForFences(
            self.logical_device.vk_data,
            1,
            &self.vk_data,
            1u32,
            100000000000,
        ));
    }
}

impl Drop for Fence {
    fn drop(&mut self) {
        unsafe {
            vk::vkDestroyFence(self.logical_device.vk_data, self.vk_data, null());
        }
    }
}
