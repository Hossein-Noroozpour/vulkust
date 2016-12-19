use super::super::super::system::vulkan::{
    VkFence,
    VkResult,
    VkSubmitInfo,
    VkCommandBuffer,
    VkStructureType,
    VkFenceCreateInfo,
    vkFreeCommandBuffers,
    VkCommandBufferLevel,
    vkBeginCommandBuffer,
    VkCommandBufferBeginInfo,
    vkAllocateCommandBuffers,
    VkCommandBufferAllocateInfo,
};

use std::sync::{
    Arc,
    RwLock,
};
use std::default::Default;

pub struct Fence {
    device: Arc<RwLock<Device>>,
    vk_fence: VkFence,
}

impl Fence {
    pub fn new(device: Arc<RwLock<Device>>) -> Self {
        let dev = device.read().unwrap();
        let fence_create_info = VkFenceCreateInfo::default();
        fence_create_info.sType = VkStructureType::VK_STRUCTURE_TYPE_FENCE_CREATE_INFO;
        let vk_fence = 0 as VkFence;
        vulkan_check!(vkCreateFence(dev.vk_device, &fence_create_info, 0, &vk_fence));
        Fence {
            device: device.clone(),
            vk_fence: vk_fence,
        }
    }

    pub fn wait(&self) {
        let dev = self.device.read().unwrap();
        vulkan_check!(vkWaitForFences(dev.vk_device, 1, &self.vk_fence, VK_TRUE, DEFAULT_FENCE_TIMEOUT));
        vkDestroyFence(device, fence, nullptr);
    }
}

impl Drop for Fence {
    fn drop(&mut self) {
        let dev = self.device.read().unwrap();
        unsafe {
            vkDestroyFence(dev.vk_device, self.vk_fence, 0 as *const VkAllocationCallbacks);
        }
    }
}