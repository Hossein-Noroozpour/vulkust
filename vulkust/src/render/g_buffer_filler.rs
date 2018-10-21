use super::command::Buffer as CmdBuffer;
use super::framebuffer::Framebuffer;
use super::gapi::GraphicApiEngine;
use super::image::{AttachmentType, Format, View as ImageView};
use super::pipeline::{Pipeline, PipelineType};
use super::render_pass::RenderPass;
use std::sync::Arc;

#[cfg_attr(debug_mode, derive(Debug))]
pub(super) struct GBufferFiller {
    buffers: Vec<Arc<ImageView>>,
    render_pass: Arc<RenderPass>,
    framebuffer: Arc<Framebuffer>,
    pipeline: Arc<Pipeline>,
}

impl GBufferFiller {
    pub(super) fn new(eng: &GraphicApiEngine) -> Self {
        let dev = eng.get_device();
        let memmgr = eng.get_memory_manager();
        let samples_count = eng.get_samples_count();
        let buffers = vec![
            Arc::new(ImageView::new_surface_attachment(
                dev.clone(),
                memmgr,
                Format::RgbaFloat,
                samples_count,
                AttachmentType::ColorGBuffer,
            )),
            Arc::new(ImageView::new_surface_attachment(
                dev.clone(),
                memmgr,
                Format::RgbaFloat,
                samples_count,
                AttachmentType::ColorGBuffer,
            )),
            Arc::new(ImageView::new_surface_attachment(
                dev.clone(),
                memmgr,
                Format::RgbaFloat,
                samples_count,
                AttachmentType::ColorGBuffer,
            )),
            Arc::new(ImageView::new_surface_attachment(
                dev.clone(),
                memmgr,
                Format::DepthFloat,
                samples_count,
                AttachmentType::DepthGBuffer,
            )),
        ];
        let render_pass = Arc::new(RenderPass::new(buffers.clone(), true, true));
        let framebuffer = Arc::new(Framebuffer::new(buffers.clone(), render_pass.clone()));
        let pipeline = vxresult!(eng.get_pipeline_manager().write())
            .create(render_pass.clone(), PipelineType::GBuffer);
        Self {
            buffers,
            render_pass,
            framebuffer,
            pipeline,
        }
    }

    pub(super) fn get_buffers(&self) -> &Vec<Arc<ImageView>> {
        return &self.buffers;
    }

    pub(super) fn begin_secondary(&self, cmd: &mut CmdBuffer) {
        cmd.begin_secondary(&self.framebuffer);
        cmd.bind_pipeline(&self.pipeline);
    }

    pub(super) fn begin_primary(&self, cmd: &mut CmdBuffer) {
        self.framebuffer.begin(cmd);
    }
}

unsafe impl Send for GBufferFiller {}

unsafe impl Sync for GBufferFiller {}
