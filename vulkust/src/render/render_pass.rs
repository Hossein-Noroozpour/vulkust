#[cfg(directx12_api)]
pub use super::super::d3d12::render_pass::*;
#[cfg(metal_api)]
pub use super::super::metal::render_pass::*;
#[cfg(vulkan_api)]
pub use super::super::vulkan::render_pass::*;