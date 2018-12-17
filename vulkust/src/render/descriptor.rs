#[cfg(blank_gapi)]
pub(crate) use super::super::blank_gapi::descriptor::*;
#[cfg(directx12_api)]
pub(crate) use super::super::d3d12::descriptor::*;
#[cfg(metal)]
pub(crate) use super::super::metal::descriptor::*;
#[cfg(vulkan_api)]
pub(crate) use super::super::vulkan::descriptor::*;
