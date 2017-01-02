use std::ptr::{
    null_mut,
};
use std::mem::transmute;
use libc;
use super::looper::ALooper_pollAll;
use super::glue::{
    AppCmd,
    AndroidApp,
    AndroidPollSource,
};

#[repr(C)]
pub struct Application {
    window_initialized: bool,
}

impl Application {
    pub fn initialize(&mut self) {
        self.window_initialized = false;
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
    }

    pub fn handle_cmd(&mut self, app: *mut AndroidApp, cmd: i32) {
        match unsafe { transmute::<i8, AppCmd>(cmd as i8) } {
            AppCmd::InitWindow => {
                //            initialize(app);
                self.window_initialized = false;
                logdbg!("Window has been shown!");
            },
            AppCmd::TermWindow => {
                // terminate();
                logdbg!("Window has been terminated!");
            },
            c @ _ => {
                logdbg!(format!("event {:?} not handled in app {:?} ", c, app));
            }
        }
    }
}

unsafe extern fn handle_cmd(android_app: *mut AndroidApp, cmd: i32) {
    let app: *mut Application = transmute((*android_app).user_data);
    (*app).handle_cmd(android_app, cmd);
}