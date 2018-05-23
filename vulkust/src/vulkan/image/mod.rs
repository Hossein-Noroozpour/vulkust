pub mod view;

use super::device::logical::Logical as LogicalDevice;
use super::memory::{
    Location as MemeoryLocation,
    Manager as MemeoryManager, 
    Memory, 
};
use super::vulkan as vk;

use std::ptr::null;
use std::sync::{Arc, RwLock};

pub struct Image {
    pub logical_device: Arc<LogicalDevice>,
    pub vk_data: vk::VkImage,
    pub memory: Option<Arc<RwLock<Memory>>>,
}

impl Image {
    pub fn new_with_info(
        logical_device: Arc<LogicalDevice>,
        info: &vk::VkImageCreateInfo,
        memory_mgr: &Arc<RwLock<MemeoryManager>>,
    ) -> Self {
        let mut vk_data = 0 as vk::VkImage;
        vulkan_check!(vk::vkCreateImage(
            logical_device.vk_data,
            info,
            null(),
            &mut vk_data,
        ));
        let mut mem_reqs = vk::VkMemoryRequirements::default();
        unsafe {
            vk::vkGetImageMemoryRequirements(
                logical_device.vk_data,
                vk_data,
                &mut mem_reqs,
            );
        }
        let memory = vxresult!(memory_mgr.write()).allocate(&mem_reqs, MemeoryLocation::GPU);
        {
            let memory_r = vxresult!(memory.read());
            let root_memory = vxresult!(memory_r.root_memory.read());
            vulkan_check!(vk::vkBindImageMemory(
                logical_device.vk_data,
                vk_data,
                root_memory.vk_data,
                memory_r.info.offset as vk::VkDeviceSize,
            ));
        }
        Image {
            logical_device,
            vk_data,
            memory: Some(memory),
        }
    }

    pub fn new_with_vk_data(logical_device: Arc<LogicalDevice>, vk_image: vk::VkImage) -> Self {
        Image {
            logical_device,
            vk_data: vk_image,
            memory: None,
        }
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        if self.memory.is_some() {
            unsafe { vk::vkDestroyImage(self.logical_device.vk_data, self.vk_data, null()); }
        }
    }
}
