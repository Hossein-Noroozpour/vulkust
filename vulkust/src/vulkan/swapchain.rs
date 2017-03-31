use super::super::system::vulkan as vk;
use super::device::logical::Logical as LogicalDevice;
use std::ptr::null;
//use super::image::view::View as ImageView;
//use super::image::Image;
use std::sync::{
    Arc,
};

pub struct Swapchain {
    pub logical_device: Arc<LogicalDevice>,
    pub surface_format: vk::VkSurfaceFormatKHR,
    pub vk_data: vk::VkSwapchainKHR,
}

impl Swapchain {
    pub fn new(logical_device: Arc<LogicalDevice>) -> Self {
        let surface_caps = logical_device.physical_device.get_surface_capabilities();
        let surface_formats = logical_device.physical_device.get_surface_formats();
        let mut best_surface_format = vk::VkSurfaceFormatKHR::default();
        for format in surface_formats.clone() {
            if format.format as u32 == vk::VkFormat::VK_FORMAT_R8G8B8A8_UNORM as u32 {
                best_surface_format = format;
                break;
            }
        }
        if best_surface_format.format as u32 != vk::VkFormat::VK_FORMAT_R8G8B8A8_UNORM as u32 {
            logi!("VK_FORMAT_R8G8B8A8_UNORM not found in the surface.");
            best_surface_format = surface_formats[0];
            if best_surface_format.format as u32 == vk::VkFormat::VK_FORMAT_UNDEFINED as u32 {
                best_surface_format.format = vk::VkFormat::VK_FORMAT_R8G8B8A8_UNORM;
            }
        }
        let mut swapchain_images_count = surface_caps.minImageCount + 1;
        if surface_caps.maxImageCount > 0 && swapchain_images_count > surface_caps.maxImageCount {
            swapchain_images_count = surface_caps.maxImageCount;
        }
        let mut image_usage = vk::VkImageUsageFlagBits::VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT as u32;
        let mut format_props = vk::VkFormatProperties::default();
        unsafe {
            vk::vkGetPhysicalDeviceFormatProperties(
                logical_device.physical_device.vk_data, best_surface_format.format,
                &mut format_props);
        };
        if ((format_props.optimalTilingFeatures as u32) &
            (vk::VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT as u32)) != 0  {
            image_usage |= vk::VkImageUsageFlagBits::VK_IMAGE_USAGE_TRANSFER_SRC_BIT as u32;
        }
        let mut swapchain_create_info = vk::VkSwapchainCreateInfoKHR::default();
        swapchain_create_info.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR;
        swapchain_create_info.surface = logical_device.physical_device.surface.vk_data;
        swapchain_create_info.minImageCount = swapchain_images_count;
        swapchain_create_info.imageFormat = best_surface_format.format;
        swapchain_create_info.imageColorSpace = best_surface_format.colorSpace;
        swapchain_create_info.imageExtent = surface_caps.currentExtent;
        swapchain_create_info.imageUsage = image_usage;
        swapchain_create_info.preTransform =
            vk::VkSurfaceTransformFlagBitsKHR::VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR;
        swapchain_create_info.imageArrayLayers = 1;
        swapchain_create_info.imageSharingMode = vk::VkSharingMode::VK_SHARING_MODE_EXCLUSIVE;
        swapchain_create_info.presentMode = vk::VkPresentModeKHR::VK_PRESENT_MODE_FIFO_KHR;
        swapchain_create_info.clipped = 1 as vk::VkBool32;
        if ((surface_caps.supportedCompositeAlpha as u32) &
            (vk::VkCompositeAlphaFlagBitsKHR::VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR as u32)) != 0 {
            swapchain_create_info.compositeAlpha =
                vk::VkCompositeAlphaFlagBitsKHR::VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR;
        } else if ((surface_caps.supportedCompositeAlpha as u32) &
            (vk::VkCompositeAlphaFlagBitsKHR::VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR as u32)) != 0 {
            swapchain_create_info.compositeAlpha =
                vk::VkCompositeAlphaFlagBitsKHR::VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR;
        } else if ((surface_caps.supportedCompositeAlpha as u32) &
            (vk::VkCompositeAlphaFlagBitsKHR::VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR as u32))
            != 0 {
            swapchain_create_info.compositeAlpha =
                vk::VkCompositeAlphaFlagBitsKHR::VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR;
        } else if ((surface_caps.supportedCompositeAlpha as u32) &
            (vk::VkCompositeAlphaFlagBitsKHR::VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR as u32))
            != 0 {
            swapchain_create_info.compositeAlpha =
                vk::VkCompositeAlphaFlagBitsKHR::VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR;
        } else if ((surface_caps.supportedCompositeAlpha as u32) &
            (vk::VkCompositeAlphaFlagBitsKHR::VK_COMPOSITE_ALPHA_FLAG_BITS_MAX_ENUM_KHR as u32))
            != 0 {
            swapchain_create_info.compositeAlpha =
                vk::VkCompositeAlphaFlagBitsKHR::VK_COMPOSITE_ALPHA_FLAG_BITS_MAX_ENUM_KHR;
        } else {
            logf!("Error composite is unknown.");
        }
        let mut vk_data = 0 as vk::VkSwapchainKHR;
        vulkan_check!(vk::vkCreateSwapchainKHR(
            logical_device.vk_data, &swapchain_create_info, null(), &mut vk_data));
        Swapchain {
            logical_device: logical_device,
            surface_format: best_surface_format,
            vk_data: vk_data,
        }
    }
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        unsafe {
            vk::vkDestroySwapchainKHR(
                self.logical_device.vk_data, self.vk_data, null());
        }
    }
}