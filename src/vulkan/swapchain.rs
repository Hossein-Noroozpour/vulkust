use super::super::system::vulkan::{
    VkSwapchainCreateInfoKHR,
    VkImageUsageFlagBits,
    VkStructureType,
    VkSharingMode,
};

use super::surface::Surface;

pub struct Swapchain {

}

impl Swapchain {
    pub fn new(surface: &Surface, width: u32, height: u32) -> Self {
        let swapchain_create_info = VkSwapchainCreateInfoKHR {
            sType: VkStructureType::VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
            surface: surface.vk_surface,
            minImageCount: 2,
            imageFormat: surface.format,
            imageColorSpace: surface.color_space,
            width: width,
            height: height,
            imageArrayLayers: 1,
            imageUsage: VkImageUsageFlagBits::VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT,
            imageSharingMode: VkSharingMode::VK_SHARING_MODE_EXCLUSIVE,
        };
        swapchain_create_info.queueFamilyIndexCount		= 0;
        swapchain_create_info.pQueueFamilyIndices		= nullptr;
        swapchain_create_info.preTransform				= VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR;
        swapchain_create_info.compositeAlpha			= VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR;
        swapchain_create_info.presentMode				= present_mode;
        swapchain_create_info.clipped					= VK_TRUE;
        swapchain_create_info.oldSwapchain				= VK_NULL_HANDLE;

        ErrorCheck( vkCreateSwapchainKHR( _renderer->GetVulkanDevice(), &swapchain_create_info, nullptr, &_swapchain ) );

        ErrorCheck( vkGetSwapchainImagesKHR( _renderer->GetVulkanDevice(), _swapchain, &_swapchain_image_count, nullptr ) );
    }
    }
}