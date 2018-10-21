#[cfg(directx12_api)]
pub use super::super::d3d12::pipeline::*;
#[cfg(metal_api)]
pub use super::super::metal::pileline::*;
#[cfg(vulkan_api)]
pub use super::super::vulkan::pipeline::*;

#[repr(u8)]
#[derive(Clone, Copy)]
#[cfg_attr(debug_mode, derive(Debug))]
pub enum PipelineType {
    Deferred,
    Resolver,
    GBuffer,
    ShadowMapper,
    ShadowAccumulatorDirectional,
}
