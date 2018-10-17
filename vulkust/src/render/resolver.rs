use super::gapi::GraphicApiEngine;
use super::image::{View as ImageView, Format, AttachmentType};
use super::render_pass::RenderPass;
use super::framebuffer::Framebuffer;
use std::sync::Arc;

pub(super) struct Resolver {
    buffers: Vec<Arc<ImageView>>,
    render_pass: Arc<RenderPass>,
    framebuffer: Arc<Framebuffer>,
}

impl Resolver {
    pub fn new(eng: &GraphicApiEngine) -> Self {
        let dev = eng.get_device();
        let memmgr = eng.get_memory_manager();
        let buffers = vec![
            Arc::new(ImageView::new_surface_attachment(
                dev.clone(), memmgr, Format::RgbaFloat, 1,
                AttachmentType::ResolverBuffer,
            )),
            Arc::new(ImageView::new_surface_attachment(
                dev.clone(), memmgr, Format::RgbaFloat, 1,
                AttachmentType::ResolverBuffer,
            )),
            Arc::new(ImageView::new_surface_attachment(
                dev.clone(), memmgr, Format::RgbaFloat, 1,
                AttachmentType::ResolverBuffer,
            )),
            Arc::new(ImageView::new_surface_attachment(
                dev.clone(), memmgr, Format::Float, 1,
                AttachmentType::ResolverBuffer,
            )),
        ];
        let render_pass = Arc::new(RenderPass::new(buffers.clone(), true, true));
        let framebuffer = Arc::new(Framebuffer::new(buffers.clone(), render_pass.clone()));
        Self {
            buffers,
            render_pass,
            framebuffer,
        }
    }
}