//pub mod buffer;
//pub mod command;
pub mod device;
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
    pub surface: Option<Arc<surface::Surface>>,
    pub physical_device: Option<Arc<device::physical::Physical>>,
    pub logical_device: Option<Arc<device::logical::Logical>>,
//    pub cmd_pool: Arc<command::pool::Pool>,
//    pub window: Arc<window::Window>,
//    pub swapchain: Arc<swapchain::Swapchain>,
}

impl Driver {
    pub fn new() -> Self {
        Driver {
            instance: Arc::new(instance::Instance::new()),
            surface: None,
            physical_device: None,
            logical_device: None,
        }
//        let dev = Arc::new(device::Device::new(ins.clone()));
//        let cmd_pool = Arc::new(command::pool::Pool::new(
//            dev.clone(), dev.graphics_family_index));
//        let win = Arc::new(window::Window::new(dev.clone()));
//        let swp = Arc::new(swapchain::Swapchain::new(win.clone()));
    }

    pub fn initialize(&mut self, surface: surface::Surface) {
        let surface = Arc::new(surface);
        let physical_device = Arc::new(device::physical::Physical::new(self.instance.clone()));
        let logical_device = Arc::new(device::logical::Logical::new(physical_device.clone()));
        logerr!("Reached!!");
        logdbg!(format!("Depth format is: {:?}", physical_device.get_supported_depth_format()));
        logerr!("Reached!!");
        self.surface = Some(surface);
        self.physical_device = Some(physical_device);
        self.logical_device = Some(logical_device);
    }

    pub fn terminate(&mut self) {
        self.logical_device = None;
        self.physical_device = None;
        self.surface = None;
    }
}
