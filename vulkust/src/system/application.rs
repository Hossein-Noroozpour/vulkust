use super::super::core::application::ApplicationTrait;
use super::super::render::engine::{RenderEngine, EngineTrait as RenderEngineTrait};
use super::os::{OsApplication, ApplicationTrait as OsApplicationTrait};

use std::ptr::{
    null_mut,
};

pub struct Application <CoreApp> where CoreApp: ApplicationTrait {
    os_app: OsApplication<CoreApp>,
    render_engine: RenderEngine<CoreApp>,
   	core_app: CoreApp,
}

impl<CoreApp> Application<CoreApp> where CoreApp: ApplicationTrait {
	pub fn new() -> Self {
        let mut o = OsApplication::new();
        let mut r = RenderEngine::new();
        let mut c = CoreApp::new();
		Application {
            os_app: o,
            render_engine: r,
            core_app: c,
		}
	}
    pub fn run(&mut self) {
        self.core_app.update();
    }
}
