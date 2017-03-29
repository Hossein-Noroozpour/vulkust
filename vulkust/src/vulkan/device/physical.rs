use std::sync::Arc;
use std::ptr::null_mut;
use std::mem::transmute;
use super::super::super::system::vulkan as vk;
use super::super::surface::Surface;
pub struct Physical {
    pub surface: Arc<Surface>,
    graphics_queue_node_index: u32,
	transfer_queue_node_index: u32,
	compute_queue_node_index: u32,
	present_queue_node_index: u32,
    pub vk_data: vk::VkPhysicalDevice,
}
impl Physical {
    pub fn new(surface: Arc<Surface>) -> Self {
        let mut physical = Physical {
            surface: surface,
            graphics_queue_node_index: u32::max_value(),
        	transfer_queue_node_index: u32::max_value(),
        	compute_queue_node_index: u32::max_value(),
        	present_queue_node_index: u32::max_value(),
            vk_data: 0 as vk::VkPhysicalDevice,
        };
        physical
    }
    fn enumerate_devices(vk_instance: vk::VkInstance) -> Vec<vk::VkPhysicalDevice> {
        let mut gpu_count = 0u32;
        vulkan_check!(vk::vkEnumeratePhysicalDevices(
            vk_instance, &mut gpu_count as *mut u32, null_mut()));
        logi!("Number of devices is: {}", gpu_count);
        let mut devices = vec![0 as vk::VkPhysicalDevice; gpu_count as usize];
        vulkan_check!(vk::vkEnumeratePhysicalDevices(
            vk_instance, &mut gpu_count, devices.as_mut_ptr()));
        devices
    }
    pub fn get_queue_family_properties(&self) -> Vec<vk::VkQueueFamilyProperties> {
        let mut count = 0u32;
        unsafe {
            vk::vkGetPhysicalDeviceQueueFamilyProperties(
                self.vk_data, &mut count, null_mut());
        }
        let mut families = vec![vk::VkQueueFamilyProperties::default(); count as usize];
        unsafe {
            vk::vkGetPhysicalDeviceQueueFamilyProperties(
                self.vk_data, &mut count, families.as_mut_ptr());
        }
        return families;
    }
    pub fn get_supported_depth_format(&self) -> vk::VkFormat {
        let depth_formats = vec![
            vk::VkFormat::VK_FORMAT_D32_SFLOAT_S8_UINT,
            vk::VkFormat::VK_FORMAT_D32_SFLOAT,
            vk::VkFormat::VK_FORMAT_D24_UNORM_S8_UINT,
            vk::VkFormat::VK_FORMAT_D16_UNORM_S8_UINT,
            vk::VkFormat::VK_FORMAT_D16_UNORM,
        ];
        for format in depth_formats {
            let mut format_props = vk::VkFormatProperties::default();
            unsafe {
                vk::vkGetPhysicalDeviceFormatProperties(
                    self.vk_data, format, &mut format_props);
            }
            if format_props.optimalTilingFeatures as u32 &
                vk::VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT as u32
                != 0 {
                return format;
            }
        }
        logf!("No depth format found!");
    }
    pub fn get_surface_capabilities(&self) -> vk::VkSurfaceCapabilitiesKHR {
        let mut caps = vk::VkSurfaceCapabilitiesKHR::default();
        let vk_get_physical_device_surface_capabilities_khr:
            vk::PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR = unsafe {
                transmute(self.surface.instance.get_function(
                    "vkGetPhysicalDeviceSurfaceCapabilitiesKHR"))
            };
        logi!("gpu: {:?}, surface: {:?}", self.vk_data, self.surface.vk_surface);
        vulkan_check!((vk_get_physical_device_surface_capabilities_khr)(
            self.vk_data, self.surface.vk_surface, &mut caps));
        return caps;
    }
}
impl Drop for Physical {
    fn drop(&mut self) {
//        unsafe {} TODO: it may be unnecessary
    }
}
