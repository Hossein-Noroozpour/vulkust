use super::super::super::system::vulkan::{
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

use super::pool::Pool;
use super::super::fence::Fence;

use std::sync::{
    Arc,
    RwLock,
};
use std::default::Default;

pub struct Buffer {
    pool: Arc<Pool>,
    vk_buffer: VkCommandBuffer,
}

impl Buffer {
    pub fn new(cmd_pool: Arc<Pool>) -> Self {
        let mut cmd_buf_allocate_info = VkCommandBufferAllocateInfo::default();
        cmd_buf_allocate_info.sType = VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO;
        cmd_buf_allocate_info.commandPool = pool.vk_pool;
        cmd_buf_allocate_info.level = VkCommandBufferLevel::VK_COMMAND_BUFFER_LEVEL_PRIMARY;
        cmd_buf_allocate_info.commandBufferCount = 1;
        let mut vk_buffer = 0 as VkCommandBuffer;
        vulkan_check!(vkAllocateCommandBuffers(
            device.vk_device, &cmd_buf_allocate_info as *const VkCommandBufferAllocateInfo,
            &mut vk_buffer as *mut VkCommandBuffer));
        let cmd_buf_info = VkCommandBufferBeginInfo::default();
        cmd_buf_info.sType = VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO;
        vulkan_check!(vkBeginCommandBuffer(vk_buffer, &cmd_buf_info));
        Buffer {
            pool: cmd_pool.clone(),
            vk_buffer: vk_buffer,
        }
    }
    pub fn flush(&self) {;
        let fence = Fence::new(pool.device.clone());
        vulkan_check!(vkEndCommandBuffer(self.vk_buffer));
        let submit_info = VkSubmitInfo::default();
        submit_info.sType = VkStructureType::VK_STRUCTURE_TYPE_SUBMIT_INFO;
        submit_info.commandBufferCount = 1;
        submit_info.pCommandBuffers = &self.vk_buffer as *const VkCommandBuffer;
        vulkan_check!(vkQueueSubmit(dev.vk_queue, 1, &submit_info, fence.vk_fence));
        fence.wait();
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        let device = pool.device.read().unwrap();
        unsafe {
            vkFreeCommandBuffers(device.vk_device, pool.vk_pool, 1, &mut self.vk_buffer);
        }
    }
}