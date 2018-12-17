#[cfg(blank_gapi)]
pub(crate) use super::super::blank_gapi::render_pass::*;
#[cfg(directx12_api)]
pub(crate) use super::super::d3d12::render_pass::*;
#[cfg(metal_api)]
pub(crate) use super::super::metal::render_pass::*;
#[cfg(vulkan_api)]
pub(crate) use super::super::vulkan::render_pass::*;
