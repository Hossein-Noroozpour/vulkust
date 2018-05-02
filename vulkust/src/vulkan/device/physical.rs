use super::super::surface::Surface;
use super::super::vulkan as vk;
use std::cmp::max;
use std::mem::transmute;
use std::ptr::null_mut;
use std::sync::Arc;

pub struct Physical {
    pub surface: Arc<Surface>,
    pub graphics_queue_node_index: u32,
    pub transfer_queue_node_index: u32,
    pub compute_queue_node_index: u32,
    pub present_queue_node_index: u32,
    pub vk_data: vk::VkPhysicalDevice,
    pub memory_properties: vk::VkPhysicalDeviceMemoryProperties,
    pub properties: vk::VkPhysicalDeviceProperties,
}

impl Physical {
    pub fn new(surface: &Arc<Surface>) -> Self {
        let (
            vk_data,
            graphics_queue_node_index,
            transfer_queue_node_index,
            compute_queue_node_index,
            present_queue_node_index,
        ) = Self::find_device(surface);
        let mut memory_properties = vk::VkPhysicalDeviceMemoryProperties::default();
        let mut properties = vk::VkPhysicalDeviceProperties::default();
        unsafe {
            vk::vkGetPhysicalDeviceMemoryProperties(vk_data, &mut memory_properties);
            vk::vkGetPhysicalDeviceProperties(vk_data, &mut properties);
        }
        let physical = Physical {
            surface: surface.clone(),
            graphics_queue_node_index,
            transfer_queue_node_index,
            compute_queue_node_index,
            present_queue_node_index,
            vk_data,
            memory_properties,
            properties,
        };
        physical
    }

    fn find_device(surface: &Arc<Surface>) -> (vk::VkPhysicalDevice, u32, u32, u32, u32) {
        let devices = Self::enumerate_devices(surface.instance.vk_data);
        vxlogi!("Number of physical devices is: {}", devices.len());
        for device in &devices {
            if Self::device_is_discrete(*device) {
                match Self::fetch_queues(*device, surface) {
                    Some((g, t, c, p)) => return (*device, g, t, c, p),
                    None => {}
                }
            }
        }
        for device in &devices {
            if !Self::device_is_discrete(*device) {
                match Self::fetch_queues(*device, surface) {
                    Some((g, t, c, p)) => return (*device, g, t, c, p),
                    None => {}
                }
            }
        }
        vxlogf!("Required device not found!");
    }

    fn get_properties(device: vk::VkPhysicalDevice) -> vk::VkPhysicalDeviceProperties {
        let mut properties = vk::VkPhysicalDeviceProperties::default();
        unsafe {
            vk::vkGetPhysicalDeviceProperties(device, &mut properties);
        }
        properties
    }

    fn device_is_discrete(device: vk::VkPhysicalDevice) -> bool {
        Self::get_properties(device).deviceType as u32
            == vk::VkPhysicalDeviceType::VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU as u32
    }

    fn enumerate_devices(vk_instance: vk::VkInstance) -> Vec<vk::VkPhysicalDevice> {
        let mut gpu_count = 0u32;
        vulkan_check!(vk::vkEnumeratePhysicalDevices(
            vk_instance,
            &mut gpu_count as *mut u32,
            null_mut(),
        ));
        let mut devices = vec![0 as vk::VkPhysicalDevice; gpu_count as usize];
        vulkan_check!(vk::vkEnumeratePhysicalDevices(
            vk_instance,
            &mut gpu_count,
            devices.as_mut_ptr(),
        ));
        devices
    }

