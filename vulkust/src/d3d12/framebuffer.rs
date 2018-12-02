use super::command::Buffer as CmdBuffer;
use super::image::View as ImageView;
use super::render_pass::RenderPass;
use std::sync::Arc;

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct Framebuffer {}

impl Framebuffer {
    pub(crate) fn new(buffers: Vec<Arc<ImageView>>, render_pass: Arc<RenderPass>) -> Self {
        vxunimplemented!();
    }

    pub(crate) fn begin(&self, cmd_buffer: &mut CmdBuffer) {
        vxunimplemented!();
    }

    pub(crate) fn get_render_pass(&self) -> &Arc<RenderPass> {
        vxunimplemented!();
    }

    pub(crate) fn get_dimensions(&self) -> (u32, u32) {
        vxunimplemented!();
    }
}
