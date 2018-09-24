#[cfg(d3d12)]
pub use super::super::d3d12::device::*;
#[cfg(metal)]
pub use super::super::metal::device::*;
#[cfg(vulkan_api)]
pub use super::super::vulkan::device::*;
