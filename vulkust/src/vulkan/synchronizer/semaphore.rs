use std::sync::Arc;
use std::ptr::null;
use std::default::Default;
use super::super::super::system::vulkan as vk;
use super::super::device::logical::Logical as LogicalDevice;
pub struct Semaphore {
    pub logical_device: Arc<LogicalDevice>,
    pub vk_data: vk::VkSemaphore,
}
impl Semaphore {
    pub fn new(logical_device: Arc<LogicalDevice>) -> Self {
        let mut vk_data = 0 as vk::VkSemaphore;
        let mut semaphore_create_info = vk::VkSemaphoreCreateInfo::default();
        semaphore_create_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO;
        vulkan_check!(vk::vkCreateSemaphore(
            logical_device.vk_data,
            &semaphore_create_info,
            null(),
            &mut vk_data
        ));
        Semaphore {
            logical_device: logical_device,
            vk_data: vk_data,
        }
    }
}
impl Drop for Semaphore {
    fn drop(&mut self) {
        unsafe {
            vk::vkDestroySemaphore(self.logical_device.vk_data, self.vk_data, null());
        }
    }
}
