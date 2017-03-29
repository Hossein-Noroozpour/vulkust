use std::ptr::null_mut;
use std::sync::Arc;

use super::super::render::engine::EngineTrait;
use super::super::core::application::ApplicationTrait;
use super::super::system::os::OsApplication;
use super::instance::Instance;
use super::surface::Surface;
use super::device::physical::Physical as PhysicalDevice;

pub struct Engine<CoreApp> where CoreApp: ApplicationTrait {
    pub core_app: *mut CoreApp,
    pub os_app: *mut OsApplication<CoreApp>,
    pub instance: Option<Arc<Instance>>,
    pub surface: Option<Arc<Surface>>,
    pub physical_device: Option<Arc<PhysicalDevice>>,
}

impl<CoreApp> EngineTrait<CoreApp> for Engine<CoreApp> where CoreApp: ApplicationTrait {
    fn new() -> Self {
        Engine {
            core_app: null_mut(),
            os_app: null_mut(),
            instance: None,
            surface: None,
            physical_device: None,
        }
    }

    fn set_core_app(&mut self, c: *mut CoreApp) {
        self.core_app = c;
    }

    fn set_os_app(&mut self, o: *mut OsApplication<CoreApp>) {
        self.os_app = o;
    }

    fn initialize(&mut self) {
        let instance = Arc::new(Instance::new());
        #[cfg(target_os = "linux")]
        let surface = Arc::new(Surface::new(
            instance.clone(),
            unsafe { (*self.os_app).connection },
            unsafe { (*self.os_app).window }));
        let physical_device = Arc::new(PhysicalDevice::new(surface.clone()));
        self.instance = Some(instance);
        self.surface = Some(surface);
        self.physical_device = Some(physical_device);
    }

    fn update(&mut self) {
        // TODO
    }

    fn terminate(&mut self) {
        self.physical_device = None;
        self.surface = None;
        self.instance = None;
    }
}
