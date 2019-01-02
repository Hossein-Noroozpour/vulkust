use super::super::command::Buffer as CmdBuffer;
use super::super::config::Configurations;
use super::super::framebuffer::Framebuffer;
use super::super::g_buffer_filler::GBufferFiller;
use super::super::gapi::GraphicApiEngine;
use super::super::image::{AttachmentType, Format, View as ImageView};
use super::super::pipeline::{Pipeline, PipelineType};
use super::super::render_pass::RenderPass;
use super::super::texture::{Manager as TextureManager, Texture};
use std::sync::{Arc, RwLock};

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Transparent {
    render_pass: Arc<RenderPass>,
    framebuffer: Arc<Framebuffer>,
    pipeline: Arc<Pipeline>,
    color_texture: Arc<RwLock<Texture>>,
}

impl Transparent {
    pub(crate) fn new(
        eng: &GraphicApiEngine,
        texmgr: &mut TextureManager,
        g_buffer_filler: &GBufferFiller,
        config: &Configurations,
    ) -> Self {
        let dev = eng.get_device();
        let memmgr = eng.get_memory_manager();
        let buffers = vec![
            Arc::new(ImageView::new_surface_attachment(
                dev.clone(),
                memmgr,
                Format::RgbaFloat,
                AttachmentType::ColorGBuffer,
            )),
            vxresult!(g_buffer_filler.get_depth_texture().read())
                .get_image_view()
                .clone(),
        ];
        let sampler = eng.get_nearest_repeat_sampler();
        let color_texture = texmgr.create_2d_with_view_sampler(buffers[0].clone(), sampler.clone());
        let render_pass = Arc::new(RenderPass::new(buffers.clone(), true, true));
        let framebuffer = Arc::new(Framebuffer::new(buffers, render_pass.clone()));
        let pipeline = vxresult!(eng.get_pipeline_manager().write()).create(
            render_pass.clone(),
            PipelineType::TransparentPBR,
            config,
        );
        Self {
            pipeline,
            render_pass,
            framebuffer,
            color_texture,
        }
    }

    pub(super) fn begin_secondary(&self, cmd: &mut CmdBuffer) {
        cmd.begin_secondary(&self.framebuffer);
        cmd.bind_pipeline(&self.pipeline);
    }

    pub(super) fn record_primary(&self, pricmd: &mut CmdBuffer, seccmd: &CmdBuffer) {
        pricmd.begin();
        self.framebuffer.begin(pricmd);
        pricmd.exe_cmd(seccmd);
        pricmd.end_render_pass();
        pricmd.end();
    }

    pub(crate) fn get_color_texture(&self) -> &Arc<RwLock<Texture>> {
        return &self.color_texture;
    }
}
