use super::super::command::Buffer as CmdBuffer;
use super::super::config::Configurations;
use super::super::framebuffer::Framebuffer;
use super::super::gapi::GraphicApiEngine;
use super::super::pipeline::{Pipeline, PipelineType};
use super::super::render_pass::RenderPass;
use std::sync::Arc;

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Unlit {
    render_pass: Arc<RenderPass>,
    framebuffers: Vec<Arc<Framebuffer>>,
    pipeline: Arc<Pipeline>,
}

impl Unlit {
    pub(crate) fn new(gapi_engine: &GraphicApiEngine, config: &Configurations) -> Self {
        let render_pass = gapi_engine.get_render_pass().clone();
        let framebuffers = gapi_engine.get_framebuffers().clone();
        let pipeline = vx_result!(gapi_engine.get_pipeline_manager().write()).create(
            render_pass.clone(),
            PipelineType::Unlit,
            config,
        );
        Self {
            pipeline,
            render_pass,
            framebuffers,
        }
    }

    pub(super) fn begin_secondary(&self, cmd: &mut CmdBuffer, frame_bumber: usize) {
        cmd.begin_secondary(&self.framebuffers[frame_bumber]);
        cmd.bind_pipeline(&self.pipeline);
    }
}
