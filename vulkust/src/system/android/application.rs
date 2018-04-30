use super::super::super::core::application::ApplicationTrait;
use super::super::super::libc::{c_int, c_void, size_t};
use super::activity::ANativeActivity;
use super::glue;
use super::glue::{AndroidApp, AndroidPollSource, AppCmd};
use super::looper::ALooper_pollAll;
use super::window::ANativeWindow;
use std;
use std::mem::transmute;
use std::ptr::null_mut;

pub struct Application {
    pub window: *mut ANativeWindow,
    pub window_initialized: bool,
    activity: *mut ANativeActivity,
    saved_state: *mut c_void,
    saved_state_size: size_t,
}

impl Application {
    fn new(
        activity: *mut ANativeActivity,
        saved_state: *mut c_void,
        saved_state_size: size_t,
    ) -> Self {
        vxlogi!("Creating: {:?}", activity);
        let app = Application {
            window: null_mut(),
            window_initialized: false,
            activity,
            saved_state,
            saved_state_size,
        };
        unsafe {
            (*(*activity).callbacks).onDestroy = glue::on_destroy;
            (*(*activity).callbacks).onStart = glue::on_start;
            (*(*activity).callbacks).onResume = glue::on_resume;
            (*(*activity).callbacks).onSaveInstanceState = glue::on_save_instance_state;
            (*(*activity).callbacks).onPause = glue::on_pause;
            (*(*activity).callbacks).onStop = glue::on_stop;
            (*(*activity).callbacks).onConfigurationChanged = glue::on_configuration_changed;
            (*(*activity).callbacks).onLowMemory = glue::on_low_memory;
            (*(*activity).callbacks).onWindowFocusChanged = glue::on_window_focus_changed;
            (*(*activity).callbacks).onNativeWindowCreated = glue::on_native_window_created;
            (*(*activity).callbacks).onNativeWindowDestroyed = glue::on_native_window_destroyed;
            (*(*activity).callbacks).onInputQueueCreated = glue::on_input_queue_created;
            (*(*activity).callbacks).onInputQueueDestroyed = glue::on_input_queue_destroyed;
        }
        app
    }

    fn initialize(&mut self) -> bool {
        unsafe {
            (*activity).instance =
                glue::android_app_create(activity, saved_state, saved_state_size);
        }
        self.asset_manager.initialize();
        return true;
    }

    pub fn main(&mut self, android_app: *mut AndroidApp) {
        logi!("I'm in");
        unsafe {
            (*android_app).on_app_cmd = handle_cmd;
        }
        let mut events = 0 as c_int;
        let mut source = 0 as *mut AndroidPollSource;
        while unsafe { (*android_app).destroy_requested } == 0 {
            if unsafe {
                ALooper_pollAll(
                    if self.window_initialized { 0 } else { 0 },
                    null_mut(),
                    &mut events,
                    transmute(&mut source),
                )
            } >= 0
            {
                if source != null_mut() {
                    unsafe {
                        ((*source).process)(android_app, source);
                    }
                }
                if self.window_initialized {
                    unsafe {
                        (*self.render_engine).update();
                    }
                }
            }
        }
        loge!("Unexpected flow.");
    }
    fn handle_cmd(&mut self, app: *mut AndroidApp, cmd: i32) {
        match unsafe { transmute::<i8, AppCmd>(cmd as i8) } {
            AppCmd::InitWindow => {
                logi!("Window has been shown!");
                self.window_initialized = true;
                self.window = unsafe { (*app).window };
                loge!("{:?}", self.window);
                unsafe { (*self.render_engine).initialize() };
                unsafe {
                    (*self.core_app).initialize((*self.render_engine).os_app, self.render_engine)
                };
            }
            AppCmd::TermWindow => {
                logi!("Window has been terminated!");
            }
            c @ _ => {
                let _ = c;
                logi!("event {:?} not handled in app {:?} ", c, app);
            }
        }
    }
}
unsafe extern "C" fn handle_cmd(android_app: *mut AndroidApp, cmd: i32) {
    let app: *mut Application = transmute((*android_app).user_data);
    (*app).handle_cmd(android_app, cmd);
}

impl Drop for Application {
    fn drop(&mut self) {
        vxloge!("Error unexpected deletion of Os Application.");
    }
}
