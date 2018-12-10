use super::command::Buffer as CmdBuffer;
use super::config::Configurations;
use super::framebuffer::Framebuffer;
use super::gapi::GraphicApiEngine;
use super::image::{AttachmentType, Format, View as ImageView};
use super::pipeline::{Pipeline, PipelineType};
use super::render_pass::RenderPass;
use super::texture::{Manager as TextureManager, Texture};
use std::sync::{Arc, RwLock};

#[cfg_attr(debug_mode, derive(Debug))]
pub struct GBufferFiller {
    textures: Vec<Arc<RwLock<Texture>>>,
    render_pass: Arc<RenderPass>,
    framebuffer: Arc<Framebuffer>,
    pipeline: Arc<Pipeline>,
}

impl GBufferFiller {
    pub(super) fn new(
        eng: &GraphicApiEngine,
        texmgr: &mut TextureManager,
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
            Arc::new(ImageView::new_surface_attachment(
                dev.clone(),
                memmgr,
                Format::RgbaFloat,
                AttachmentType::ColorGBuffer,
            )),
            Arc::new(ImageView::new_surface_attachment(
                dev.clone(),
                memmgr,
                Format::RgbaFloat,
                AttachmentType::ColorGBuffer,
            )),
            Arc::new(ImageView::new_surface_attachment(
                dev.clone(),
                memmgr,
                Format::DepthFloat,
                AttachmentType::DepthGBuffer,
            )),
        ];
        let sampler = eng.get_nearest_repeat_sampler();
        let mut textures = Vec::with_capacity(buffers.len());
        for b in &buffers {
            textures.push(texmgr.create_2d_with_view_sampler(b.clone(), sampler.clone()));
        }
        let render_pass = Arc::new(RenderPass::new(buffers.clone(), true, true));
        let framebuffer = Arc::new(Framebuffer::new(buffers, render_pass.clone()));
        let pipeline = vxresult!(eng.get_pipeline_manager().write()).create(
            render_pass.clone(),
            PipelineType::GBuffer,
            config,
        );
        Self {
            textures,
            render_pass,
            framebuffer,
            pipeline,
        }
    }

    pub(super) fn begin_secondary(&self, cmd: &mut CmdBuffer) {
        cmd.begin_secondary(&self.framebuffer);
        cmd.bind_pipeline(&self.pipeline);
    }

    pub(super) fn begin_primary(&self, cmd: &mut CmdBuffer) {
        self.framebuffer.begin(cmd);
    }

    pub(super) fn get_textures(&self) -> &Vec<Arc<RwLock<Texture>>> {
        return &self.textures;
    }

    pub(super) fn get_normal_texture(&self) -> &Arc<RwLock<Texture>> {
        return &self.textures[1];
    }

    pub(super) fn get_position_texture(&self) -> &Arc<RwLock<Texture>> {
        return &self.textures[0];
    }

    pub(super) fn get_depth_texture(&self) -> &Arc<RwLock<Texture>> {
        return &self.textures[3];
    }

    pub(super) fn get_framebuffer(&self) -> &Framebuffer {
        return &self.framebuffer;
    }
}

unsafe impl Send for GBufferFiller {}

unsafe impl Sync for GBufferFiller {}
