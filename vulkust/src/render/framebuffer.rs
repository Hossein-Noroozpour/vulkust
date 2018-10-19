#[cfg(directx12_api)]
pub(crate) use super::super::d3d12::framebuffer::*;
#[cfg(metal_api)]
pub(crate) use super::super::metal::framebuffer::*;
#[cfg(vulkan_api)]
pub(crate) use super::super::vulkan::framebuffer::*;
