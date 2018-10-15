#[cfg(d3d12)]
pub use super::super::d3d12::pipeline::*;
#[cfg(metal)]
pub use super::super::metal::pileline::*;
#[cfg(vulkan)]
pub use super::super::vulkan::pipeline::*;

#[derive(PartialEq)]
#[cfg_attr(debug_mode, derive(Debug))]
pub enum PipelineType {
    Deferred,
    GBuffer,
}
