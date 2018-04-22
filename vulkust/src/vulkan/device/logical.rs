use super::super::super::core::string::{cstrings_to_ptrs, strings_to_cstrings};
use super::super::vulkan as vk;
use super::physical::Physical;
use std::collections::HashSet;
use std::ptr::null;
use std::sync::Arc;

pub struct Logical {
    pub physical_device: Arc<Physical>,
    pub vk_data: vk::VkDevice,
    pub vk_graphic_queue: vk::VkQueue,
    pub vk_compute_queue: vk::VkQueue,
    pub vk_present_queue: vk::VkQueue,
}

impl Logical {
    pub fn new(physical_device: &Arc<Physical>) -> Self {
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
            physical_device.vk_data,
            &device_create_info,
            null(),
            &mut vk_data,
        ));
        let mut vk_graphic_queue = 0 as vk::VkQueue;
        unsafe {
            vk::vkGetDeviceQueue(
                vk_data,
                physical_device.graphics_queue_node_index,
                0,
                &mut vk_graphic_queue,
            );
        }
        let mut vk_compute_queue = 0 as vk::VkQueue;
        unsafe {
            vk::vkGetDeviceQueue(
                vk_data,
                physical_device.compute_queue_node_index,
                0,
                &mut vk_compute_queue,
            );
        }
        let mut vk_present_queue = 0 as vk::VkQueue;
        unsafe {
            vk::vkGetDeviceQueue(
                vk_data,
                physical_device.present_queue_node_index,
                0,
                &mut vk_present_queue,
            );
        }
        Logical {
            physical_device: physical_device.clone(),
            vk_data,
            vk_graphic_queue,
            vk_compute_queue,
            vk_present_queue,
        }
    }
    pub fn wait_idle(&self) {
        unsafe {
            vk::vkDeviceWaitIdle(self.vk_data);
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
