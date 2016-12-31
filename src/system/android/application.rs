use std;
use std::process::exit;
use std::thread;
use std::sync::{
    mpsc,
    Arc
};
use std::mem::transmute;
use libc;
use libc::{
    c_int,
    pipe,
    c_void,
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
    ALooper,
    ALooperCallbackFunc,
};
//use super::asset::{
//    AAssetManager,
//};
use super::input::{
    AInputQueue,
    AInputQueue_detachLooper,
};
use super::window::{
    ANativeWindow,
};
use super::config::{
    AConfiguration_new,
    AConfiguration_fromAssetManager,
};

pub struct Application {
    main_thread: thread::JoinHandle<()>,
    msg_read_fd: c_int,
    input_queue: *mut AInputQueue,
    pending_input_queue: *mut AInputQueue,
    looper: *mut ALooper,
    user_data: *mut c_void,
    on_app_cmd: fn (app: *mut Application, cmd: i32),

    // Fill this in with the function to process input events.  At this point
    // the event has already been pre-dispatched, and it will be finished upon
    // return.  Return 1 if you have handled the event, 0 for any default
    // dispatching.
    int32_t (*onInputEvent)(struct android_app* app, AInputEvent* event);

    // The ANativeActivity object instance that this app is running in.
    ANativeActivity* activity;

    // The current configuration the app is running in.
    AConfiguration* config;

    // This is the last instance's saved state, as provided at creation time.
    // It is NULL if there was no state.  You can use this as you need; the
    // memory will remain around until you call android_app_exec_cmd() for
    // APP_CMD_RESUME, at which point it will be freed and savedState set to NULL.
    // These variables should only be changed when processing a APP_CMD_SAVE_STATE,
    // at which point they will be initialized to NULL and you can malloc your
    // state and place the information here.  In that case the memory will be
    // freed for you later.
    void* savedState;
    size_t savedStateSize;

    // The ALooper associated with the app's thread.
    ALooper* looper;

    // When non-NULL, this is the input queue from which the app will
    // receive user input events.
    AInputQueue* inputQueue;

    // When non-NULL, this is the window surface that the app can draw in.
    ANativeWindow* window;

    // Current content rectangle of the window; this is the area where the
    // window's content should be placed to be seen by the user.
    ARect contentRect;

    // Current state of the app's activity.  May be either APP_CMD_START,
    // APP_CMD_RESUME, APP_CMD_PAUSE, or APP_CMD_STOP; see below.
    int activityState;

    // This is non-zero when the application's NativeActivity is being
    // destroyed and waiting for the app thread to complete.
    int destroyRequested;

    // -------------------------------------------------
    // Below are "private" implementation of the glue code.

    pthread_mutex_t mutex;
    pthread_cond_t cond;

    int msgread;
    int msgwrite;

    pthread_t thread;

    struct android_poll_source cmdPollSource;
    struct android_poll_source inputPollSource;

    int running;
    int stateSaved;
    int destroyed;
    int redrawNeeded;
    AInputQueue* pendingInputQueue;
    ANativeWindow* pendingWindow;
    ARect pendingContentRect;
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

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
enum AppCmd {
    InputChanged,
    InitWindow,
    TermWindow,
    WindowResized,
    WindowRedrawNeeded,
    ContentRectChanged,
    GainedFocus,
    LostFocus,
    ConfigChanged,
    LowMemory,
    Start,
    Resume,
    SaveState,
    Pause,
    Stop,
    Destroy,
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
            let mut cmd_poll_source = Box::new(AndroidPollSource {
                id: LooperId::Main,
                android_app: android_app,
                process: process_cmd,
            });
            let mut input_poll_source = Box::new(AndroidPollSource {
                id: LooperId::Input,
                android_app: android_app,
                process: process_input,
            });
            (*android_app).looper = unsafe {
                ALooper_prepare(ALooperPrepare::AllowNonCallbacks as c_int)
            };
            let mut pipe_fds = [0 as c_int, 2];
            (*android_app).msg_read_fd = pipe_fds[0];
            unsafe { pipe(pipe_fds.as_mut_ptr() as *mut c_int); }
            unsafe { ALooper_addFd(
                looper, pipe_fds[0], LooperId::Main as c_int, ALooperEvent::Input as c_int,
                transmute(0), transmute(&mut (*cmd_poll_source)));
            }
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

fn android_app_read_cmd(android_app: *mut Application) -> u8 {
    let mut cmd = 0u8;
    if read((*android_app).msgread, &mut cmd, 1) == 1 {
        match cmd as AppCmd {
            AppCmd::SaveState => {
                // TODO
                // free_saved_state(android_app);
            }
        }
        return cmd;
    } else {
        logftl!("No data on command pipe!");
    }
    return u8::max_value();
}

fn android_app_pre_exec_cmd(android_app: *mut Application, cmd: u8) {
    match cmd {
        AppCmd::InputChanged => {
            logdbg!("AppCmd::InputChanged");
            // pthread_mutex_lock(&android_app->mutex); !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            if (*android_app).input_queue != 0 as *mut AInputQueue {
                AInputQueue_detachLooper((*android_app).input_queue);
            }
            (*android_app).input_queue = (*android_app).pending_input_queue;
            if (*android_app).input_queue != 0 as *mut AInputQueue {
                logdbg!("Attaching input queue to looper");
                AInputQueue_attachLooper(
                    (*android_app).input_queue, (*android_app).looper, LooperId::Input, 0,
                    &mut android_app -> inputPollSource);
            }
            //            pthread_cond_broadcast(&android_app->cond);
            //            pthread_mutex_unlock(&android_app->mutex);
        },
        AppCmd::InitWindow => {
            logdbg!("APP_CMD_INIT_WINDOW");
//            pthread_mutex_lock(&android_app->mutex);!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            android_app->window = android_app->pendingWindow;
            pthread_cond_broadcast(&android_app->cond);
            pthread_mutex_unlock(&android_app->mutex);
            break;

            case APP_CMD_TERM_WINDOW:
            LOGV("APP_CMD_TERM_WINDOW\n");
            pthread_cond_broadcast(&android_app->cond);
            break;

            case APP_CMD_RESUME:
            case APP_CMD_START:
            case APP_CMD_PAUSE:
            case APP_CMD_STOP:
            LOGV("activityState=%d\n", cmd);
            pthread_mutex_lock(&android_app->mutex);
            android_app->activityState = cmd;
            pthread_cond_broadcast(&android_app->cond);
            pthread_mutex_unlock(&android_app->mutex);
            break;

            case APP_CMD_CONFIG_CHANGED:
            LOGV("APP_CMD_CONFIG_CHANGED\n");
            AConfiguration_fromAssetManager(android_app->config,
            android_app->activity->assetManager);
            print_cur_config(android_app);
            break;

            case APP_CMD_DESTROY:
            LOGV("APP_CMD_DESTROY\n");
            android_app->destroyRequested = 1;
            break;
            }
            }

fn process_cmd(android_app: *mut Application, source: *mut AndroidPollSource) {
    let cmd = android_app_read_cmd(app);
    android_app_pre_exec_cmd(app, cmd);
    if (app->onAppCmd != NULL) app->onAppCmd(app, cmd);
    android_app_post_exec_cmd(app, cmd);
}

fn process_input(android_app: *mut Application, source: *mut AndroidPollSource) {

}