use super::super::system::vulkan::{
    VkSharingMode,
    VkStructureType,
    VkPresentModeKHR,
    VkImageUsageFlagBits,
    VkSwapchainCreateInfoKHR,
};

use super::window::Window;

use std::sync::{
    Arc,
    RwLock,
};

pub struct Swapchain {
    window: Arc<RwLock<Window>>,
    vk_image_views: Vec<VkImageView>,
    vk_swapchain: VkSwapchainKHR,
}

impl Swapchain {
    pub fn new(window: Arc<RwLock<Window>>, width: u32, height: u32) -> Self {
        let win = window.read().unwrap();
        let dev = window.device.read().unwrap();
        if dev.vk_surface_capabilities.maxImageCount < 2 {
            panic!("Vulkan driver does not support 2 double image buffering.");
        }
        let pre_transform = if (dev.vk_surface_capabilities.supportedTransforms as u32) &
            (VkSurfaceTransformFlagBitsKHR::VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR as u32) {
            VkSurfaceTransformFlagBitsKHR::VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR
        } else {
            dev.vk_surface_capabilities.currentTransform
        };

        let swapchain_ci = VkSwapchainCreateInfoKHR {
            sType: VkStructureType::VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
            surface: dev.vk_surface,
            minImageCount: 2,
            imageFormat: dev.vk_suraface_format.format,
            imageColorSpace: dev.vk_suraface_format.colorSpace,
            imageExtent: dev.vk_surface_capabilities.currentExtent,
            imageUsage: VkImageUsageFlagBits::VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT,
            preTransform: pre_transform,
            imageArrayLayers: 1,
            imageSharingMode: VkSharingMode::VK_SHARING_MODE_EXCLUSIVE,
            queueFamilyIndexCount: 0,
            presentMode: VkPresentModeKHR::VK_PRESENT_MODE_FIFO_KHR,
            clipped: 1u32,
            compositeAlpha: VkCompositeAlphaFlagBitsKHR::VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
            ..VkSwapchainCreateInfoKHR::default()
        };
        let mut vk_swapchain = 0 as VkSwapChain;
        vulkan_check!(vkCreateSwapchainKHR(
            dev.vk_device, &swapchain_ci as *const VkSwapchainCreateInfoKHR,
            0 as *const VkAllocationCallbacks, &mut vk_swapchain as *mut VkSwapchainKHR));
        let mut image_count = 0u32;
        vulkan_check!(vkGetSwapchainImagesKHR(
            dev.vk_device, vk_swapchain, &mut image_count as *mut u32, 0 as *mut VkImage));
        let mut vk_images = vec![VkImage::default(); image_count as usize];
        vulkan_check!(vkGetSwapchainImagesKHR(
            dev.vk_device, vk_swapchain, &mut image_count as *mut u32,
            vk_images.as_mut_ptr() as *mut VkImage));
        let mut vk_image_views = vec![VkImageView::default(); image_count as usize];
        for i in 0..image_count {
            let color_attachment_view = VkImageViewCreateInfo {
                sType: VkStructureType::VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
                image: vk_images,
                viewType: VkImageViewType::VK_IMAGE_VIEW_TYPE_2D,
                format: dev.vk_suraface_format.format,
                components: VkComponentMapping {
                    r: VkComponentSwizzle::VK_COMPONENT_SWIZZLE_R,
                    g: VkComponentSwizzle::VK_COMPONENT_SWIZZLE_G,
                    b: VkComponentSwizzle::VK_COMPONENT_SWIZZLE_B,
                    a: VkComponentSwizzle::VK_COMPONENT_SWIZZLE_A,
                },
                subresourceRange: VkImageSubresourceRange {
                    aspectMask: VkImageAspectFlags::VK_IMAGE_ASPECT_COLOR_BIT,
                    baseMipLevel: 0,
                    levelCount: 1,
                    baseArrayLayer: 0,
                    layerCount: 1,
                },
                ..VkImageViewCreateInfo::default()
            };
            let ptr_vk_image_views = vk_image_views.as_mut_ptr();
            vulkan_check!(vkCreateImageView(
                dev.vk_device, &color_attachment_view as *const VkImageViewCreateInfo,
                0 as *const VkAllocationCallbacks, ptr_vk_image_views.offset(i)));
        }
        Swapchain {
            window: window.clone(),
            vk_image_views: vk_image_views,
            vk_swapchain: vk_swapchain,
        }
    }
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        let win = window.read().unwrap();
        let dev = window.device.read().unwrap();
        for v in self.vk_image_views {
            vkDestroyImageView(dev.vk_device, v, 0 as *const VkAllocationCallbacks);
        }
        vkDestroySwapchainKHR(dev.vk_device, self.vk_swapchain, 0 as *const VkAllocationCallbacks);
    }
}