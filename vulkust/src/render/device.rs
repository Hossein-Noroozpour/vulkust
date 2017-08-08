#[cfg(feature = "d3d12")]
pub use super::super::d3d12::device::*;
#[cfg(any(feature = "metal", target_os = "macos"))]
pub use super::super::metal::device::*;
#[cfg(all(not(feature = "metal"), not(feature = "d3d12"), not(target_os = "macos")))]
pub use super::super::vulkan::device::*;
