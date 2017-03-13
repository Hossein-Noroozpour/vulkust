use super::super::super::system::vulkan::{
    VkFormat,
    VkResult,
    VkImageView,
    VkStructureType,
    VkImageViewType,
    vkCreateImageView,
    vkDestroyImageView,
    VkAllocationCallbacks,
    VkImageAspectFlagBits,
    VkImageViewCreateInfo,
};

use super::Image;

use std::default::Default;
use std::sync::{
    Arc,
    RwLock,
};

pub struct View {
    pub image: Arc<RwLock<Image>>,
    pub vk_view: VkImageView,
}

impl View {
    pub fn new_depth_stencil(image: Arc<RwLock<Image>>) -> Self {
        let img = image.read().unwrap();
        let dev = img.device.read().unwrap();
        let mut depth_stencil_view = VkImageViewCreateInfo::default();
        depth_stencil_view.sType = VkStructureType::VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO;
        depth_stencil_view.viewType = VkImageViewType::VK_IMAGE_VIEW_TYPE_2D;
        depth_stencil_view.format = img.vk_format;
        depth_stencil_view.subresourceRange.aspectMask =
            VkImageAspectFlagBits::VK_IMAGE_ASPECT_DEPTH_BIT as u32 |
                VkImageAspectFlagBits::VK_IMAGE_ASPECT_STENCIL_BIT as u32;
        depth_stencil_view.subresourceRange.baseMipLevel = 0;
        depth_stencil_view.subresourceRange.levelCount = 1;
        depth_stencil_view.subresourceRange.baseArrayLayer = 0;
        depth_stencil_view.subresourceRange.layerCount = 1;
        depth_stencil_view.image = img.vk_image;
        let mut vk_view = 0 as VkImageView;
        vulkan_check!(vkCreateImageView(
            dev.vk_device, &depth_stencil_view as *const VkImageViewCreateInfo,
            0 as *const VkAllocationCallbacks, &mut vk_view as *mut VkImageView));
        View {
            image: image.clone(),
            vk_view: vk_view,
        }
    }
}

impl Drop for View {
    fn drop(&mut self) {
        let img = self.image.read().unwrap();
        let dev = img.device.read().unwrap();
        unsafe {
            vkDestroyImageView(dev.vk_device, self.vk_view, 0 as *const VkAllocationCallbacks);
        }
    }
}