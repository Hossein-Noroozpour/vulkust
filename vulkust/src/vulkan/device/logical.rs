use std::sync::Arc;
use std::ptr::null;
use std::default::Default;
use std::collections::HashSet;
use super::super::super::system::vulkan as vk;
use super::physical::Physical;
use super::super::super::util::string::{
    strings_to_cstrings,
    cstrings_to_ptrs,
};

pub struct Logical {
    pub physical_device: Arc<Physical>,
    pub vk_data: vk::VkDevice,
}

impl Logical {
    pub fn new(physical_device: Arc<Physical>) -> Self {
        let mut device_extensions = Vec::new();
        device_extensions.push("VK_KHR_swapchain".to_string());
        let device_extensions = strings_to_cstrings(device_extensions);
        let device_extensions = cstrings_to_ptrs(&device_extensions);
        let mut queue_family_index_set = HashSet::new();
        queue_family_index_set.insert(physical_device.graphics_queue_node_index);
        queue_family_index_set.insert(physical_device.transfer_queue_node_index);
        queue_family_index_set.insert(physical_device.compute_queue_node_index);
        queue_family_index_set.insert(physical_device.present_queue_node_index);
        let mut queue_create_info_s = Vec::new();
        let queue_priorities = vec![1f32];
        // TODO: create as many as possible queue to separate independent works as many as possible
        // on the queues but it is not required currently
        for q in queue_family_index_set {
            let mut queue_create_info = vk::VkDeviceQueueCreateInfo::default();
            queue_create_info.sType =
                vk::VkStructureType::VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO;
            queue_create_info.queueCount = 1;
            queue_create_info.queueFamilyIndex = q;
            queue_create_info.pQueuePriorities = queue_priorities.as_ptr();
            queue_create_info_s.push(queue_create_info);
        }
        let mut device_create_info = vk::VkDeviceCreateInfo::default();
        device_create_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO;
        device_create_info.queueCreateInfoCount = queue_create_info_s.len() as u32;
        device_create_info.pQueueCreateInfos = queue_create_info_s.as_ptr();
        device_create_info.enabledExtensionCount = device_extensions.len() as u32;
        device_create_info.ppEnabledExtensionNames = device_extensions.as_ptr();
        let mut vk_data = 0 as vk::VkDevice;
        vulkan_check!(vk::vkCreateDevice(
            physical_device.vk_data, &device_create_info, null(), &mut vk_data));
        Logical {
            physical_device: physical_device,
            vk_data: vk_data,
        }
    }
}

impl Drop for Logical {
    fn drop(&mut self) {
        unsafe {
            vk::vkDestroyDevice(self.vk_data, null());
        }
    }
}