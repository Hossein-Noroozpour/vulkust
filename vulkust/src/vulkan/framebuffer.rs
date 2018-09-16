use super::image::View;
use super::render_pass::RenderPass;
use super::vulkan as vk;

use std::ptr::null;
use std::sync::Arc;

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Framebuffer {
    pub color_buffer: Vec<Arc<View>>,
    pub depth_buffer: Arc<View>,
    pub render_pass: Arc<RenderPass>,
    pub vk_data: vk::VkFramebuffer,
}

impl Framebuffer {
    pub fn new(
        color_buffer: Vec<Arc<View>>,
        depth_buffer: Arc<View>,
        render_pass: Arc<RenderPass>,
    ) -> Self {
        let mut attachments = vec![0 as vk::VkImageView; color_buffer.len() + 1];
        let mut attachments_index = 0;
        for v in &color_buffer {
            attachments[attachments_index] = v.vk_data;
            attachments_index += 1;
        }
        attachments[attachments_index] = depth_buffer.vk_data;
        let dev = vxresult!(depth_buffer.image.read()).logical_device.clone();
        let ref surface_caps = &dev.physical_device.surface_caps;
        let mut fb_create_info = vk::VkFramebufferCreateInfo::default();
        fb_create_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO;
        fb_create_info.renderPass = render_pass.vk_data;
        fb_create_info.layers = 1;
        fb_create_info.attachmentCount = attachments.len() as u32;
        fb_create_info.pAttachments = attachments.as_ptr();
        fb_create_info.width = surface_caps.currentExtent.width;
        fb_create_info.height = surface_caps.currentExtent.height;
        let mut vk_data = 0 as vk::VkFramebuffer;
        vulkan_check!(vk::vkCreateFramebuffer(
            dev.vk_data,
            &fb_create_info,
            null(),
            &mut vk_data,
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
                vxresult!(self.depth_buffer.image.read())
                    .logical_device
                    .vk_data,
                self.vk_data,
                null(),
            );
        }
    }
}
