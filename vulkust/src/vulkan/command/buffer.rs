use super::super::super::system::vulkan as vk;

use super::pool::Pool;
// use super::super::fence::Fence;
use std::sync::Arc;
use std::default::Default;

pub struct Buffer {
    pool: Arc<Pool>,
    vk_data: vk::VkCommandBuffer,
}

impl Buffer {
    pub fn new(pool: Arc<Pool>) -> Self {
        let mut cmd_buf_allocate_info = vk::VkCommandBufferAllocateInfo::default();
        cmd_buf_allocate_info.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO;
        cmd_buf_allocate_info.commandPool = pool.vk_data;
        cmd_buf_allocate_info.level = vk::VkCommandBufferLevel::VK_COMMAND_BUFFER_LEVEL_PRIMARY;
        cmd_buf_allocate_info.commandBufferCount = 1;
        let mut vk_data = 0 as vk::VkCommandBuffer;
        vulkan_check!(vk::vkAllocateCommandBuffers(
            pool.logical_device.vk_data, &cmd_buf_allocate_info, &mut vk_data));
        let mut cmd_buf_info = vk::VkCommandBufferBeginInfo::default();
        cmd_buf_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO;
        vulkan_check!(vk::vkBeginCommandBuffer(vk_data, &cmd_buf_info));
        Buffer {
            pool: pool.clone(),
            vk_data: vk_data,
        }
    }
    pub fn flush(&self) {;
        let fence = Fence::new(pool.device.clone());
        vulkan_check!(vk::vkEndCommandBuffer(self.vk_data));
        let mut submit_info = vk::VkSubmitInfo::default();
        submit_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_SUBMIT_INFO;
        submit_info.commandBufferCount = 1;
        submit_info.pCommandBuffers = &self.vk_data;
        vulkan_check!(vk::vkQueueSubmit(dev.vk_queue, 1, &submit_info, fence.vk_data));
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
