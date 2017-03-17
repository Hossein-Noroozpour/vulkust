
use super::super::super::core::application::ApplicationTrait;
use super::super::super::render::engine::RenderEngine;
use super::super::os::{OsApplication, ApplicationTrait as OsApplicationTrait};

use std::ptr::{
    null,
    null_mut,
};

pub struct Application <CoreApp> where CoreApp: ApplicationTrait {

   	core_app: *mut CoreApp,
    render_engine: *mut RenderEngine<CoreApp>,
}

impl<CoreApp> OsApplicationTrait <CoreApp> for Application<CoreApp>
        where CoreApp: ApplicationTrait {
	fn new() -> Self {
		Application {
           	core_app: null_mut(),
            render_engine: null_mut(),
		}
	}
    fn start(&mut self) -> bool {
        return true;
    }
    fn set_core_app(&mut self, c: *mut CoreApp) {
        self.core_app = c;
    }
    fn set_rnd_eng(&mut self, r: *mut RenderEngine<CoreApp>) {
        self.render_engine = r;
    }
    fn execute() -> bool {
        return true;
    }
}
