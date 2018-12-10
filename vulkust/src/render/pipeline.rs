#[cfg(directx12_api)]
pub(crate) use super::super::d3d12::pipeline::*;
#[cfg(metal_api)]
pub(crate) use super::super::metal::pileline::*;
#[cfg(vulkan_api)]
pub(crate) use super::super::vulkan::pipeline::*;

#[repr(u8)]
#[derive(Clone, Copy, PartialOrd, PartialEq, Eq, Ord)]
#[cfg_attr(debug_mode, derive(Debug))]
pub enum PipelineType {
    Deferred,
    GBuffer,
    ShadowMapper,
    ShadowAccumulatorDirectional,
    SSAO,
}
