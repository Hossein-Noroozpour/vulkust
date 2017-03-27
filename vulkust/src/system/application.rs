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
        let mut o = OsApplication::new();
        let mut r = RenderEngine::new();
        let mut c = CoreApp::new();

        o.set_core_app(&mut c);
        o.set_rnd_eng(&mut r);

        r.set_os_app(&mut o);
        r.set_core_app(&mut c);

        Application {
            os_app: o,
            render_engine: r,
            core_app: c,
		}
	}
    pub fn run(&mut self) {
        self.os_app.start();
        self.render_engine.initialize();
        self.core_app.initialize(&mut self.os_app, &mut self.render_engine);
        self.os_app.execute();
        self.core_app.terminate();
        self.render_engine.terminate();
    }
}
