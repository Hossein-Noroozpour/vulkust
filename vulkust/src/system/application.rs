use super::super::core::application::ApplicationTrait;
use super::super::render::engine::{RenderEngine, EngineTrait as RenderEngineTrait};
use super::os::{OsApplication, ApplicationTrait as OsApplicationTrait};

pub struct Application <CoreApp> where CoreApp: ApplicationTrait {
    os_app: OsApplication<CoreApp>,
    render_engine: RenderEngine<CoreApp>,
   	core_app: CoreApp,
}

impl<CoreApp> Application<CoreApp> where CoreApp: ApplicationTrait {
	pub fn new() -> Self {
        Application {
            os_app: OsApplication::new(),
            render_engine: RenderEngine::new(),
            core_app: CoreApp::new(),
		}
	}
    pub fn run(&mut self) {
        self.os_app.set_core_app(&mut self.core_app);
        self.os_app.set_rnd_eng(&mut self.render_engine);
        self.render_engine.set_os_app(&mut self.os_app);
        self.render_engine.set_core_app(&mut self.core_app);
        self.os_app.start();
        self.render_engine.initialize();
        self.core_app.initialize(&mut self.os_app, &mut self.render_engine);
        self.os_app.execute();
        self.core_app.terminate();
        self.render_engine.terminate();
    }
}
