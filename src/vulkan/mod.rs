pub mod buffer;
pub mod command;
pub mod device;
pub mod fence;
pub mod image;
pub mod instance;
pub mod swapchain;
pub mod window;

use std;
use std::sync::{
    Arc,
    RwLock,
};

struct Driver {
    instance: Arc<RwLock<instance::Instance>>,
    device: Arc<RwLock<device::Device>>,
    cmd_pool: Arc<RwLock<command::pool::Pool>>,
    window: Arc<RwLock<window::Window>>,
    swapchain: Arc<RwLock<swapchain::Swapchain>>,
}

impl Driver {
    pub fn new(fullscreen_mode: bool) -> Self {
        let ins = Arc::new(RwLock::new(instance::Instance::new()));
        let dev = Arc::new(RwLock::new(device::Device::new(ins.clone())));
        let cmd_pool = Arc::new(RwLock::new(command::pool::Pool::new(
            dev.clone(), dev.read().unwrap().graphics_family_index)));
        let win = Arc::new(RwLock::new(window::Window::new(dev.clone())));
        let swp = Arc::new(RwLock::new(swapchain::Swapchain::new(win.clone())));
        Driver {
            instance: ins,
            device: dev,
            cmd_pool: cmd_pool,
            window: win,
            swapchain: swp,
        }
    }
}
