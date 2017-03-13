use std::sync::Arc;
use std::ptr::null_mut;
use std::mem::transmute;
use super::super::super::system::vulkan::{
    VkResult,
    VkFormat,
    VkPhysicalDevice,
    VkFormatProperties,
    VkFormatFeatureFlagBits,
    VkQueueFamilyProperties,
    VkSurfaceCapabilitiesKHR,
    vkEnumeratePhysicalDevices,
    vkGetPhysicalDeviceFormatProperties,
    vkGetPhysicalDeviceQueueFamilyProperties,
    PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR,
};
use super::super::instance::Instance;
use super::super::surface::Surface;
// this properties of device is using in setup process no other time, no in rendering procedure
// if this assumption was wrong cache this properties in this structure.
//     Properties
//     Features
//     MemoryProperties
//     QueueFamilyProperties
pub struct Physical {
    pub instance: Arc<Instance>,
    pub vk_physical_device: VkPhysicalDevice,
}
impl Physical {
    pub fn new(instance: Arc<Instance>) -> Self {
        let mut physical = Physical {
            instance: instance,
            vk_physical_device: 0 as VkPhysicalDevice,
        };
        physical.init_physical_device();
        physical
    }
    fn init_physical_device(&mut self) {
        let mut gpu_count = 0u32;
        vulkan_check!(vkEnumeratePhysicalDevices(
            self.instance.vk_instance, &mut gpu_count as *mut u32, 0 as *mut VkPhysicalDevice));
        logdbg!(format!("Number of devices is: {}", gpu_count));
        let mut devices = vec![0 as VkPhysicalDevice; gpu_count as usize];
        vulkan_check!(vkEnumeratePhysicalDevices(
            self.instance.vk_instance, &mut gpu_count,
            devices.as_mut_ptr() as *mut VkPhysicalDevice));
        self.vk_physical_device = devices[0];
    }
    pub fn get_queue_family_properties(&self) -> Vec<VkQueueFamilyProperties> {
        let mut count = 0u32;
        unsafe {
            vkGetPhysicalDeviceQueueFamilyProperties(
                self.vk_physical_device, &mut count, null_mut());
        }
        let mut families = vec![VkQueueFamilyProperties::default(); count as usize];
        unsafe {
            vkGetPhysicalDeviceQueueFamilyProperties(
                self.vk_physical_device, &mut count, families.as_mut_ptr());
        }
        return families;
    }
    pub fn get_supported_depth_format(&self) -> VkFormat {
        let depth_formats = vec![
            VkFormat::VK_FORMAT_D32_SFLOAT_S8_UINT,
            VkFormat::VK_FORMAT_D32_SFLOAT,
            VkFormat::VK_FORMAT_D24_UNORM_S8_UINT,
            VkFormat::VK_FORMAT_D16_UNORM_S8_UINT,
            VkFormat::VK_FORMAT_D16_UNORM,
        ];
        for format in depth_formats {
            let mut format_props = VkFormatProperties::default();
            unsafe {
                vkGetPhysicalDeviceFormatProperties(
                    self.vk_physical_device, format, &mut format_props);
            }
            if format_props.optimalTilingFeatures as u32 &
                VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT as u32
                != 0 {
                return format;
            }
        }
        logftl!("No depth format found!");
    }
    pub fn get_surface_capabilities(&self, surface: Arc<Surface>) -> VkSurfaceCapabilitiesKHR {
        let mut caps = VkSurfaceCapabilitiesKHR::default();
        let vk_get_physical_device_surface_capabilities_khr:
            PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR = unsafe {transmute(
            self.instance.get_function("vkGetPhysicalDeviceSurfaceCapabilitiesKHR")
        )};
        logdbg!(format!("gpu: {:?}, surface: {:?}", self.vk_physical_device, surface.vk_surface));
        vulkan_check!((vk_get_physical_device_surface_capabilities_khr)(
            self.vk_physical_device, surface.vk_surface, &mut caps));
        logerr!("reached");
        return caps;
    }
}
impl Drop for Physical {
    fn drop(&mut self) {
//        unsafe {} TODO: it may be unnecessary
    }
}