use super::command::Buffer as CmdBuffer;
use super::image::View as ImageView;
use super::render_pass::RenderPass;
use super::vulkan as vk;

use std::ptr::null;
use std::sync::Arc;

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct Framebuffer {
    clear_values: Vec<vk::VkClearValue>,
    buffers: Vec<Arc<ImageView>>,
    render_pass: Arc<RenderPass>,
    viewport: vk::VkViewport,
    scissor: vk::VkRect2D,
    vk_data: vk::VkFramebuffer,
}

impl Framebuffer {
    pub(crate) fn new(buffers: Vec<Arc<ImageView>>, render_pass: Arc<RenderPass>) -> Self {
        let mut width: u32 = 0;
        let mut height: u32 = 0;
        let mut vkdev = 0 as vk::VkDevice;
        let mut has_depth = false;

        let mut attachments = Vec::<vk::VkImageView>::new();
        for v in &buffers {
            attachments.push(v.get_data());
            let img = vxresult!(v.get_image().read());
            let a = img.get_dimensions();
            if img.get_vk_usage()
                & vk::VkImageUsageFlagBits::VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT
                    as vk::VkImageUsageFlags
                != 0
            {
                has_depth = true;
            }
            width = a.0;
            height = a.1;
            vkdev = img.get_device().get_data();
        }

        let mut fb_create_info = vk::VkFramebufferCreateInfo::default();
        fb_create_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO;
        fb_create_info.renderPass = render_pass.get_data();
        fb_create_info.layers = 1;
        fb_create_info.attachmentCount = attachments.len() as u32;
        fb_create_info.pAttachments = attachments.as_ptr();
        fb_create_info.width = width;
        fb_create_info.height = height;
        let mut vk_data = 0 as vk::VkFramebuffer;
        vulkan_check!(vk::vkCreateFramebuffer(
            vkdev,
            &fb_create_info,
            null(),
            &mut vk_data,
        ));

        let mut clear_values = vec![
            vk::VkClearValue {
                data: [0.0, 0.0, 0.0, 0.0],
            };
            buffers.len()
        ];
        if has_depth {
            clear_values[buffers.len() - 1].data[0] = 1.0;
        }

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
            buffers,
            render_pass,
            viewport,
            scissor,
            vk_data,
        }
    }

    pub(crate) fn get_dimensions(&self) -> (u32, u32) {
        return vxresult!(self.buffers[0].get_image().read()).get_dimensions();
    }

    pub(crate) fn begin(&self, cmd_buffer: &mut CmdBuffer) {
        let (width, height) = self.get_dimensions();

        let mut render_pass_begin_info = vk::VkRenderPassBeginInfo::default();
        render_pass_begin_info.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO;
        render_pass_begin_info.renderPass = self.render_pass.get_data();
        render_pass_begin_info.renderArea.offset.x = 0;
        render_pass_begin_info.renderArea.offset.y = 0;
        render_pass_begin_info.renderArea.extent.width = width;
        render_pass_begin_info.renderArea.extent.height = height;
        render_pass_begin_info.clearValueCount = self.clear_values.len() as u32;
        render_pass_begin_info.pClearValues = self.clear_values.as_ptr();
        render_pass_begin_info.framebuffer = self.vk_data;

        cmd_buffer.begin_render_pass_with_info(render_pass_begin_info);
    }

    // pub(crate) fn get_buffers(&self) -> &Vec<Arc<ImageView>> {
    //     return &self.buffers;
    // }

    pub(crate) fn get_data(&self) -> vk::VkFramebuffer {
        return self.vk_data;
    }

    pub(crate) fn get_render_pass(&self) -> &Arc<RenderPass> {
        return &self.render_pass;
    }

    pub(crate) fn get_vk_viewport(&self) -> &vk::VkViewport {
        return &self.viewport;
    }

    pub(crate) fn get_vk_scissor(&self) -> &vk::VkRect2D {
        return &self.scissor;
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        let vkdev = vxresult!(self.buffers[0].get_image().read())
            .get_device()
            .get_data();
        unsafe {
            vk::vkDestroyFramebuffer(vkdev, self.vk_data, null());
        }
    }
}
