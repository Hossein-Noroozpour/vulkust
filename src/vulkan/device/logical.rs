use std::sync::Arc;
use std::ptr::{null, null_mut};
use std::ffi::CString;
use std::default::Default;
use super::super::super::system::vulkan::{
    VkResult,
    VkDevice,
    vkCreateDevice,
    vkDestroyDevice,
    VkStructureType,
    VkQueueFlagBits,
    VkDeviceCreateInfo,
    VkDeviceQueueCreateInfo,
};
use super::physical::Physical;

pub struct Logical {
    pub physical_device: Arc<Physical>,
    pub vk_device: VkDevice,
}

impl Logical {
    pub fn new(physical_device: Arc<Physical>) -> Self {
        let mut this = Logical {
            physical_device: physical_device.clone(),
            vk_device: null_mut(),
        };
        let mut queue_create_infos = vec![VkDeviceQueueCreateInfo::default(); 1];
        let default_queue_priority = 0f32;
        // graphic, compute, transfer
        let mut queue_family_indices = Vec::<u32>::new();
        let queue_family_properties = physical_device.get_queue_family_properties();
        let get_queue_family_index = move |queue_flags: VkQueueFlagBits| -> u32 {
            if queue_flags as u32 & VkQueueFlagBits::VK_QUEUE_COMPUTE_BIT as u32 != 0 {
                for i in 0..queue_family_properties.len() {
                    if queue_family_properties[i].queueFlags as u32 & queue_flags as u32 != 0 &&
                        queue_family_properties[i].queueFlags as u32 &
                            VkQueueFlagBits::VK_QUEUE_GRAPHICS_BIT as u32 == 0 {
                        return i as u32;
                    }
                }
            }
            if queue_flags as u32 & VkQueueFlagBits::VK_QUEUE_TRANSFER_BIT as u32 != 0 {
                for i in 0..queue_family_properties.len() {
                    if queue_family_properties[i].queueFlags as u32 & queue_flags as u32 != 0 &&
                        queue_family_properties[i].queueFlags as u32 &
                            VkQueueFlagBits::VK_QUEUE_GRAPHICS_BIT as u32 == 0 &&
                        queue_family_properties[i].queueFlags as u32 &
                            VkQueueFlagBits::VK_QUEUE_COMPUTE_BIT as u32 == 0 {
                        return i as u32;
                    }
                }
            }
            for i in 0..queue_family_properties.len() {
                if queue_family_properties[i].queueFlags as u32 & queue_flags as u32 != 0 {
                    return i as u32;
                }
            }
            logerr!(format!("No queue family found for {:?}", queue_flags));
            return 0xFFFFFFFF;
        };
        queue_family_indices.push(get_queue_family_index(VkQueueFlagBits::VK_QUEUE_GRAPHICS_BIT));
        queue_create_infos[0].sType = VkStructureType::VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO;
        queue_create_infos[0].queueFamilyIndex = queue_family_indices[0];
        queue_create_infos[0].queueCount = 1;
        queue_create_infos[0].pQueuePriorities = &default_queue_priority;
        queue_family_indices.push(get_queue_family_index(VkQueueFlagBits::VK_QUEUE_COMPUTE_BIT));
        if queue_family_indices[1] != 0xFFFFFFFF &&
            queue_family_indices[1] != queue_family_indices[0] {
            let mut queue_info = VkDeviceQueueCreateInfo::default();
            queue_info.sType = VkStructureType::VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO;
            queue_info.queueFamilyIndex = queue_family_indices[1];
            queue_info.queueCount = 1;
            queue_info.pQueuePriorities = &default_queue_priority;
            queue_create_infos.push(queue_info);
        }
        queue_family_indices.push(get_queue_family_index(VkQueueFlagBits::VK_QUEUE_TRANSFER_BIT));
        if queue_family_indices[2] != 0xFFFFFFFF &&
            queue_family_indices[2] != queue_family_indices[1] &&
            queue_family_indices[2] != queue_family_indices[0] {
            let mut queue_info = VkDeviceQueueCreateInfo::default();
            queue_info.sType = VkStructureType::VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO;
            queue_info.queueFamilyIndex = queue_family_indices[2];
            queue_info.queueCount = 1;
            queue_info.pQueuePriorities = &default_queue_priority;
            queue_create_infos.push(queue_info);
        }
        let vk_khr_swapchain_ext = CString::new("VK_KHR_swapchain").unwrap();
        let vulkan_extensions = [
            vk_khr_swapchain_ext.as_ptr()
        ];
        let mut device_create_info = VkDeviceCreateInfo::default();
        device_create_info.sType = VkStructureType::VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO;
        device_create_info.queueCreateInfoCount = queue_create_infos.len() as u32;
        device_create_info.pQueueCreateInfos = queue_create_infos.as_ptr();
// device_create_info.pEnabledFeatures = &enabledFeatures; TODO: maybe in future i need this
        device_create_info.enabledExtensionCount = vulkan_extensions.len() as u32;
        device_create_info.ppEnabledExtensionNames = vulkan_extensions.as_ptr();
        vulkan_check!(vkCreateDevice(
            physical_device.vk_physical_device, &device_create_info, null(),
            &mut this.vk_device));
// commandPool = createCommandPool(queueFamilyIndices.graphics); !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// it has command pool but I don't know
        return this;
    }
}

impl Drop for Logical {
    fn drop(&mut self) {
        unsafe {
            vkDestroyDevice(self.vk_device, null());
        }
    }
}