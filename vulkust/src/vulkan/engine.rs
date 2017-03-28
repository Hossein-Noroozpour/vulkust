use std::ptr::null_mut;
use std::sync::Arc;

use super::super::render::engine::EngineTrait;
use super::super::core::application::ApplicationTrait;
use super::super::system::os::OsApplication;
use super::instance::Instance;

pub struct Engine<CoreApp> where CoreApp: ApplicationTrait {
    pub core_app: *mut CoreApp,
    pub os_app: *mut OsApplication<CoreApp>,
    pub instance: Option<Arc<Instance>>,
}

impl<CoreApp> EngineTrait<CoreApp> for Engine<CoreApp> where CoreApp: ApplicationTrait {
    fn new() -> Self {
        Engine {
            core_app: null_mut(),
            os_app: null_mut(),
            instance: None,
        }
    }

    fn set_core_app(&mut self, c: *mut CoreApp) {
        self.core_app = c;
    }

    fn set_os_app(&mut self, o: *mut OsApplication<CoreApp>) {
        self.os_app = o;
    }

    fn initialize(&mut self) {
        self.instance = Some(Arc::new(Instance::new()));
    }

    fn update(&mut self) {
        // TODO
    }

    fn terminate(&mut self) {
        self.instance = None;
    }
}
