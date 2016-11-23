use super::instance::Instance;

use super::super::system::vulkan::{
    VkQueue,
    uint32_t,
    VkDevice,
    VkResult,
    vkCreateDevice,
    VkStructureType,
    vkDestroyDevice,
    VkQueueFlagBits,
    VkPhysicalDevice,
    vkGetDeviceQueue,
    VkDeviceCreateInfo,
    VkAllocationCallbacks,
    VkDeviceQueueCreateInfo,
    VkQueueFamilyProperties,
    vkEnumeratePhysicalDevices,
    vkGetPhysicalDeviceQueueFamilyProperties,
};

use std::default::Default;
use std::ffi::CString;
use std::sync::{
    Arc,
    RwLock,
};

pub struct Device {
    pub instance: Arc<RwLock<Instance>>,
    pub device: VkDevice,
    pub gpu: VkPhysicalDevice,
    pub graphics_family_index: u32,
}

impl Device {
    pub fn new(instance: Arc<RwLock<Instance>>) -> Self {
        let mut gpu_count: uint32_t = 0;
        let ins = instance.read().unwrap();
        vulkan_check!(
            vkEnumeratePhysicalDevices(
                ins.vk_instance,
                &mut gpu_count as *mut uint32_t,
                0 as *mut VkPhysicalDevice));
        println!("Number of devices is: {}", gpu_count);
        let mut devices = vec![0 as VkPhysicalDevice; gpu_count as usize];
        vulkan_check!(
            vkEnumeratePhysicalDevices(
                ins.vk_instance,
                &mut gpu_count as *mut uint32_t,
                devices.as_mut_ptr() as *mut VkPhysicalDevice));
        let gpu = devices[0]; // TODO: it can be better
        let mut family_count: uint32_t = 0;
        unsafe {
            vkGetPhysicalDeviceQueueFamilyProperties(
                gpu, &mut family_count as *mut uint32_t, 0 as *mut VkQueueFamilyProperties);
        }
        let mut family_property_list = vec![VkQueueFamilyProperties::default(); family_count as usize];
        unsafe {
            vkGetPhysicalDeviceQueueFamilyProperties(
                gpu, &mut family_count as *mut uint32_t,
                family_property_list.as_mut_ptr() as *mut VkQueueFamilyProperties);
        }
        let mut found = false;
        let mut graphics_family_index = 0;
        for i in 0..(family_count as usize) {
            if family_property_list[i].queueFlags &
                (VkQueueFlagBits::VK_QUEUE_GRAPHICS_BIT as u32) != 0 {
                found = true;
                graphics_family_index = i as u32;
                break;
            }
        }
        if !found {
            panic!("Queue family supporting graphics not found.");
        }
        let queue_priorities = [1.0f32];
        let device_queue_create_info = VkDeviceQueueCreateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
            queueFamilyIndex: graphics_family_index,
            queueCount: 1,
            pQueuePriorities: queue_priorities.as_ptr() as *const f32,
            ..VkDeviceQueueCreateInfo::default()
        };
        let vk_khr_swapchain_ext = CString::new("VK_KHR_swapchain").unwrap();
        let vulkan_extensions = [
            vk_khr_swapchain_ext.as_ptr()];
        let device_create_info = VkDeviceCreateInfo {
            sType: VkStructureType::VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
            queueCreateInfoCount: 1,
            pQueueCreateInfos: &device_queue_create_info,
            enabledExtensionCount: vulkan_extensions.len() as u32,
            ppEnabledExtensionNames: vulkan_extensions.as_ptr(),
            ..VkDeviceCreateInfo::default()
        };
        let mut device = 0 as VkDevice;
        let mut queue = 0 as VkQueue;
        vulkan_check!(vkCreateDevice(
            gpu, &device_create_info as *const VkDeviceCreateInfo,
            0 as *const VkAllocationCallbacks, &mut device as *mut VkDevice));
        unsafe {
            vkGetDeviceQueue(device, graphics_family_index, 0, &mut queue as *mut VkQueue);
        }
        Device {
            instance: instance.clone(),
            device: device,
            gpu: gpu,
            graphics_family_index: graphics_family_index,
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            vkDestroyDevice(self.device, 0 as *const VkAllocationCallbacks);
        }
    }
}