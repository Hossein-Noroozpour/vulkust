use super::instance::Instance;

use super::super::system::vulkan::{
    VkQueue,
    uint32_t,
    VkDevice,
    VkResult,
    VkFormat,
    vkCreateDevice,
    VkStructureType,
    vkDestroyDevice,
    VkQueueFlagBits,
    VkPhysicalDevice,
    vkGetDeviceQueue,
    VkFormatProperties,
    VkDeviceCreateInfo,
    VkMemoryRequirements,
    VkMemoryPropertyFlags,
    VkAllocationCallbacks,
    VkDeviceQueueCreateInfo,
    VkQueueFamilyProperties,
    VkFormatFeatureFlagBits,
    vkEnumeratePhysicalDevices,
    VkPhysicalDeviceMemoryProperties,
    vkGetPhysicalDeviceFormatProperties,
    vkGetPhysicalDeviceMemoryProperties,
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
    pub vk_device: VkDevice,
    pub gpu: VkPhysicalDevice,
    pub vk_queue: VkQueue,
    pub graphics_family_index: u32,
    pub vk_mem_prop: VkPhysicalDeviceMemoryProperties,
    pub vk_depth_format: VkFormat,
}

// TODO: it need a good way to find the better physical device
// TODO: in case it was needed: device properties
// TODO: in case it was needed: feature
// TODO: in case it was needed: queue family properties
// TODO: in case it was needed: more than graphic queue, maybe compute or transfer; it must try to
//                              find dedicated queue at first but if there wasn't any dedicated

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
        let mut memory_properties = VkPhysicalDeviceMemoryProperties::default();
        unsafe {
            vkGetPhysicalDeviceMemoryProperties(
                gpu, &mut memory_properties as *mut VkPhysicalDeviceMemoryProperties);
        }
        let mut depth_format = VkFormat::VK_FORMAT_UNDEFINED;
        for format in vec![
            VkFormat::VK_FORMAT_D32_SFLOAT_S8_UINT,
            VkFormat::VK_FORMAT_D32_SFLOAT,
            VkFormat::VK_FORMAT_D24_UNORM_S8_UINT,
        ] {
            let mut format_props = VkFormatProperties::default();
            unsafe {
                vkGetPhysicalDeviceFormatProperties(
                    gpu, format, &mut format_props as *mut VkFormatProperties);
            }
            // TODO: I must be careful maybe in future there must be more necessary features
            //       for a format
            if VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT as u32 &
                format_props.optimalTilingFeatures != 0 {
                depth_format = format;
                break;
            }
        }
        if depth_format as u32 == VkFormat::VK_FORMAT_UNDEFINED as u32 {
            panic!("Depth format not found!");
        }
        Device {
            instance: instance.clone(),
            vk_device: device,
            gpu: gpu,
            vk_queue: queue,
            graphics_family_index: graphics_family_index,
            vk_mem_prop: memory_properties,
            vk_depth_format: depth_format,
        }
    }

    pub fn choose_heap_from_flags(
        &self, memory_requirements: &VkMemoryRequirements,
        required_flags: VkMemoryPropertyFlags, preferred_flags: VkMemoryPropertyFlags) -> u32 {
        for i in 0..32u32 {
            if memory_requirements.memoryTypeBits & (1 << i) != 0 {
                if (self.vk_mem_prop.memoryTypes[i as usize].propertyFlags & preferred_flags) ==
                    preferred_flags {
                    return i;
                }
            }
        }
        for i in 0..32u32 {
            if memory_requirements.memoryTypeBits & (1 << i) != 0 {
                if (self.vk_mem_prop.memoryTypes[i as usize].propertyFlags & required_flags) ==
                    required_flags {
                    return i;
                }
            }
        }
        panic!("Required memory type not found")
    }

    pub fn get_memory_type_index(type_bits: u32, properties: VkMemoryPropertyFlags) -> u32 {
        for i in 0..deviceMemoryProperties.memoryTypeCount {
            if (typeBits & 1) == 1 {
                if (deviceMemoryProperties.memoryTypes[i].propertyFlags & properties) == properties {
                    return i;
                }
            }
            typeBits >>= 1;
        }
        panic!("Could not find a suitable memory type!");
        return 0;
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            vkDestroyDevice(self.vk_device, 0 as *const VkAllocationCallbacks);
        }
    }
}