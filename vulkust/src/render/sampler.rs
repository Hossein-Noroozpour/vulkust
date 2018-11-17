#[cfg(directx12_api)]
pub use super::super::d3d12::sampler::*;
#[cfg(metal_api)]
pub use super::super::metal::sampler::*;
#[cfg(vulkan_api)]
pub use super::super::vulkan::sampler::*;

#[derive(Clone, Copy)]
#[cfg_attr(debug_mode, derive(Debug))]
pub enum Filter {
    Nearest,
    Linear,
    Cube,
}
