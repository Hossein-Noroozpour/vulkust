use super::command::Buffer as CmdBuffer;
use super::config::Configurations;
use super::framebuffer::Framebuffer;
use super::gapi::GraphicApiEngine;
use super::image::{AttachmentType, Format as ImageFormat, View as ImageView};
use super::pipeline::{Pipeline, PipelineType};
use super::render_pass::RenderPass;
use std::sync::Arc;

const SHADOW_MAP_FMT: ImageFormat = ImageFormat::Float;
const SHADOW_ACCUMULATOR_FMT: ImageFormat = ImageFormat::Float;

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Shadower {
    //---------------------------------------
    shadow_map_buffers: Vec<Arc<ImageView>>,
    shadow_map_render_pass: Arc<RenderPass>,
    shadow_map_framebuffers: Vec<Arc<Framebuffer>>,
    shadow_map_pipeline: Arc<Pipeline>,
    //---------------------------------------
    black_accumulator_buffer: Arc<ImageView>,
    black_accumulator_render_pass: Arc<RenderPass>,
    black_accumulator_framebuffer: Arc<Framebuffer>,
    clear_black_accumulator_render_pass: Arc<RenderPass>,
    clear_black_accumulator_framebuffer: Arc<Framebuffer>,
}

impl Shadower {
    pub(super) fn new(geng: &GraphicApiEngine, conf: &Configurations) -> Self {
        let mut shadow_map_buffers = Vec::new();
        let dev = geng.get_device();
        let memmgr = geng.get_memory_manager();
        for _ in 0..conf.max_shadow_maps_count {
            shadow_map_buffers.push(Arc::new(ImageView::new_attachment(
                dev.clone(),
                memmgr,
                SHADOW_MAP_FMT,
                1,
                AttachmentType::DepthShadowBuffer,
                conf.shadow_map_aspect,
                conf.shadow_map_aspect,
            )));
        }
        shadow_map_buffers.shrink_to_fit();
        let black_accumulator_buffer = Arc::new(ImageView::new_surface_attachment(
            dev.clone(),
            memmgr,
            SHADOW_ACCUMULATOR_FMT,
            1,
            AttachmentType::ColorDisplay,
        ));
        let clear_black_accumulator_render_pass = Arc::new(RenderPass::new(
            vec![black_accumulator_buffer.clone()],
            true,
            false,
        ));
        let black_accumulator_render_pass = Arc::new(RenderPass::new(
            vec![black_accumulator_buffer.clone()],
            false,
            true,
        ));
        let shadow_map_render_pass = Arc::new(RenderPass::new(
            vec![shadow_map_buffers[0].clone()],
            true,
            true,
        ));
        let clear_black_accumulator_framebuffer = Arc::new(Framebuffer::new(
            vec![black_accumulator_buffer.clone()],
            black_accumulator_render_pass.clone(),
        ));
        let black_accumulator_framebuffer = Arc::new(Framebuffer::new(
            vec![black_accumulator_buffer.clone()],
            clear_black_accumulator_render_pass.clone(),
        ));
        let mut shadow_map_framebuffers = Vec::new();
        for v in &shadow_map_buffers {
            shadow_map_framebuffers.push(Arc::new(Framebuffer::new(
                vec![v.clone()],
                shadow_map_render_pass.clone(),
            )));
        }
        shadow_map_framebuffers.shrink_to_fit();
        let shadow_map_pipeline = vxresult!(geng.get_pipeline_manager().write())
            .create(shadow_map_render_pass.clone(), PipelineType::ShadowMapper);
        Self {
            shadow_map_buffers,
            shadow_map_render_pass,
            shadow_map_framebuffers,
            shadow_map_pipeline,
            //---------------------------------------
            black_accumulator_buffer,
            black_accumulator_render_pass,
            black_accumulator_framebuffer,
            clear_black_accumulator_render_pass,
            clear_black_accumulator_framebuffer,
        }
    }

    pub(super) fn begin_secondary_shadow_mappers(&self, cmds: &mut [CmdBuffer]) {
        let cmds_len = cmds.len();
        for i in 0..cmds_len {
            cmds[i].begin_secondary(&self.shadow_map_framebuffers[i]);
            cmds[i].bind_pipeline(&self.shadow_map_pipeline);
        }
    }
    // do thread shadow gathering
    // do main thread shadow accumulating
}

unsafe impl Send for Shadower {}
unsafe impl Sync for Shadower {}
