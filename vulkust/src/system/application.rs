#[cfg(target_os = "android")]
extern crate libc;
use std::ptr::null;
use std::mem::transmute;
#[cfg(target_os = "android")]
use self::libc::{
    c_void,
    size_t,
};
use super::super::core::application::ApplicationTrait;
use super::super::render::engine::{RenderEngine, EngineTrait as RenderEngineTrait};
use super::os::{OsApplication, ApplicationTrait as OsApplicationTrait};
pub struct Application <CoreApp> where CoreApp: ApplicationTrait {
    os_app: OsApplication<CoreApp>,
    render_engine: RenderEngine<CoreApp>,
   	core_app: CoreApp,
}
impl<CoreApp> Application<CoreApp> where CoreApp: ApplicationTrait {
    #[cfg(any(target_os = "linux", target_os = "windows"))]
	pub fn new() -> Self {
        Application {
            os_app: OsApplication::new(null()),
            render_engine: RenderEngine::new(),
            core_app: CoreApp::new(),
		}
	}
    #[cfg(target_os = "android")]
	pub fn new(
        activity: *mut super::android::activity::ANativeActivity,
        saved_state: *mut libc::c_void,
        saved_state_size: libc::size_t) -> Self {
        use super::android::application::Args;
        let args = Args {
            activity: activity,
            saved_state: saved_state,
            saved_state_size: saved_state_size,
        };
        Application {
            os_app: OsApplication::new(unsafe { transmute(&args) }),
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
