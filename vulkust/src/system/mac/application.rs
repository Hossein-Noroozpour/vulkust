use std::ptr::null_mut;
use std::os::raw::c_void;
use super::super::super::core::application::ApplicationTrait;
use super::super::super::render::engine::RenderEngine;
use super::super::os::ApplicationTrait as OsApplicationTrait;

pub struct Application<CoreApp> where CoreApp: ApplicationTrait  {
    pub core_app: *mut CoreApp,
    pub render_engine: *mut RenderEngine<CoreApp>,
}

impl<CoreApp> OsApplicationTrait<CoreApp> for Application<CoreApp>
    where CoreApp: ApplicationTrait {
    fn new(_: *const c_void) -> Self {
        Application {
            core_app: null_mut(),
            render_engine: null_mut(),
        }
    }
    fn initialize(&mut self) -> bool {
        true
    }
    fn set_core_app(&mut self, c: *mut CoreApp) {
        self.core_app = c;
    }
    fn set_rnd_eng(&mut self, r: *mut RenderEngine<CoreApp>) {
        self.render_engine = r;
    }
    fn execute(&mut self) -> bool {
        true
    }
}
