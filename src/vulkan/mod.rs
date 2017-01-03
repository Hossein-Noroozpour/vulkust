//pub mod buffer;
//pub mod command;
//pub mod device;
//pub mod fence;
//pub mod image;
pub mod instance;
pub mod surface;
//pub mod swapchain;
//pub mod window;

//use std;
use std::sync::{
    Arc,
};

pub struct Driver {
    pub instance: Arc<instance::Instance>,
    pub surface: Option<Arc<surface::Surface>>
//    pub device: Arc<device::Device>,
//    pub cmd_pool: Arc<command::pool::Pool>,
//    pub window: Arc<window::Window>,
//    pub swapchain: Arc<swapchain::Swapchain>,
}

impl Driver {
    pub fn new() -> Self {
        Driver {
            instance: Arc::new(instance::Instance::new()),
            surface: None,
        }
//        let dev = Arc::new(device::Device::new(ins.clone()));
//        let cmd_pool = Arc::new(command::pool::Pool::new(
//            dev.clone(), dev.graphics_family_index));
//        let win = Arc::new(window::Window::new(dev.clone()));
//        let swp = Arc::new(swapchain::Swapchain::new(win.clone()));
    }
}
