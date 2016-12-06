pub mod vertex;
pub mod mesh;

use super::vulkan;

pub fn initialize() {
    #[cfg(debug_assertions)]
    const FULLSCREEN_MODE: bool = false;
    #[cfg(not(debug_assertions))]
    const FULLSCREEN_MODE: bool = true;
    #[cfg(not(target_os = "android"))]
    vulkan::initialize(FULLSCREEN_MODE);
}