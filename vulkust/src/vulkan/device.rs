use super::super::core::string::{cstrings_to_ptrs, strings_to_cstrings};
use super::super::render::config::Configurations;
use super::super::render::image::Format;
use super::surface::Surface;
use ash::vk;
use std::collections::HashSet;
use std::ptr::{null, null_mut};
use std::sync::Arc;
// use std::cmp::max;

#[cfg_attr(debug_mode, derive(Debug))]
pub(super) struct Physical {
    surface: Arc<Surface>,
    graphics_queue_node_index: u32,
    transfer_queue_node_index: u32,
    compute_queue_node_index: u32,
    present_queue_node_index: u32,
    vk_data: vk::PhysicalDevice,
    memory_properties: vk::PhysicalDeviceMemoryProperties,
    properties: vk::PhysicalDeviceProperties,
    surface_caps: vk::SurfaceCapabilitiesKHR,
    supported_depth_format: vk::Format,
}

impl Physical {
    pub(super) fn new(surface: &Arc<Surface>) -> Self {
        let (
            vk_data,
            graphics_queue_node_index,
            transfer_queue_node_index,
            compute_queue_node_index,
            present_queue_node_index,
        ) = Self::find_device(surface);
        let mut memory_properties = vk::PhysicalDeviceMemoryProperties::default();
        let mut properties = vk::PhysicalDeviceProperties::default();
        let mut surface_caps = vk::SurfaceCapabilitiesKHR::default();
        vulkan_check!(vk::vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
            vk_data,
            surface.get_data(),
            &mut surface_caps,
        ));
        unsafe {
            vk::vkGetPhysicalDeviceMemoryProperties(vk_data, &mut memory_properties);
            vk::vkGetPhysicalDeviceProperties(vk_data, &mut properties);
        }
        vxlogi!("{:?}", &surface_caps);
        let mut physical = Physical {
            surface: surface.clone(),
            graphics_queue_node_index,
            transfer_queue_node_index,
            compute_queue_node_index,
            present_queue_node_index,
            vk_data,
            memory_properties,
            properties,
            surface_caps,
            supported_depth_format: vk::VkFormat::VK_FORMAT_D32_SFLOAT,
        };
        physical.supported_depth_format = physical.get_supported_depth_format();
        return physical;
    }

    pub(super) fn get_surface(&self) -> &Surface {
        return &self.surface;
    }

    pub(super) fn get_surface_capabilities(&self) -> &vk::VkSurfaceCapabilitiesKHR {
        return &self.surface_caps;
    }

    fn find_device(surface: &Arc<Surface>) -> (vk::PhysicalDevice, u32, u32, u32, u32) {
        let devices = Self::enumerate_devices(surface.get_instance().get_data());
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

    fn device_is_discrete(device: vk::PhysicalDevice) -> bool {
        get_properties(device).deviceType as u32
            == vk::PhysicalDeviceType::VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU as u32
    }

    fn enumerate_devices(vk_instance: vk::VkInstance) -> Vec<vk::PhysicalDevice> {
        let mut gpu_count = 0u32;
        vulkan_check!(vk::vkEnumeratePhysicalDevices(
            vk_instance,
            &mut gpu_count as *mut u32,
            null_mut(),
        ));
        let mut devices = vec![0 as vk::PhysicalDevice; gpu_count as usize];
        vulkan_check!(vk::vkEnumeratePhysicalDevices(
            vk_instance,
            &mut gpu_count,
            devices.as_mut_ptr(),
        ));
        devices
    }

    fn fetch_queues(
        device: vk::PhysicalDevice,
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

        for i in 0..(queue_family_properties.len() as u32) {
            let ref queue_family = queue_family_properties[i as usize];
            let mut b = 0 as vk::VkBool32;
            unsafe {
                vk::vkGetPhysicalDeviceSurfaceSupportKHR(device, i, surface.get_data(), &mut b);
            }
            if queue_family.queueCount > 0
                && b != 0
                && (queue_family.queueFlags as u32
                    & vk::VkQueueFlagBits::VK_QUEUE_GRAPHICS_BIT as u32)
                    != 0
                && (queue_family.queueFlags as u32
                    & vk::VkQueueFlagBits::VK_QUEUE_COMPUTE_BIT as u32)
                    != 0
                && (queue_family.queueFlags as u32
                    & vk::VkQueueFlagBits::VK_QUEUE_TRANSFER_BIT as u32)
                    != 0
            {
                return Some((i, i, i, i));
            }
        }

        for i in 0..(queue_family_properties.len() as u32) {
            let ref queue_family = queue_family_properties[i as usize];
            let mut b = 0 as vk::VkBool32;
            unsafe {
                vk::vkGetPhysicalDeviceSurfaceSupportKHR(device, i, surface.get_data(), &mut b);
            }
            if queue_family.queueCount > 0
                && b != 0
                && (queue_family.queueFlags as u32
                    & vk::VkQueueFlagBits::VK_QUEUE_GRAPHICS_BIT as u32)
                    != 0
                && (queue_family.queueFlags as u32
                    & vk::VkQueueFlagBits::VK_QUEUE_COMPUTE_BIT as u32)
                    != 0
            {
                graphics_queue_node_index = i;
                compute_queue_node_index = i;
                present_queue_node_index = i;
            }
            if queue_family.queueCount > 0
                && (queue_family.queueFlags as u32
                    & vk::VkQueueFlagBits::VK_QUEUE_TRANSFER_BIT as u32)
                    != 0
            {
                transfer_queue_node_index = i;
            }
        }

        if graphics_queue_node_index == u32::max_value() {
            return None;
        }

        if transfer_queue_node_index == u32::max_value() {
            transfer_queue_node_index = graphics_queue_node_index;
        }

        return Some((
            graphics_queue_node_index,
            transfer_queue_node_index,
            compute_queue_node_index,
            present_queue_node_index,
        ));
    }

    fn get_device_queue_family_properties(
        device: vk::PhysicalDevice,
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

    // pub(super) fn get_queue_family_properties(&self) -> Vec<vk::VkQueueFamilyProperties> {
    //     Self::get_device_queue_family_properties(self.vk_data)
    // }

    fn get_supported_depth_format(&self) -> vk::VkFormat {
        let depth_formats = vec![
            vk::VkFormat::VK_FORMAT_D32_SFLOAT,
            vk::VkFormat::VK_FORMAT_D32_SFLOAT_S8_UINT,
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

    pub(super) fn get_surface_formats(&self) -> Vec<vk::VkSurfaceFormatKHR> {
        let mut count = 0u32;
        vulkan_check!(vk::vkGetPhysicalDeviceSurfaceFormatsKHR(
            self.vk_data,
            self.surface.get_data(),
            &mut count,
            null_mut(),
        ));
        let mut result = vec![vk::VkSurfaceFormatKHR::default(); count as usize];
        vulkan_check!(vk::vkGetPhysicalDeviceSurfaceFormatsKHR(
            self.vk_data,
            self.surface.get_data(),
            &mut count,
            result.as_mut_ptr(),
        ));
        result
    }

    pub(super) fn get_memory_type_index(&self, type_bits: u32, properties: u32) -> u32 {
        // Iterate over all memory types available for the device used in this example
        let mut type_bits = type_bits;
        for i in 0..self.memory_properties.memoryTypeCount {
            if (type_bits & 1) == 1 {
                if (self.memory_properties.memoryTypes[i as usize].propertyFlags as u32)
                    & properties
                    == properties
                {
                    return i;
                }
            }
            type_bits >>= 1;
        }
        vxlogf!("Could not find the requsted memory type.");
    }

    // pub(super) fn get_max_min_alignment(&self) -> u64 {
    //     let limits = &self.properties.limits;
    //     max(
    //         max(
    //             max(
    //                 limits.minMemoryMapAlignment as u64,
    //                 limits.minStorageBufferOffsetAlignment,
    //             ),
    //             max(
    //                 limits.minTexelBufferOffsetAlignment,
    //                 limits.minUniformBufferOffsetAlignment,
    //             ),
    //         ),
    //         max(
    //             max(
    //                 limits.optimalBufferCopyOffsetAlignment,
    //                 limits.optimalBufferCopyRowPitchAlignment,
    //             ),
    //             limits.bufferImageGranularity,
    //         ),
    //     )
    // }

    pub(super) fn get_vk_features(&self) -> vk::PhysicalDeviceFeatures {
        let mut result = vk::PhysicalDeviceFeatures::default();
        unsafe {
            vk::vkGetPhysicalDeviceFeatures(self.vk_data, &mut result);
        }
        return result;
    }

    pub(super) fn get_graphics_queue_node_index(&self) -> u32 {
        return self.graphics_queue_node_index;
    }

    pub(super) fn get_transfer_queue_node_index(&self) -> u32 {
        return self.transfer_queue_node_index;
    }

    pub(super) fn get_compute_queue_node_index(&self) -> u32 {
        return self.compute_queue_node_index;
    }

    pub(super) fn get_present_queue_node_index(&self) -> u32 {
        return self.present_queue_node_index;
    }

    pub(super) fn get_data(&self) -> vk::PhysicalDevice {
        return self.vk_data;
    }

    pub(super) fn get_properties(&self) -> &vk::PhysicalDeviceProperties {
        return &self.properties;
    }

    pub(super) fn convert_format(&self, f: Format) -> vk::VkFormat {
        match f {
            Format::RgbaFloat => return vk::VkFormat::VK_FORMAT_R32G32B32A32_SFLOAT,
            Format::DepthFloat => return self.get_supported_depth_format(),
            Format::Float => return vk::VkFormat::VK_FORMAT_R32_SFLOAT,
            Format::FlagBits8 => return vk::VkFormat::VK_FORMAT_R8_UNORM,
            Format::FlagBits64 => return vk::VkFormat::VK_FORMAT_R32G32_UINT,
            _ => vxunexpected!(),
        }
    }
}

impl Drop for Physical {
    fn drop(&mut self) {}
}

unsafe impl Send for Physical {}

unsafe impl Sync for Physical {}

fn get_properties(device: vk::PhysicalDevice) -> vk::PhysicalDeviceProperties {
    let mut properties = vk::PhysicalDeviceProperties::default();
    unsafe {
        vk::vkGetPhysicalDeviceProperties(device, &mut properties);
    }
    properties
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Logical {
    physical_device: Arc<Physical>,
    vk_data: vk::VkDevice,
    vk_graphic_queue: vk::VkQueue,
    vk_compute_queue: vk::VkQueue,
    vk_present_queue: vk::VkQueue,
}

impl Logical {
    pub(super) fn new(physical_device: &Arc<Physical>, config: &Configurations) -> Self {
        let mut device_extensions = Vec::new();
        device_extensions.push("VK_KHR_swapchain".to_string());
        let device_extensions = strings_to_cstrings(device_extensions);
        let device_extensions = cstrings_to_ptrs(&device_extensions);
        let mut queue_family_index_set = HashSet::new();
        queue_family_index_set.insert(physical_device.get_graphics_queue_node_index());
        queue_family_index_set.insert(physical_device.get_transfer_queue_node_index());
        queue_family_index_set.insert(physical_device.get_compute_queue_node_index());
        queue_family_index_set.insert(physical_device.get_present_queue_node_index());
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
        let available_features = physical_device.get_vk_features();
        let mut features = vk::PhysicalDeviceFeatures::default();
        if config.get_enable_anistropic_texture() {
            features.samplerAnisotropy = available_features.samplerAnisotropy;
        }
        let mut device_create_info = vk::VkDeviceCreateInfo::default();
        device_create_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO;
        device_create_info.queueCreateInfoCount = queue_create_info_s.len() as u32;
        device_create_info.pQueueCreateInfos = queue_create_info_s.as_ptr();
        device_create_info.enabledExtensionCount = device_extensions.len() as u32;
        device_create_info.ppEnabledExtensionNames = device_extensions.as_ptr();
        device_create_info.pEnabledFeatures = &features;
        let mut vk_data = 0 as vk::VkDevice;
        vulkan_check!(vk::vkCreateDevice(
            physical_device.get_data(),
            &device_create_info,
            null(),
            &mut vk_data,
        ));
        let mut vk_graphic_queue = 0 as vk::VkQueue;
        unsafe {
            vk::vkGetDeviceQueue(
                vk_data,
                physical_device.get_graphics_queue_node_index(),
                0,
                &mut vk_graphic_queue,
            );
        }
        let mut vk_compute_queue = 0 as vk::VkQueue;
        unsafe {
            vk::vkGetDeviceQueue(
                vk_data,
                physical_device.get_compute_queue_node_index(),
                0,
                &mut vk_compute_queue,
            );
        }
        let mut vk_present_queue = 0 as vk::VkQueue;
        unsafe {
            vk::vkGetDeviceQueue(
                vk_data,
                physical_device.get_present_queue_node_index(),
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

    pub(crate) fn get_uniform_buffer_alignment(&self) -> isize {
        self.physical_device
            .get_properties()
            .limits
            .minUniformBufferOffsetAlignment as isize
    }

    pub(super) fn wait_idle(&self) {
        unsafe {
            vk::vkDeviceWaitIdle(self.vk_data);
        }
    }

    pub(super) fn get_data(&self) -> vk::VkDevice {
        return self.vk_data;
    }

    pub(super) fn get_physical(&self) -> &Physical {
        return &self.physical_device;
    }

    pub(super) fn get_vk_graphic_queue(&self) -> vk::VkQueue {
        return self.vk_graphic_queue;
    }

    pub(super) fn convert_format(&self, f: Format) -> vk::VkFormat {
        return self.physical_device.convert_format(f);
    }

    // pub(super) fn get_vk_compute_queue(&self) -> vk::VkQueue {
    //     return self.vk_compute_queue;
    // }

    // pub(super) fn get_vk_present_queue(&self) -> vk::VkQueue {
    //     return self.vk_present_queue;
    // }
}

impl Drop for Logical {
    fn drop(&mut self) {
        unsafe {
            vk::vkDestroyDevice(self.vk_data, null());
        }
    }
}

unsafe impl Send for Logical {}

unsafe impl Sync for Logical {}
