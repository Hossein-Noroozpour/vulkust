pub mod config;
//pub mod vertex;
//pub mod mesh;

use super::vulkan;

pub fn initialize() {
    let driver = vulkan::Driver::new(config::FULLSCREEN_MODE);
}