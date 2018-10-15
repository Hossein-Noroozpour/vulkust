use super::command::Buffer as CmdBuffer;
use super::image::View as ImageView;
use super::render_pass::RenderPass;
use super::vulkan as vk;

use std::ptr::null;
use std::sync::Arc;

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Framebuffer {
    pub(crate) clear_values: Vec<vk::VkClearValue>,
    pub(crate) color_buffers: Vec<Arc<ImageView>>,
    pub(crate) depth_buffer: Option<Arc<ImageView>>,
    pub(crate) render_pass: Arc<RenderPass>,
    pub(crate) viewport: vk::VkViewport,
    pub(crate) scissor: vk::VkRect2D,
    pub(crate) vk_data: vk::VkFramebuffer,
    width: u32,
    height: u32,
}

impl Framebuffer {
    pub(crate) fn new(
        color_buffers: Vec<Arc<ImageView>>,
        depth_buffer: Option<Arc<ImageView>>,
        render_pass: Arc<RenderPass>,
    ) -> Self {
        let mut width: u32 = 0;
        let mut height: u32 = 0;
        let mut vkdev = 0 as vk::VkDevice;
        //vxresult!(depth_buffer.image.read()).get_dimensions();

        let mut attachments = Vec::<vk::VkImageView>::new();
        {
            let mut push_view = |v: &Arc<ImageView>| {
                attachments.push(v.vk_data);
                let img = vxresult!(v.image.read());
                let a = img.get_dimensions();
                width = a.0;
                height = a.1;
                vkdev = img.logical_device.vk_data;
            };
            for v in &color_buffers {
                vxlogi!("cccccccccccccccccccccccccccccc");
                push_view(v);
            }
            if let Some(v) = &depth_buffer {
                vxlogi!("dddddddddddddddddddddddddddddd");
                push_view(v);
            }
        }
        vxlogi!("{}", attachments.len());

        let mut fb_create_info = vk::VkFramebufferCreateInfo::default();
        fb_create_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO;
        fb_create_info.renderPass = render_pass.vk_data;
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
            width,
            height,
        }
    }

    pub(crate) fn get_dimensions(&self) -> (u32, u32) {
        return (self.width, self.height);
    }

    pub(crate) fn begin(&self, cmd_buffer: &mut CmdBuffer) {
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
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        let vkdev = if self.color_buffers.len() > 0 {
            vxresult!(self.color_buffers[0].image.read())
                .logical_device
                .vk_data
        } else if let Some(v) = &self.depth_buffer {
            vxresult!(v.image.read()).logical_device.vk_data
        } else {
            vxunexpected!();
        };
        unsafe {
            vk::vkDestroyFramebuffer(vkdev, self.vk_data, null());
        }
    }
}
