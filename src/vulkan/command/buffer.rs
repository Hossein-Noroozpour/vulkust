use super::super::super::system::vulkan::{
    VkResult,
    VkCommandBuffer,
    VkStructureType,
    VkCommandBufferLevel,
    vkBeginCommandBuffer,
    VkCommandBufferBeginInfo,
    vkAllocateCommandBuffers,
    VkCommandBufferAllocateInfo,
};

use super::pool::Pool;

use std::sync::{
    Arc,
    RwLock,
};
use std::default::Default;

pub struct Buffer {
    pool: Arc<RwLock<Pool>>,
    vk_buffer: VkCommandBuffer,
}

impl Buffer {
    pub fn new(cmd_pool: Arc<RwLock<Pool>>) -> Self {
        let pool = cmd_pool.read().unwrap();
        let device = pool.device.read().unwrap();
        let cmd_buf_allocate_info = VkCommandBufferAllocateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
            commandPool: pool.vk_cmd_pool,
            level: VkCommandBufferLevel::VK_COMMAND_BUFFER_LEVEL_PRIMARY,
            commandBufferCount: 1,
            ..VkCommandBufferAllocateInfo::default()
        };
        let mut vk_buffer = 0 as VkCommandBuffer;
        vulkan_check!(vkAllocateCommandBuffers(
            device.vk_device, &cmd_buf_allocate_info as *const VkCommandBufferAllocateInfo,
            &mut vk_buffer as *mut VkCommandBuffer));
        let cmd_buf_info = VkCommandBufferBeginInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
            ..VkCommandBufferBeginInfo::default()
        };
        vulkan_check!(vkBeginCommandBuffer(vk_buffer, &cmd_buf_info));
        Buffer {
            pool: cmd_pool.clone(),
            vk_buffer: vk_buffer,
        }
    }
}