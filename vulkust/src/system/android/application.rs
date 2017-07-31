extern crate libc;
use std;
use std::mem::transmute;
use std::ptr::null_mut;
use self::libc::{c_void, size_t};
use super::super::super::core::application::ApplicationTrait;
use super::super::super::core::asset::manager::Manager as AssetManager;
use super::super::super::render::engine::{EngineTrait as RenderEngineTrait, RenderEngine};
use super::super::file::File;
use super::super::os::ApplicationTrait as OsApplicationTrait;
use super::activity::ANativeActivity;
use super::glue;
use super::glue::{AndroidApp, AndroidPollSource, AppCmd};
use super::window::ANativeWindow;
use super::looper::ALooper_pollAll;

#[derive(Clone, Copy)]
pub struct Args {
    pub activity: *mut ANativeActivity,
    pub saved_state: *mut c_void,
    pub saved_state_size: size_t,
}
pub struct Application<CoreApp>
where
    CoreApp: ApplicationTrait,
{
    pub asset_manager: AssetManager,
    pub core_app: *mut CoreApp,
    pub render_engine: *mut RenderEngine<CoreApp>,
    pub window: *mut ANativeWindow,
    pub window_initialized: bool,
    pub args: Args,
}

impl<CoreApp> OsApplicationTrait<CoreApp> for Application<CoreApp>
where
    CoreApp: ApplicationTrait,
{
    fn new(args: *const std::os::raw::c_void) -> Self {
        let args: *mut Args = unsafe { transmute(args) };
        let args = unsafe { (*args).clone() };
        let activity = args.activity;
        logi!("Creating: {:?}", activity);
        let app = Application {
            asset_manager: AssetManager::new(File::new(
                &"data.gx3d".to_string(),
                unsafe { (*activity).assetManager },
            )),
            core_app: null_mut(),
            render_engine: null_mut(),
            window: null_mut(),
            window_initialized: false,
            args: args,
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
    fn set_core_app(&mut self, c: *mut CoreApp) {
        self.core_app = c;
    }
    fn set_rnd_eng(&mut self, r: *mut RenderEngine<CoreApp>) {
        self.render_engine = r;
    }
    fn initialize(&mut self) -> bool {
        unsafe {
            (*self.args.activity).instance = glue::android_app_create::<CoreApp>(
                self.args.activity,
                self.args.saved_state,
                self.args.saved_state_size,
                transmute((*self.render_engine).os_app),
            )
        }
        self.asset_manager.initialize();
        return true;
    }
    fn execute(&mut self) -> bool {
        return true;
    }
}

impl<CoreApp> Application<CoreApp>
where
    CoreApp: ApplicationTrait,
{
    pub fn main(&mut self, android_app: *mut AndroidApp) {
        logi!("I'm in");
        unsafe {
            (*android_app).on_app_cmd = handle_cmd::<CoreApp>;
        }
        let mut events = 0 as libc::c_int;
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
unsafe extern "C" fn handle_cmd<CoreApp>(android_app: *mut AndroidApp, cmd: i32)
where
    CoreApp: ApplicationTrait,
{
    let app: *mut Application<CoreApp> = transmute((*android_app).user_data);
    (*app).handle_cmd(android_app, cmd);
}

impl<CoreApp> Drop for Application<CoreApp>
where
    CoreApp: ApplicationTrait,
{
    fn drop(&mut self) {
        loge!("Error unexpected deletion of Os Application.");
    }
}
