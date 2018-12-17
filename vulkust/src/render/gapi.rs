#[cfg(directx12_api)]
pub use super::super::d3d12::engine::Engine as GraphicApiEngine;
#[cfg(vulkan_api)]
pub use super::super::vulkan::engine::Engine as GraphicApiEngine;
// maybe a day I forced to implement with other API
#[cfg(blank_gapi)]
pub(crate) use super::super::blank_gapi::engine::Engine as GraphicApiEngine;
