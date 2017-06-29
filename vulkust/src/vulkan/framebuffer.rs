use super::image::view::View;
use super::render_pass::RenderPass;
use super::super::system::vulkan as vk;

use std::ptr::null;
use std::sync::Arc;
use std::default::Default;

pub struct Framebuffer {
    pub color_buffer: Arc<View>,
    pub depth_buffer: Arc<View>,
    pub render_pass: Arc<RenderPass>,
    pub vk_data: vk::VkFramebuffer,
}

impl Framebuffer {
    pub fn new(
        color_buffer: Arc<View>,
        depth_buffer: Arc<View>,
        render_pass: Arc<RenderPass>,
    ) -> Self {
        let attachments = vec![color_buffer.vk_data, depth_buffer.vk_data];
        let surface_caps = color_buffer
            .image
            .logical_device
            .physical_device
            .get_surface_capabilities();
        let mut fb_create_info = vk::VkFramebufferCreateInfo::default();
        fb_create_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO;
        fb_create_info.renderPass = render_pass.vk_data;
        fb_create_info.layers = 1;
        fb_create_info.attachmentCount = 2;
        fb_create_info.pAttachments = attachments.as_ptr();
        fb_create_info.width = surface_caps.currentExtent.width;
        fb_create_info.height = surface_caps.currentExtent.height;
        let mut vk_data = 0 as vk::VkFramebuffer;
        vulkan_check!(vk::vkCreateFramebuffer(
            color_buffer.image.logical_device.vk_data,
            &fb_create_info,
            null(),
            &mut vk_data
        ));
        Framebuffer {
            color_buffer: color_buffer,
            depth_buffer: depth_buffer,
            render_pass: render_pass,
            vk_data: vk_data,
        }
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            vk::vkDestroyFramebuffer(
                self.color_buffer.image.logical_device.vk_data,
                self.vk_data,
                null(),
            );
        }
    }
}
