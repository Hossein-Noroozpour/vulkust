use std::sync::Arc;
use std::ptr::null_mut;
use std::mem::transmute;
use super::super::super::system::vulkan as vk;
use super::super::surface::Surface;

pub struct Physical {
    pub surface: Arc<Surface>,
    pub graphics_queue_node_index: u32,
    pub transfer_queue_node_index: u32,
    pub compute_queue_node_index: u32,
    pub present_queue_node_index: u32,
    pub vk_data: vk::VkPhysicalDevice,
    pub memory_properties: vk::VkPhysicalDeviceMemoryProperties,
}

#[derive(Debug, Clone, Copy)]
struct ScoreIndices {
    score: i32,
    graphics_queue_node_index: u32,
    transfer_queue_node_index: u32,
    compute_queue_node_index: u32,
    present_queue_node_index: u32,
}

impl Physical {
    pub fn new(surface: Arc<Surface>) -> Self {
        let devices = Self::enumerate_devices(surface.instance.vk_data);
        let (vk_data, si) = Self::choose_best_device(&devices, &surface);
        let mut memory_properties = vk::VkPhysicalDeviceMemoryProperties::default();
        unsafe {
            vk::vkGetPhysicalDeviceMemoryProperties(vk_data, &mut memory_properties);
        }
        let physical = Physical {
            surface: surface,
            graphics_queue_node_index: si.graphics_queue_node_index,
            transfer_queue_node_index: si.transfer_queue_node_index,
            compute_queue_node_index: si.compute_queue_node_index,
            present_queue_node_index: si.present_queue_node_index,
            vk_data: vk_data,
            memory_properties: memory_properties,
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
    fn choose_best_device(devices: &Vec<vk::VkPhysicalDevice>, surface: &Arc<Surface>)
        -> (vk::VkPhysicalDevice, ScoreIndices) {
        let mut highest_score =  ScoreIndices {
            score: -1,
            graphics_queue_node_index: u32::max_value(),
            transfer_queue_node_index: u32::max_value(),
            compute_queue_node_index: u32::max_value(),
            present_queue_node_index: u32::max_value(),
        };
        // if devices.len() == 1 {
        //     return (devices[0], 1);
        // }
        let mut device: vk::VkPhysicalDevice = null_mut();
        for d in devices {
            let score = Self::score_device(*d, surface);
            if score.score > highest_score.score {
                highest_score = score;
                device = *d;
            }
        }
        if highest_score.score < 0 {
            logf!("No appropriate device have been found!");
        }
        logi!("The chosen device is: {:?} and its score-indices is: {:?}", device, highest_score);
        return (device, highest_score);
    }
    fn get_device_queue_family_properties(
        device: vk::VkPhysicalDevice) -> Vec<vk::VkQueueFamilyProperties> {
        let mut count = 0u32;
        unsafe {
            vk::vkGetPhysicalDeviceQueueFamilyProperties(device, &mut count, null_mut());
        }
        if count == 0 {
            return Vec::new();
        }
        let mut queue_props = vec![vk::VkQueueFamilyProperties::default(); count as usize];
        unsafe {
            vk::vkGetPhysicalDeviceQueueFamilyProperties(
                device, &mut count, queue_props.as_mut_ptr());
        }
        queue_props
    }
    pub fn get_queue_family_properties(&self) -> Vec<vk::VkQueueFamilyProperties> {
        Self::get_device_queue_family_properties(self.vk_data)
    }
    fn score_device(device: vk::VkPhysicalDevice, surface: &Arc<Surface>) -> ScoreIndices {
        let mut score_indices = ScoreIndices {
            score: -1,
            graphics_queue_node_index: u32::max_value(),
            transfer_queue_node_index: u32::max_value(),
            compute_queue_node_index: u32::max_value(),
            present_queue_node_index: u32::max_value(),
        };
        let queue_family_properties = Self::get_device_queue_family_properties(device);
        if queue_family_properties.len() == 0 {
            return score_indices;
        }
        let mut supports_present = vec![false; queue_family_properties.len()];
        loge!("{:?}", surface.vk_data);
        loge!("111111111111111111111111111111111111111111111111111111111111111111111111111111111");
        loge!("111111111111111111111111111111111111111111111111111111111111111111111111111111111");
        loge!("111111111111111111111111111111111111111111111111111111111111111111111111111111111");
        loge!("111111111111111111111111111111111111111111111111111111111111111111111111111111111");
        loge!("111111111111111111111111111111111111111111111111111111111111111111111111111111111");
        loge!("111111111111111111111111111111111111111111111111111111111111111111111111111111111");
        loge!("111111111111111111111111111111111111111111111111111111111111111111111111111111111");
        loge!("111111111111111111111111111111111111111111111111111111111111111111111111111111111");
        use std::ffi::CString;
        let vk_proc_name = CString::new("vkGetPhysicalDeviceSurfaceSupportKHR").unwrap();
        let vk_get_physical_device_surface_support_khr:
            vk::PFN_vkGetPhysicalDeviceSurfaceSupportKHR = unsafe { transmute(
                vk::vkGetInstanceProcAddr(
                    surface.instance.vk_data, vk_proc_name.as_ptr()))};
        if vk_get_physical_device_surface_support_khr == unsafe { transmute(0usize) } {
            loge!("22222222222222222222222222222222222222222222222222222222222");
            loge!("22222222222222222222222222222222222222222222222222222222222");
            loge!("22222222222222222222222222222222222222222222222222222222222");
            loge!("22222222222222222222222222222222222222222222222222222222222");
            loge!("22222222222222222222222222222222222222222222222222222222222");
            loge!("22222222222222222222222222222222222222222222222222222222222");
            loge!("22222222222222222222222222222222222222222222222222222222222");
            loge!("22222222222222222222222222222222222222222222222222222222222");
        }
        // let pfn
        for i in 0..(queue_family_properties.len() as u32) {
            let mut b = 0 as vk::VkBool32;
            unsafe {
                vk_get_physical_device_surface_support_khr(device, i, surface.vk_data, &mut b);
                // vk::vkGetPhysicalDeviceSurfaceSupportKHR(device, i, surface.vk_data, &mut b);
            }
            if b != 0 {
                supports_present[i as usize] = true;
            }
        }
        loge!("3333333333333333333333333333333333333333333333333");
        loge!("3333333333333333333333333333333333333333333333333");
        loge!("3333333333333333333333333333333333333333333333333");
        loge!("3333333333333333333333333333333333333333333333333");
        loge!("3333333333333333333333333333333333333333333333333");
        loge!("3333333333333333333333333333333333333333333333333");
        loge!("3333333333333333333333333333333333333333333333333");
        loge!("3333333333333333333333333333333333333333333333333");
        for i in 0..queue_family_properties.len() {
            if ((queue_family_properties[i].queueFlags as u32) &
                (vk::VkQueueFlagBits::VK_QUEUE_GRAPHICS_BIT as u32)) != 0 &&
                ((queue_family_properties[i].queueFlags as u32) &
                    (vk::VkQueueFlagBits::VK_QUEUE_TRANSFER_BIT as u32)) != 0 &&
                ((queue_family_properties[i].queueFlags as u32) &
                    (vk::VkQueueFlagBits::VK_QUEUE_COMPUTE_BIT as u32)) != 0 &&
                supports_present[i] {
                score_indices.score = 100;
                score_indices.graphics_queue_node_index = i as u32;
                score_indices.transfer_queue_node_index = i as u32;
                score_indices.compute_queue_node_index = i as u32;
                score_indices.present_queue_node_index = i as u32;
                return score_indices;
            }
            if ((queue_family_properties[i].queueFlags as u32) &
                (vk::VkQueueFlagBits::VK_QUEUE_GRAPHICS_BIT as u32)) != 0 {
                score_indices.graphics_queue_node_index = i as u32;
            }
            if ((queue_family_properties[i].queueFlags as u32) &
                (vk::VkQueueFlagBits::VK_QUEUE_TRANSFER_BIT as u32)) != 0 {
                score_indices.transfer_queue_node_index = i as u32;
            }
            if ((queue_family_properties[i].queueFlags as u32) &
                (vk::VkQueueFlagBits::VK_QUEUE_COMPUTE_BIT as u32)) != 0 {
                score_indices.compute_queue_node_index = i as u32;
            }
            if supports_present[i] {
                score_indices.present_queue_node_index = i as u32;
            }
        }
        for i in 0..queue_family_properties.len() {
            if ((queue_family_properties[i].queueFlags as u32) &
                (vk::VkQueueFlagBits::VK_QUEUE_GRAPHICS_BIT as u32)) != 0 &&
                ((queue_family_properties[i].queueFlags as u32) &
                    (vk::VkQueueFlagBits::VK_QUEUE_TRANSFER_BIT as u32)) != 0 &&
                supports_present[i] {
                if score_indices.compute_queue_node_index != u32::max_value() {
                    score_indices.score = 90;
                } else {
                    score_indices.score = 50;
                }
                score_indices.graphics_queue_node_index = i as u32;
                score_indices.transfer_queue_node_index = i as u32;
                score_indices.present_queue_node_index = i as u32;
                return score_indices;
            }
        }
        for i in 0..queue_family_properties.len() {
            if ((queue_family_properties[i].queueFlags as u32) &
                (vk::VkQueueFlagBits::VK_QUEUE_GRAPHICS_BIT as u32)) != 0 &&
                supports_present[i] {
                if score_indices.compute_queue_node_index != u32::max_value() {
                    if score_indices.transfer_queue_node_index != u32::max_value() {
                        score_indices.score = 80;
                    } else {
                        score_indices.score = 30;
                    }
                } else {
                    if score_indices.transfer_queue_node_index != u32::max_value() {
                        score_indices.score = 40;
                    } else {
                        score_indices.score = 25;
                    }
                }
                score_indices.graphics_queue_node_index = i as u32;
                score_indices.present_queue_node_index = i as u32;
                return score_indices;
            }
        }
        logf!("Separate graphics and presenting queues are not supported yet!");
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
        logi!("gpu: {:?}, surface: {:?}", self.vk_data, self.surface.vk_data);
        vulkan_check!((vk_get_physical_device_surface_capabilities_khr)(
            self.vk_data, self.surface.vk_data, &mut caps));
        return caps;
    }
    pub fn get_surface_formats(&self) -> Vec<vk::VkSurfaceFormatKHR> {
        let mut count = 0u32;
        vulkan_check!(vk::vkGetPhysicalDeviceSurfaceFormatsKHR(
            self.vk_data, self.surface.vk_data, &mut count, null_mut()));
        let mut result = vec![vk::VkSurfaceFormatKHR::default(); count as usize];
        vulkan_check!(vk::vkGetPhysicalDeviceSurfaceFormatsKHR(
            self.vk_data, self.surface.vk_data, &mut count, result.as_mut_ptr()));
        result
    }
    pub fn get_memory_type_index(&self, type_bits: u32, properties: u32) -> u32 {
		// Iterate over all memory types available for the device used in this example
        let mut type_bits = type_bits;
		for i in 0..self.memory_properties.memoryTypeCount {
			if (type_bits & 1) == 1 {
				if (self.memory_properties.memoryTypes[i as usize].propertyFlags as u32)
                        & properties == properties {
					return i;
				}
			}
			type_bits >>= 1;
		}
		logf!("Could not find the requsted memory type.");
	}
}

impl Drop for Physical {
    fn drop(&mut self) {
    }
}
