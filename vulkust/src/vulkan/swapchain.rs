use super::device::logical::Logical as LogicalDevice;
use super::image::view::View as ImageView;
use super::synchronizer::semaphore::Semaphore;
use super::vulkan as vk;
use std::ptr::{null, null_mut};
use std::sync::Arc;

pub enum NextImageResult {
    NeedsRefresh,
    Next(u32),
}

pub struct Swapchain {
    pub logical_device: Arc<LogicalDevice>,
    pub surface_format: vk::VkSurfaceFormatKHR,
    pub image_views: Vec<Arc<ImageView>>,
    pub vk_data: vk::VkSwapchainKHR,
}

impl Swapchain {
    pub fn new(logical_device: &Arc<LogicalDevice>) -> Self {
        let surface_caps = logical_device.physical_device.get_surface_capabilities();
        let surface_formats = logical_device.physical_device.get_surface_formats();
        let mut best_surface_format = vk::VkSurfaceFormatKHR::default();
        for format in &surface_formats {
            if format.format as u32 == vk::VkFormat::VK_FORMAT_R8G8B8A8_UNORM as u32 {
                best_surface_format = *format;
                break;
            }
        }
        for format in &surface_formats {
            if format.format as u32 == vk::VkFormat::VK_FORMAT_B8G8R8A8_UNORM as u32 {
                best_surface_format = *format;
                break;
            }
        }
        if best_surface_format.format as u32 != vk::VkFormat::VK_FORMAT_R8G8B8A8_UNORM as u32
            && best_surface_format.format as u32 != vk::VkFormat::VK_FORMAT_B8G8R8A8_UNORM as u32
        {
            vxlogi!("VK_FORMAT_R8G8B8A8_UNORM not found in the surface.");
            best_surface_format = surface_formats[0];
            vxlogi!("The specified format is {:?}", best_surface_format);
        }
        let mut swapchain_images_count = surface_caps.minImageCount + 1;
        if surface_caps.maxImageCount > 0 && swapchain_images_count > surface_caps.maxImageCount {
            swapchain_images_count = surface_caps.maxImageCount;
        }
        vxlogi!("Swapchain images count: {:?}", swapchain_images_count);
        let mut image_usage = vk::VkImageUsageFlagBits::VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT as u32;
        let mut format_props = vk::VkFormatProperties::default();
        unsafe {
            vk::vkGetPhysicalDeviceFormatProperties(
                logical_device.physical_device.vk_data,
                best_surface_format.format,
                &mut format_props,
            );
        };
        if ((format_props.optimalTilingFeatures as u32)
            & (vk::VkFormatFeatureFlagBits::VK_FORMAT_FEATURE_BLIT_DST_BIT as u32)) != 0
        {
            image_usage |= vk::VkImageUsageFlagBits::VK_IMAGE_USAGE_TRANSFER_SRC_BIT as u32;
        }
        let queue_family_indices = [
            logical_device.physical_device.graphics_queue_node_index,
            logical_device.physical_device.present_queue_node_index,
        ];
        let mut swapchain_create_info = vk::VkSwapchainCreateInfoKHR::default();
        swapchain_create_info.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR;
        swapchain_create_info.surface = logical_device.physical_device.surface.vk_data;
        swapchain_create_info.minImageCount = swapchain_images_count;
        swapchain_create_info.imageFormat = best_surface_format.format;
        swapchain_create_info.imageColorSpace = best_surface_format.colorSpace;
        swapchain_create_info.imageExtent = surface_caps.currentExtent;
        swapchain_create_info.imageUsage = image_usage;
        swapchain_create_info.preTransform = surface_caps.currentTransform;
        swapchain_create_info.imageArrayLayers = 1;
        if queue_family_indices[0] != queue_family_indices[1] {
            swapchain_create_info.imageSharingMode = vk::VkSharingMode::VK_SHARING_MODE_CONCURRENT;
            swapchain_create_info.queueFamilyIndexCount = 2;
            swapchain_create_info.pQueueFamilyIndices = queue_family_indices.as_ptr();
        } else {
            swapchain_create_info.imageSharingMode = vk::VkSharingMode::VK_SHARING_MODE_EXCLUSIVE;
        }
        swapchain_create_info.presentMode = vk::VkPresentModeKHR::VK_PRESENT_MODE_FIFO_KHR;
        swapchain_create_info.clipped = 1 as vk::VkBool32;
        if ((surface_caps.supportedCompositeAlpha as u32)
            & (vk::VkCompositeAlphaFlagBitsKHR::VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR as u32))
            != 0
        {
            swapchain_create_info.compositeAlpha =
                vk::VkCompositeAlphaFlagBitsKHR::VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR;
        } else if ((surface_caps.supportedCompositeAlpha as u32)
            & (vk::VkCompositeAlphaFlagBitsKHR::VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR as u32))
            != 0
        {
            swapchain_create_info.compositeAlpha =
                vk::VkCompositeAlphaFlagBitsKHR::VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR;
        } else if ((surface_caps.supportedCompositeAlpha as u32)
            & (vk::VkCompositeAlphaFlagBitsKHR::VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR as u32))
            != 0
        {
            swapchain_create_info.compositeAlpha =
                vk::VkCompositeAlphaFlagBitsKHR::VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR;
        } else if ((surface_caps.supportedCompositeAlpha as u32)
            & (vk::VkCompositeAlphaFlagBitsKHR::VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR as u32))
            != 0
        {
            swapchain_create_info.compositeAlpha =
                vk::VkCompositeAlphaFlagBitsKHR::VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR;
        } else if ((surface_caps.supportedCompositeAlpha as u32)
            & (vk::VkCompositeAlphaFlagBitsKHR::VK_COMPOSITE_ALPHA_FLAG_BITS_MAX_ENUM_KHR as u32))
            != 0
        {
            swapchain_create_info.compositeAlpha =
                vk::VkCompositeAlphaFlagBitsKHR::VK_COMPOSITE_ALPHA_FLAG_BITS_MAX_ENUM_KHR;
        } else {
            vxlogf!("Error composite is unknown.");
        }
        let mut vk_data = 0 as vk::VkSwapchainKHR;
        vulkan_check!(vk::vkCreateSwapchainKHR(
            logical_device.vk_data,
            &swapchain_create_info,
            null(),
            &mut vk_data,
        ));
        let mut count = 0u32;
        vulkan_check!(vk::vkGetSwapchainImagesKHR(
            logical_device.vk_data,
            vk_data,
            &mut count,
            null_mut(),
        ));
        let mut images = vec![0 as vk::VkImage; count as usize];
        vulkan_check!(vk::vkGetSwapchainImagesKHR(
            logical_device.vk_data,
            vk_data,
            &mut count,
            images.as_mut_ptr(),
        ));
        let mut views = Vec::new();
        for i in 0..(count as usize) {
            views.push(Arc::new(ImageView::new_with_vk_image(
                logical_device.clone(),
                images[i],
                best_surface_format.format,
            )));
        }
        Swapchain {
            logical_device: logical_device.clone(),
            surface_format: best_surface_format,
            image_views: views,
            vk_data: vk_data,
        }
    }

    pub fn get_next_image_index(&self, sem: &Semaphore) -> NextImageResult {
        let mut image_index = 0u32;
        let res = unsafe {
            vk::vkAcquireNextImageKHR(
                self.logical_device.vk_data,
                self.vk_data,
                u64::max_value(),
                sem.vk_data,
                0 as vk::VkFence,
                &mut image_index,
            )
        };
        match res {
            vk::VkResult::VK_ERROR_OUT_OF_DATE_KHR | vk::VkResult::VK_SUBOPTIMAL_KHR => {
                return NextImageResult::NeedsRefresh;
            }
            res @ _ => {
                vulkan_check!(res);
            }
        }
        return NextImageResult::Next(image_index);
    }
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        self.image_views.clear();
        unsafe {
            vk::vkDestroySwapchainKHR(self.logical_device.vk_data, self.vk_data, null());
        }
    }
}
