use super::super::render::config::Configurations;
use super::super::render::pipeline::PipelineType;
use super::render_pass::RenderPass;
use std::sync::Arc;

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct Pipeline {}

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct Manager {}

impl Manager {
    pub(crate) fn create(
        &mut self,
        _render_pass: Arc<RenderPass>,
        _pipeline_type: PipelineType,
        _config: &Configurations,
    ) -> Arc<Pipeline> {
        vx_unimplemented!();
    }
}
