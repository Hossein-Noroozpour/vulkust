use std;
use std::process::exit;
use std::thread;
use std::sync::{
    mpsc,
    Arc
};
use std::mem::transmute;
use libc::{
    c_int,
    pipe,
};
use super::super::super::core::application::{
    BasicApplication as CoreApp,
    Application as CoreAppTrait,
};
use super::super::application::Application as SysApp;
use super::activity::ANativeActivity;
use super::rect::{
    ARect,
};
use super::looper::{
    ALooper_prepare,
    ALooperPrepare,
    ALooper_addFd,
    ALooperEvent,
    ALooperCallbackFunc,
};
//use super::asset::{
//    AAssetManager,
//};
use super::input::{
    AInputQueue,
};
use super::window::{
    ANativeWindow,
};
use super::config::{
    AConfiguration_new,
    AConfiguration_fromAssetManager,
};

pub struct Application {
    pub main_thread: thread::JoinHandle<()>,
}

struct AndroidPollSource {
    id: LooperId,
    android_app: *mut Application,
    process: fn (android_app: *mut Application, source: *mut AndroidPollSource),
}
#[repr(i32)]
#[derive(Debug, Clone, Copy)]
enum LooperId {
    Main = 1,
    Input = 2,
    User = 3,
}

impl Application {
    pub fn on_start(&mut self, activity: *mut ANativeActivity) {
        logdbg!(format!("Activity {:?} on_start.", activity));
    }
    pub fn on_resume(&mut self, activity: *mut ANativeActivity) {
        logdbg!(format!("Activity {:?} on_resume.", activity));
    }
    pub fn on_save_instance_state(&mut self, activity: *mut ANativeActivity, size: *mut usize) {
        logdbg!(format!("Activity {:?}   {:?} on_save_instance_state.", activity, size));
    }
    pub fn on_pause(&mut self, activity: *mut ANativeActivity) {
        logdbg!(format!("Activity {:?} on_pause.", activity));
    }
    pub fn on_stop(&mut self, activity: *mut ANativeActivity) {
        logdbg!(format!("Activity {:?} on_stop.", activity));
    }
    pub fn on_destroy(&mut self, activity: *mut ANativeActivity) {
        logdbg!(format!("Activity {:?} on_destroy.", activity));
        exit(0);
    }
    pub fn on_window_focus_changed(&mut self, activity: *mut ANativeActivity, has_focus: i64) {
        logdbg!(format!("Activity {:?}   {:?} on_window_focus_changed.", activity, has_focus));
    }
    pub fn on_native_window_created(&mut self, activity: *mut ANativeActivity, window: *mut ANativeWindow) {
        logdbg!(format!("Activity {:?}   {:?} on_native_window_created.", activity, window));
    }
    pub fn on_native_window_resized(&mut self, activity: *mut ANativeActivity, window: *mut ANativeWindow) {
        logdbg!(format!("Activity {:?}   {:?} on_native_window_resized.", activity, window));
    }
    pub fn on_native_window_redraw_needed(&mut self, activity: *mut ANativeActivity, window: *mut ANativeWindow) {
        logdbg!(format!("Activity {:?}   {:?} on_native_window_redraw_needed.", activity, window));
    }
    pub fn on_native_window_destroyed(&mut self, activity: *mut ANativeActivity, window: *mut ANativeWindow) {
        logdbg!(format!("Activity {:?}   {:?} on_native_window_destroyed.", activity, window));
    }
    pub fn on_input_queue_created(&mut self, activity: *mut ANativeActivity, queue: *mut AInputQueue) {
        logdbg!(format!("Activity {:?}   {:?} on_input_queue_created.", activity, queue));
    }
    pub fn on_input_queue_destroyed(&mut self, activity: *mut ANativeActivity, queue: *mut AInputQueue) {
        logdbg!(format!("Activity {:?}   {:?} on_input_queue_destroyed.", activity, queue));
    }
    pub fn on_content_rect_changed(&mut self, activity: *mut ANativeActivity, rect: *const ARect) {
        logdbg!(format!("Activity {:?}   {:?} on_content_rect_changed.", activity, rect));
    }
    pub fn on_configuration_changed(&mut self, activity: *mut ANativeActivity) {
        logdbg!(format!("Activity {:?} on_configuration_changed.", activity));
    }
    pub fn on_low_memory(&mut self, activity: *mut ANativeActivity) {
        logdbg!(format!("Activity {:?} on_low_memory.", activity));
    }

    pub fn new(activity: *mut ANativeActivity) -> Self {
        let activity_copy: usize = unsafe {std::mem::transmute(activity) };
        let mut android_app = Application {
            main_thread: thread::spawn(move || {})
        };
        let android_app_copy: usize = unsafe {std::mem::transmute(&mut android_app) };
        let main_thread = thread::spawn(move || {
            logdbg!("In another thread");
            let activity: *mut ANativeActivity = unsafe {std::mem::transmute(activity_copy) };
            let android_app: *mut Application = unsafe {std::mem::transmute(android_app_copy) };
            let config = unsafe { AConfiguration_new() };
            unsafe { AConfiguration_fromAssetManager(config, (*activity).assetManager); }
            logdbg!(*config);
            let mut cmd_poll_source = AndroidPollSource {
                id: LooperId::Main,
                android_app: android_app,
                process: process_cmd,
            };
            let mut input_poll_source = AndroidPollSource {
                id: LooperId::Input,
                android_app: android_app,
                process: process_input,
            };
            let mut looper = unsafe { ALooper_prepare(ALooperPrepare::AllowNonCallbacks as c_int) };
            let mut pipe_fds = [0 as c_int, 2];
            unsafe { pipe(pipe_fds.as_mut_ptr() as *mut c_int); }
            unsafe { ALooper_addFd(
                looper, pipe_fds[0], LooperId::Main as c_int, ALooperEvent::Input as c_int,
                transmute(0), transmute(&mut cmd_poll_source))
            };
//            android_app -> looper = looper;
//            pthread_mutex_lock(& android_app -> mutex);
//            android_app -> running = 1;
//            pthread_cond_broadcast( & android_app -> cond);
//            pthread_mutex_unlock( & android_app -> mutex);
            let mut core_app = CoreApp::new();
            core_app.main();
        });
        android_app.main_thread = main_thread;
        android_app
    }
}

impl SysApp for Application {}

fn process_cmd(android_app: *mut Application, source: *mut AndroidPollSource) {

}

fn process_input(android_app: *mut Application, source: *mut AndroidPollSource) {

}