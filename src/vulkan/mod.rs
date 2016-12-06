pub mod instance;
pub mod device;
pub mod window;
pub mod command;
pub mod swapchain;
pub mod image;

use std;
use std::sync::{
    Arc,
    RwLock,
};

pub fn initialize(fullscreen_mode: bool) {
    let ins = Arc::new(RwLock::new(instance::Instance::new()));
    let dev = Arc::new(RwLock::new(device::Device::new(ins.clone())));
    let cmd_pool = command::pool::Pool::new(
        dev.clone(),
        dev.read().unwrap().graphics_family_index);
    let win = Arc::new(RwLock::new(window::Window::new(dev.clone())));
    let swp = Arc::new(RwLock::new(swapchain::Swapchain::new(win.clone())));
    std::thread::sleep(std::time::Duration::from_millis(4000));
    let _ = win;
}