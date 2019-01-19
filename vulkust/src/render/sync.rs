#[cfg(blank_gapi)]
pub(crate) use super::super::blank_gapi::sync::*;
#[cfg(directx12_api)]
pub use super::super::d3d12::sync::*;
#[cfg(metal_api)]
pub use super::super::metal::sync::*;
#[cfg(vulkan_api)]
pub(crate) use super::super::vulkan::sync::*;
