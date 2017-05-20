#[cfg(target_os = "android")]
extern crate libc;
use std::ptr::null;
use std::mem::{
    transmute,
    forget,
};
use std::os::raw::c_void as std_void;
#[cfg(target_os = "android")]
use self::libc::{
    c_void,
    size_t,
};
use super::super::core::application::ApplicationTrait;
use super::super::render::engine::{RenderEngine, EngineTrait as RenderEngineTrait};
use super::os::{OsApplication, ApplicationTrait as OsApplicationTrait};
pub struct Application <CoreApp> where CoreApp: ApplicationTrait {
    os_app: *mut OsApplication<CoreApp>,
    render_engine: *mut RenderEngine<CoreApp>,
   	core_app: *mut CoreApp,
}
impl<CoreApp> Application<CoreApp> where CoreApp: ApplicationTrait {
    #[cfg(any(target_os = "linux", target_os = "windows"))]
	pub fn new() -> Self {
        Self::set(null())
	}
    #[cfg(target_os = "android")]
	pub fn new(
        activity: *mut super::android::activity::ANativeActivity,
        saved_state: *mut libc::c_void,
        saved_state_size: libc::size_t) {
        use super::android::application::Args;
        let args = Args {
            activity: activity,
            saved_state: saved_state,
            saved_state_size: saved_state_size,
        };
        let _ = Self::set(unsafe { transmute(&args) });
	}

    fn set(args: *const std_void) -> Self {
        let mut os_app = Box::into_raw(Box::new(OsApplication::<CoreApp>::new(args)));
        loge!("Reached {:?}", os_app);
        
        let mut render_engine = Box::into_raw(Box::new(RenderEngine::<CoreApp>::new()));
        let mut core_app = Box::into_raw(Box::new(CoreApp::new()));
        unsafe { (*os_app).set_core_app(core_app)};
        unsafe { (*os_app).set_rnd_eng(render_engine)};
        unsafe { (*render_engine).set_os_app(os_app)};
        unsafe { (*render_engine).set_core_app(core_app)};
        Application {
            os_app: os_app,
            render_engine: render_engine,
            core_app: core_app,
		}
    }

    #[cfg(any(target_os = "linux", target_os = "windows"))]
    pub fn run(&mut self) {
        let pt1: usize = unsafe { transmute(self.render_engine)};
        let pt2: usize = unsafe { transmute((*self.os_app).render_engine)};
        logi!("p1 = {}, p2 = {}", pt1, pt2);
        unsafe { (*self.render_engine).initialize() };
        unsafe { (*self.core_app).initialize(self.os_app, self.render_engine) };
        unsafe { (*self.os_app).execute() };
        unsafe { (*self.core_app).terminate() };
        unsafe { (*self.render_engine).terminate() };
    }
}
#[cfg(any(target_os = "linux", target_os = "windows"))]
impl<CoreApp> Drop for Application<CoreApp> where CoreApp: ApplicationTrait {
    fn drop(&mut self) {
        logi!("Main system application got deleted.");
        unsafe { Box::from_raw(self.core_app); }
        unsafe { Box::from_raw(self.render_engine); }
        unsafe { Box::from_raw(self.os_app); }
    }
}
