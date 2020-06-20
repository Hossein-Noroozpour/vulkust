use super::command::Buffer as CmdBuffer;
use super::image::View as ImageView;
use super::render_pass::RenderPass;
use ash::version::DeviceV1_0;
use ash::vk;
use std::sync::Arc;

pub(crate) struct Framebuffer {
    clear_values: Vec<vk::ClearValue>,
    buffers: Vec<Arc<ImageView>>,
    render_pass: Arc<RenderPass>,
    viewport: vk::Viewport,
    scissor: vk::Rect2D,
    vk_data: vk::Framebuffer,
}

impl Framebuffer {
    pub(crate) fn new(buffers: Vec<Arc<ImageView>>, render_pass: Arc<RenderPass>) -> Self {
        let mut width: u32 = 0;
        let mut height: u32 = 0;
        let mut vkdev = None;
        let mut has_depth = false;

        let mut attachments = Vec::<vk::ImageView>::new();
        for v in &buffers {
            attachments.push(v.get_data());
            let img = vx_result!(v.get_image().read());
            let a = img.get_dimensions();
            if vx_flag_check!(
                img.get_vk_usage(),
                vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT
            ) {
                has_depth = true;
            }
            width = a.0;
            height = a.1;
            vkdev = Some(img.get_device().get_data().clone());
        }
        let vkdev = vx_unwrap!(vkdev);

        let fb_create_info = vk::FramebufferCreateInfo::builder()
            .render_pass(*render_pass.get_data())
            .layers(1)
            .attachments(&attachments)
            .width(width)
            .height(height);
        let vk_data = vx_result!(unsafe { vkdev.create_framebuffer(&fb_create_info, None) });

        let mut clear_values = vec![
            vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.0, 0.0, 0.0, 0.0],
                },
            };
            buffers.len()
        ];
        if has_depth {
            unsafe {
                clear_values[buffers.len() - 1].color.float32[0] = 1.0;
            }
        }

        let viewport = vk::Viewport::builder()
            .x(0.0)
            .y(0.0)
            .height(height as f32)
            .width(width as f32)
            .min_depth(0.0)
            .max_depth(1.0)
            .build();

        let mut scissor = vk::Rect2D::default();
        scissor.extent.width = width;
        scissor.extent.height = height;
        scissor.offset.x = 0;
        scissor.offset.y = 0;

        Self {
            clear_values,
            buffers,
            render_pass,
            viewport,
            scissor,
            vk_data,
        }
    }

    pub(crate) fn get_dimensions(&self) -> (u32, u32) {
        return vx_result!(self.buffers[0].get_image().read()).get_dimensions();
    }

    pub(crate) fn begin(&self, cmd_buffer: &mut CmdBuffer) {
        let (width, height) = self.get_dimensions();

        let mut render_pass_begin_info = vk::RenderPassBeginInfo::default();
        render_pass_begin_info.render_pass = *self.render_pass.get_data();
        render_pass_begin_info.render_area.offset.x = 0;
        render_pass_begin_info.render_area.offset.y = 0;
        render_pass_begin_info.render_area.extent.width = width;
        render_pass_begin_info.render_area.extent.height = height;
        render_pass_begin_info.clear_value_count = self.clear_values.len() as u32;
        render_pass_begin_info.p_clear_values = self.clear_values.as_ptr();
        render_pass_begin_info.framebuffer = self.vk_data;

        cmd_buffer.begin_render_pass_with_info(&render_pass_begin_info);
    }

    // pub(crate) fn get_buffers(&self) -> &Vec<Arc<ImageView>> {
    //     return &self.buffers;
    // }

    pub(crate) fn get_data(&self) -> &vk::Framebuffer {
        return &self.vk_data;
    }

    pub(crate) fn get_render_pass(&self) -> &Arc<RenderPass> {
        return &self.render_pass;
    }

    pub(crate) fn get_vk_viewport(&self) -> &vk::Viewport {
        return &self.viewport;
    }

    pub(crate) fn get_vk_scissor(&self) -> &vk::Rect2D {
        return &self.scissor;
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        let img = vx_result!(self.buffers[0].get_image().read());
        let vkdev = img.get_device().get_data();
        unsafe {
            vkdev.destroy_framebuffer(self.vk_data, None);
        }
    }
}

#[cfg(debug_mode)]
impl std::fmt::Debug for Framebuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Vulkan Framebuffer")
    }
}
