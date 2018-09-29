use super::command::buffer::Buffer as CmdBuffer;
use super::image::View;
use super::render_pass::RenderPass;
use super::vulkan as vk;

use std::ptr::null;
use std::sync::Arc;

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Framebuffer {
    pub clear_values: Vec<vk::VkClearValue>,
    pub color_buffers: Vec<Arc<View>>,
    pub depth_buffer: Arc<View>,
    pub render_pass: Arc<RenderPass>,
    pub viewport: vk::VkViewport,
    pub scissor: vk::VkRect2D,
    pub vk_data: vk::VkFramebuffer,
}

impl Framebuffer {
    pub fn new(
        color_buffers: Vec<Arc<View>>,
        depth_buffer: Arc<View>,
        render_pass: Arc<RenderPass>,
    ) -> Self {
        let (width, height) = vxresult!(depth_buffer.image.read()).get_dimensions();

        let mut attachments = vec![0 as vk::VkImageView; color_buffers.len() + 1];
        let mut attachments_index = 0;
        for v in &color_buffers {
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

        let mut clear_values = Vec::new();
        for _ in 0..color_buffers.len() {
            clear_values.push(vk::VkClearValue {
                data: [0.0, 0.0, 0.0, 0.0],
            });
        }
        clear_values.push(vk::VkClearValue {
            data: [1.0, 0.0, 0.0, 0.0],
        });
        clear_values.shrink_to_fit();

        let mut viewport = vk::VkViewport::default();
        viewport.x = 0.0;
        viewport.y = 0.0;
        viewport.height = height as f32;
        viewport.width = width as f32;
        viewport.minDepth = 0.0;
        viewport.maxDepth = 1.0;

        let mut scissor = vk::VkRect2D::default();
        scissor.extent.width = width;
        scissor.extent.height = height;
        scissor.offset.x = 0;
        scissor.offset.y = 0;

        Framebuffer {
            clear_values,
            color_buffers,
            depth_buffer,
            render_pass,
            viewport,
            scissor,
            vk_data,
        }
    }

    pub fn get_dimensions(&self) -> (u32, u32) {
        return vxresult!(self.depth_buffer.image.read()).get_dimensions();
    }

    pub fn begin_render(&self, cmd_buffer: &mut CmdBuffer) {
        let (width, height) = self.get_dimensions();

        let mut render_pass_begin_info = vk::VkRenderPassBeginInfo::default();
        render_pass_begin_info.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO;
        render_pass_begin_info.renderPass = self.render_pass.vk_data;
        render_pass_begin_info.renderArea.offset.x = 0;
        render_pass_begin_info.renderArea.offset.y = 0;
        render_pass_begin_info.renderArea.extent.width = width;
        render_pass_begin_info.renderArea.extent.height = height;
        render_pass_begin_info.clearValueCount = self.clear_values.len() as u32;
        render_pass_begin_info.pClearValues = self.clear_values.as_ptr();
        render_pass_begin_info.framebuffer = self.vk_data;

        cmd_buffer.begin_render_pass_with_info(render_pass_begin_info);
        cmd_buffer.set_viewport(&self.viewport);
        cmd_buffer.set_scissor(&self.scissor);
    }

    pub fn end_render(&self, cmd_buffer: &mut CmdBuffer) {
        cmd_buffer.end_render_pass();
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
