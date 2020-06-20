use super::command::Buffer as CmdBuffer;
use super::image::View as ImageView;
use super::render_pass::RenderPass;
use std::sync::Arc;

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct Framebuffer {}

impl Framebuffer {
    pub(crate) fn new(_buffers: Vec<Arc<ImageView>>, _render_pass: Arc<RenderPass>) -> Self {
        vx_unimplemented!();
    }

    pub(crate) fn begin(&self, _cmd_buffer: &mut CmdBuffer) {
        vx_unimplemented!();
    }

    pub(crate) fn get_render_pass(&self) -> &Arc<RenderPass> {
        vx_unimplemented!();
    }

    pub(crate) fn get_dimensions(&self) -> (u32, u32) {
        vx_unimplemented!();
    }
}
