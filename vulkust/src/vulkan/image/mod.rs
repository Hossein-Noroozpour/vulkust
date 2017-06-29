pub mod view;

use super::super::system::vulkan as vk;
use super::device::logical::Logical as LogicalDevice;
use super::memory::allocate_with_requirements;

//use std::default::Default;
use std::sync::Arc;
use std::ptr::null;

pub struct Image {
    pub logical_device: Arc<LogicalDevice>,
    pub vk_data: vk::VkImage,
    //    pub vk_format: VkFormat,
    pub vk_mem: vk::VkDeviceMemory,
}

impl Image {
    pub fn new_with_info(logical_device: Arc<LogicalDevice>, info: &vk::VkImageCreateInfo) -> Self {
        let mut vk_data = 0 as vk::VkImage;
        vulkan_check!(vk::vkCreateImage(
            logical_device.vk_data,
            info,
            null(),
            &mut vk_data
        ));
        let mut mem_requirements = vk::VkMemoryRequirements::default();
        unsafe {
            vk::vkGetImageMemoryRequirements(
                logical_device.vk_data,
                vk_data,
                &mut mem_requirements,
            );
        }
        let memory = allocate_with_requirements(&logical_device, mem_requirements);
        vulkan_check!(vk::vkBindImageMemory(
            logical_device.vk_data,
            vk_data,
            memory,
            0
        ));
        Image {
            logical_device: logical_device,
            vk_data: vk_data,
            vk_mem: memory,
        }
    }
    pub fn new_with_vk_data(logical_device: Arc<LogicalDevice>, vk_image: vk::VkImage) -> Self {
        Image {
            logical_device: logical_device,
            vk_data: vk_image,
            vk_mem: 0 as vk::VkDeviceMemory,
        }
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe {
            if self.vk_mem != 0 as vk::VkDeviceMemory {
                vk::vkDestroyImage(self.logical_device.vk_data, self.vk_data, null());
            }
            vk::vkFreeMemory(self.logical_device.vk_data, self.vk_mem, null());
        }
    }
}
