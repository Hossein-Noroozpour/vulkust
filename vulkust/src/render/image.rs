#[cfg(directx12_api)]
pub use super::super::d3d12::image::*;
#[cfg(metal_api)]
pub use super::super::metal::image::*;
#[cfg(vulkan_api)]
pub use super::super::vulkan::image::*;

#[cfg_attr(debug_mode, derive(Debug))]
pub enum AttachmentType {
    ColorGBuffer,
    DepthGBuffer,
    ColorDisplay,
    DepthStencilDisplay,
}
