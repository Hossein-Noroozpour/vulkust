use super::super::system::vulkan::{
    VkImage,
    VkResult,
    VkImageView,
    VkSharingMode,
    VkSwapchainKHR,
    VkImageViewType,
    VkStructureType,
    VkPresentModeKHR,
    vkCreateImageView,
    vkDestroyImageView,
    VkImageAspectFlags,
    VkComponentSwizzle,
    VkComponentMapping,
    vkCreateSwapchainKHR,
    VkImageUsageFlagBits,
    VkImageAspectFlagBits,
    VkImageViewCreateInfo,
    vkDestroySwapchainKHR,
    VkAllocationCallbacks,
    VkImageSubresourceRange,
    vkGetSwapchainImagesKHR,
    VkSwapchainCreateInfoKHR,
    VkCompositeAlphaFlagBitsKHR,
    VkSurfaceTransformFlagBitsKHR,
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
        let dev = win.device.read().unwrap();
        if win.vk_surface_capabilities.maxImageCount < 2 {
            panic!("Vulkan driver does not support 2 double image buffering.");
        }
        let pre_transform = if (win.vk_surface_capabilities.supportedTransforms as u32) &
            (VkSurfaceTransformFlagBitsKHR::VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR as u32) != 0 {
            VkSurfaceTransformFlagBitsKHR::VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR
        } else {
            win.vk_surface_capabilities.currentTransform
        };

        let swapchain_ci = VkSwapchainCreateInfoKHR {
            sType: VkStructureType::VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
            surface: win.vk_surface,
            minImageCount: 2,
            imageFormat: win.vk_surface_format.format,
            imageColorSpace: win.vk_surface_format.colorSpace,
            imageExtent: win.vk_surface_capabilities.currentExtent,
            imageUsage: VkImageUsageFlagBits::VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT as u32,
            preTransform: pre_transform,
            imageArrayLayers: 1,
            imageSharingMode: VkSharingMode::VK_SHARING_MODE_EXCLUSIVE,
            queueFamilyIndexCount: 0,
            presentMode: VkPresentModeKHR::VK_PRESENT_MODE_FIFO_KHR,
            clipped: 1u32,
            compositeAlpha: VkCompositeAlphaFlagBitsKHR::VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
            ..VkSwapchainCreateInfoKHR::default()
        };
        let mut vk_swapchain = 0 as VkSwapchainKHR;
        vulkan_check!(vkCreateSwapchainKHR(
            dev.vk_device, &swapchain_ci as *const VkSwapchainCreateInfoKHR,
            0 as *const VkAllocationCallbacks, &mut vk_swapchain as *mut VkSwapchainKHR));
        let mut image_count = 0u32;
        vulkan_check!(vkGetSwapchainImagesKHR(
            dev.vk_device, vk_swapchain, &mut image_count as *mut u32, 0 as *mut VkImage));
        let mut vk_images = vec![0 as VkImage; image_count as usize];
        vulkan_check!(vkGetSwapchainImagesKHR(
            dev.vk_device, vk_swapchain, &mut image_count as *mut u32,
            vk_images.as_mut_ptr() as *mut VkImage));
        let mut vk_image_views = vec![0 as VkImageView; image_count as usize];
        for i in 0..image_count {
            let mut color_attachment_view = VkImageViewCreateInfo::default();
            color_attachment_view.sType = VkStructureType::VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO;
            color_attachment_view.image = vk_images[i as usize];
            color_attachment_view.viewType = VkImageViewType::VK_IMAGE_VIEW_TYPE_2D;
            color_attachment_view.format = win.vk_surface_format.format;
            color_attachment_view.components = VkComponentMapping {
                r: VkComponentSwizzle::VK_COMPONENT_SWIZZLE_R,
                g: VkComponentSwizzle::VK_COMPONENT_SWIZZLE_G,
                b: VkComponentSwizzle::VK_COMPONENT_SWIZZLE_B,
                a: VkComponentSwizzle::VK_COMPONENT_SWIZZLE_A,
            };
            color_attachment_view.subresourceRange = VkImageSubresourceRange {
                aspectMask: VkImageAspectFlagBits::VK_IMAGE_ASPECT_COLOR_BIT as u32,
                baseMipLevel: 0,
                levelCount: 1,
                baseArrayLayer: 0,
                layerCount: 1,
            };
            let ptr_vk_image_views = vk_image_views.as_mut_ptr();
            vulkan_check!(vkCreateImageView(
                dev.vk_device, &color_attachment_view as *const VkImageViewCreateInfo,
                0 as *const VkAllocationCallbacks, ptr_vk_image_views.offset(i as isize)));
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
        let win = self.window.read().unwrap();
        let dev = win.device.read().unwrap();
        for i in 0..self.vk_image_views.len() {
            unsafe {
                vkDestroyImageView(
                    dev.vk_device, self.vk_image_views[i], 0 as *const VkAllocationCallbacks);
            }
        }
        unsafe {
            vkDestroySwapchainKHR(
                dev.vk_device, self.vk_swapchain, 0 as *const VkAllocationCallbacks);
        }
    }
}