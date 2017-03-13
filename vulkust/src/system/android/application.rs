use std::ptr::{
    null_mut,
};
use std::mem::transmute;
use libc;
use super::looper::ALooper_pollAll;
use super::super::super::vulkan::surface::Surface;
use super::super::super::core::application::Application as CoreApp;
use super::glue::{
    AppCmd,
    AndroidApp,
    AndroidPollSource,
};

pub struct Application {
    window_initialized: bool,
    core_app: CoreApp,
}

impl Application {
    pub fn new() -> Self {
        Application {
            window_initialized: false,
            core_app: CoreApp::new(),
        }
    }
    pub fn main(&mut self, android_app: *mut AndroidApp) {
        logdbg!("I'm in");
        unsafe { (*android_app).on_app_cmd = handle_cmd; }
        let mut events = 0 as libc::c_int;
        let mut source = 0 as *mut AndroidPollSource;
        while unsafe { (*android_app).destroy_requested } == 0 {
            if unsafe { ALooper_pollAll(
                if self.window_initialized { 1 } else { 0 }, null_mut(),
                &mut events, transmute(&mut source)) } >= 0 {
                if source != null_mut() {
                    unsafe { ((*source).process)(android_app, source); }
                }
            }
        }
        logftl!("unexpected");
    }

    fn handle_cmd(&mut self, app: *mut AndroidApp, cmd: i32) {
        match unsafe { transmute::<i8, AppCmd>(cmd as i8) } {
            AppCmd::InitWindow => {
                self.window_initialized = true;
                let surface = Surface::new(
                    self.core_app.vulkan_driver.instance.clone(), unsafe{(*app).window});
                self.core_app.initialize(surface);
                logdbg!("Window has been shown!");
            },
            AppCmd::TermWindow => {
                self.core_app.terminate();
                logdbg!("Window has been terminated!");
            },
            c @ _ => {
                #[cfg(not(debug_assertions))]
                let _ = c;
                logdbg!(format!("event {:?} not handled in app {:?} ", c, app));
            }
        }
    }
}

unsafe extern fn handle_cmd(android_app: *mut AndroidApp, cmd: i32) {
    let app: *mut Application = transmute((*android_app).user_data);
    (*app).handle_cmd(android_app, cmd);
}