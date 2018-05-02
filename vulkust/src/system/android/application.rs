use super::super::super::core::application::ApplicationTrait as CoreAppTrait;
use super::super::super::core::event::{Button, Event, Keyboard, Mouse, Type as EventType, Window};
use super::super::super::libc::{c_int, c_void, size_t};
use super::super::application::Application as SysApp;
use super::activity::ANativeActivity;
use super::glue;
use super::glue::{AndroidApp, AndroidPollSource, AppCmd};
use super::looper::ALooper_pollAll;
use super::window::ANativeWindow;
use std;
use std::mem::transmute;
use std::ptr::null_mut;
use std::sync::{Arc, RwLock};

pub struct Application {
    pub window: Arc<RwLock<Option<*mut ANativeWindow>>>,
    activity: *mut ANativeActivity,
    saved_state: *mut c_void,
    saved_state_size: size_t,
    events: Arc<RwLock<Vec<Event>>>,
}

impl Application {
    pub fn new(
        activity: *mut ANativeActivity,
        saved_state: *mut c_void,
        saved_state_size: size_t,
    ) -> Self {
        let app = Application {
            window: Arc::new(RwLock::new(None)),
            activity,
            saved_state,
            saved_state_size,
            events: Arc::new(RwLock::new(Vec::new())),
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

    pub fn initialize(
        &self,
        itself: Arc<RwLock<Application>>,
        core_app: Arc<RwLock<CoreAppTrait>>,
    ) {
        unsafe {
            (*(self.activity)).instance = glue::android_app_create(
                self.activity,
                self.saved_state,
                self.saved_state_size,
                itself,
                core_app,
            );
        }
    }

    pub fn main(&self, android_app: *mut AndroidApp) {
        vxlogi!("I'm in");
        unsafe {
            (*android_app).on_app_cmd = handle_cmd;
        }
        let mut events = 0 as c_int;
        let mut source = 0 as *mut AndroidPollSource;
        while unsafe { (*android_app).destroy_requested } == 0 {
            if unsafe { ALooper_pollAll(-1, null_mut(), &mut events, transmute(&mut source)) } >= 0
            {
                if source != null_mut() {
                    unsafe {
                        ((*source).process)(android_app, source);
                    }
                }
                if vxresult!(self.window.read()).is_some() {
                    unsafe {
                        let sys_app = Arc::new(RwLock::new(SysApp::new(
                            vxunwrap!((*android_app).core_app).clone(),
                            vxunwrap!((*android_app).os_app).clone(),
                        )));
                        (*android_app).sys_app = Some(sys_app);
                    }
                    return;
                }
            }
        }
        vxloge!("Unexpected flow.");
    }

    fn handle_cmd(&self, app: *mut AndroidApp, cmd: i32) {
        match unsafe { transmute::<i8, AppCmd>(cmd as i8) } {
            AppCmd::InitWindow => {
                vxlogi!("Window has been shown!");
                *vxresult!(self.window.write()) = Some(unsafe { (*app).window });
            }
            AppCmd::TermWindow => {
                vxlogi!("Window has been terminated!");
            }
            c @ _ => {
                let _ = c;
                vxlogi!("event {:?} not handled in app {:?} ", c, app);
            }
        }
    }

    pub fn fetch_events(&self) -> Vec<Event> {
        let mut events = 0 as c_int;
        let mut source = 0 as *mut AndroidPollSource;
        let android_app: &'static mut glue::AndroidApp =
            unsafe { transmute((*self.activity).instance) };
        while android_app.destroy_requested == 0 && unsafe {
            ALooper_pollAll(0, null_mut(), &mut events, transmute(&mut source))
        } >= 0 && source != null_mut()
        {
            unsafe {
                ((*source).process)(android_app, source);
            }
        }
        let events = vxresult!(self.events.read()).clone();
        vxresult!(self.events.write()).clear();
        return events;
    }
}
unsafe extern "C" fn handle_cmd(android_app: *mut AndroidApp, cmd: i32) {
    vxresult!(vxunwrap!((*android_app).os_app).read()).handle_cmd(android_app, cmd);
}

impl Drop for Application {
    fn drop(&mut self) {
        vxloge!("Error unexpected deletion of Os Application.");
    }
}