    fn fetch_queues(
        device: vk::VkPhysicalDevice,
        surface: &Arc<Surface>,
    ) -> Option<(u32, u32, u32, u32)> {
        let queue_family_properties = Self::get_device_queue_family_properties(device);
        if queue_family_properties.len() == 0 {
            return None;
        }
        let mut graphics_queue_node_index = u32::max_value();
        let mut transfer_queue_node_index = u32::max_value();
        let mut compute_queue_node_index = u32::max_value();
        let mut present_queue_node_index = u32::max_value();
        let mut temp_queue_node_index = u32::max_value();

        for i in 0..(queue_family_properties.len() as u32) {
            let ref queue_family = queue_family_properties[i as usize];
            if queue_family.queueCount > 0
                && (queue_family.queueFlags as u32
                    & vk::VkQueueFlagBits::VK_QUEUE_GRAPHICS_BIT as u32) != 0
            {
                graphics_queue_node_index = i;
                break;
            }
        }

        if graphics_queue_node_index == u32::max_value() {
            return None;
        }

        for i in 0..(queue_family_properties.len() as u32) {
            let mut b = 0 as vk::VkBool32;
            unsafe {
                vk::vkGetPhysicalDeviceSurfaceSupportKHR(device, i, surface.vk_data, &mut b);
            }
            let ref queue_family = queue_family_properties[i as usize];
            if b != 0 && queue_family.queueCount > 0 {
                present_queue_node_index = i;
                break;
            }
        }

        if present_queue_node_index == u32::max_value() {
            return None;
        }

        for i in 0..(queue_family_properties.len() as u32) {
            let ref queue_family = queue_family_properties[i as usize];
            if queue_family.queueCount > 0
                && (queue_family.queueFlags as u32
                    & vk::VkQueueFlagBits::VK_QUEUE_COMPUTE_BIT as u32) != 0
            {
                if i == graphics_queue_node_index || i == present_queue_node_index {
                    temp_queue_node_index = i;
                    continue;
                } else {
                    compute_queue_node_index = i;
                    break;
                }
            }
        }

        if compute_queue_node_index == u32::max_value() {
            if temp_queue_node_index == u32::max_value() {
                return None;
            } else {
                compute_queue_node_index = temp_queue_node_index;
            }
        }

        temp_queue_node_index = u32::max_value();

        for i in 0..(queue_family_properties.len() as u32) {
            let ref queue_family = queue_family_properties[i as usize];
            if queue_family.queueCount > 0
                && (queue_family.queueFlags as u32
                    & vk::VkQueueFlagBits::VK_QUEUE_TRANSFER_BIT as u32) != 0
            {
                if i == graphics_queue_node_index || i == present_queue_node_index
                    || i == compute_queue_node_index
                {
                    temp_queue_node_index = i;
                    continue;
                } else {
                    transfer_queue_node_index = i;
                    break;
                }
            }
        }

        if transfer_queue_node_index == u32::max_value() {
            if temp_queue_node_index == u32::max_value() {
                transfer_queue_node_index = graphics_queue_node_index;
            } else {
                transfer_queue_node_index = temp_queue_node_index;
            }
        }

        return Some((
            graphics_queue_node_index,
            transfer_queue_node_index,
            compute_queue_node_index,
            present_queue_node_index,
        ));
    }

    fn get_device_queue_family_properties(
        device: vk::VkPhysicalDevice,
    ) -> Vec<vk::VkQueueFamilyProperties> {
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
                device,
                &mut count,
                queue_props.as_mut_ptr(),
            );
        }
        queue_props
    }

    pub fn get_queue_family_properties(&self) -> Vec<vk::VkQueueFamilyProperties> {
        Self::get_device_queue_family_properties(self.vk_data)
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
                vk::vkGetPhysicalDeviceFormatProperties(self.vk_data, format, &mut format_props);
            }
            if format_props.optimalTilingFeatures as u32
                & vk::VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT as u32
                != 0
            {
                return format;
            }
        }
        vxlogf!("No depth format found!");
    }

    pub fn get_surface_capabilities(&self) -> vk::VkSurfaceCapabilitiesKHR {
        let mut caps = vk::VkSurfaceCapabilitiesKHR::default();
        vulkan_check!(vk::vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
            self.vk_data,
            self.surface.vk_data,
            &mut caps,
        ));
        return caps;
    }
    pub fn get_surface_formats(&self) -> Vec<vk::VkSurfaceFormatKHR> {
        let mut count = 0u32;
        vulkan_check!(vk::vkGetPhysicalDeviceSurfaceFormatsKHR(
            self.vk_data,
            self.surface.vk_data,
            &mut count,
            null_mut(),
        ));
        let mut result = vec![vk::VkSurfaceFormatKHR::default(); count as usize];
        vulkan_check!(vk::vkGetPhysicalDeviceSurfaceFormatsKHR(
            self.vk_data,
            self.surface.vk_data,
            &mut count,
            result.as_mut_ptr(),
        ));
        result
    }
    pub fn get_memory_type_index(&self, type_bits: u32, properties: u32) -> u32 {
        // Iterate over all memory types available for the device used in this example
        let mut type_bits = type_bits;
        for i in 0..self.memory_properties.memoryTypeCount {
            if (type_bits & 1) == 1 {
                if (self.memory_properties.memoryTypes[i as usize].propertyFlags as u32)
                    & properties == properties
                {
                    return i;
                }
            }
            type_bits >>= 1;
        }
        vxlogf!("Could not find the requsted memory type.");
    }
    pub fn get_max_min_alignment(&self) -> u64 {
        let limits = &self.properties.limits;
        max(
            max(
                max(
                    limits.minMemoryMapAlignment as u64,
                    limits.minStorageBufferOffsetAlignment,
                ),
                max(
                    limits.minTexelBufferOffsetAlignment,
                    limits.minUniformBufferOffsetAlignment,
                ),
            ),
            max(
                limits.optimalBufferCopyOffsetAlignment,
                limits.optimalBufferCopyRowPitchAlignment,
            ),
        )
    }
}

impl Drop for Physical {
    fn drop(&mut self) {}
}
