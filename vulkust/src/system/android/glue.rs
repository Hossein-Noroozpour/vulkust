extern crate libc;
use std::ptr;
use std::mem::{
    transmute,
    zeroed,
    size_of,
};
use super::super::super::core::application::ApplicationTrait as CoreAppTrait;
use super::config;
use super::looper;
use super::activity;
use super::input;
use super::window;
use super::rect;
use super::application::Application;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct AndroidPollSource {
    pub id: i32,
    pub app: *mut AndroidApp,
    pub process: unsafe extern fn(app: *mut AndroidApp, source: *mut AndroidPollSource),
}

impl Drop for AndroidPollSource {
    fn drop(&mut self) {
        loge!("Unexpected deletion of AndroidPollSource struct.");
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct AndroidApp {
    pub user_data: *mut libc::c_void,
    pub on_app_cmd: unsafe extern fn(app: *mut AndroidApp, cmd: i32),
    pub on_input_event: unsafe extern fn(
        app: *mut AndroidApp, event: *mut input::AInputEvent) -> i32,
    pub activity: *mut activity::ANativeActivity,
    pub config: *mut config::AConfiguration,
    pub saved_state: *mut libc::c_void,
    pub saved_state_size: libc::size_t,
    pub looper: *mut looper::ALooper,
    pub input_queue: *mut input::AInputQueue,
    pub window: *mut window::ANativeWindow,
    pub content_rect: rect::ARect,
    pub activity_state: libc::c_int,
    pub destroy_requested: libc::c_int,
    pub mutex: libc::pthread_mutex_t,
    pub cond: libc::pthread_cond_t,
    pub msg_read_fd: libc::c_int,
    pub msg_write_fd: libc::c_int,
    pub thread: libc::pthread_t,
    pub cmd_poll_source: AndroidPollSource,
    pub input_poll_source: AndroidPollSource,
    pub running: libc::c_int,
    pub state_saved: libc::c_int,
    pub destroyed: libc::c_int,
    pub redraw_needed: libc::c_int,
    pub pending_input_queue: *mut input::AInputQueue,
    pub pending_window: *mut window::ANativeWindow,
    pub pending_content_rect: rect::ARect,
}

impl Drop for AndroidApp {
    fn drop(&mut self) {
        loge!("Unexpected deletion of AndroidApp struct.");
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum LooperId {
    Main = 1,
    Input = 2,
    User = 3,
}

#[repr(i8)]
#[derive(Debug, Clone, Copy)]
pub enum AppCmd {
    InputChanged = 1,
    InitWindow = 2,
    TermWindow = 3,
    WindowResized = 4,
    WindowRedrawNeeded = 5,
    ContentRectChanged = 6,
    GainedFocus = 7,
    LostFocus = 8,
    ConfigChanged = 9,
    LowMemory = 10,
    Start = 11,
    Resume = 12,
    SaveState = 13,
    Pause = 14,
    Stop = 15,
    Destroy = 16,
    InternalError = -1,
}

unsafe extern fn free_saved_state(android_app: *mut AndroidApp) {
    libc::pthread_mutex_lock(&mut (*android_app).mutex);
//    loge!("locked");
    if (*android_app).saved_state != ptr::null_mut() {
        libc::free((*android_app).saved_state);
        (*android_app).saved_state = ptr::null_mut();
        (*android_app).saved_state_size = 0;
    }
    libc::pthread_mutex_unlock(&mut (*android_app).mutex);
//    loge!("unlocked");
}

unsafe extern fn android_app_read_cmd(android_app: *mut AndroidApp) -> i8 {
    let mut cmd = 0i8;
    if libc::read((*android_app).msg_read_fd, transmute(&mut cmd), 1) == 1 {
        let cmd: AppCmd = transmute(cmd);
        match cmd {
            AppCmd::SaveState => {
                free_saved_state(android_app);
            },
            _ => {
                return cmd as i8;
            }
        }
        return cmd as i8;
    } else {
        loge!("No data on command pipe!");
    }
    0
}

unsafe extern fn android_app_pre_exec_cmd(android_app: *mut AndroidApp, cmd: AppCmd) {
    match cmd {
        AppCmd::InputChanged => {
            logi!("AppCmdInputChanged");
            libc::pthread_mutex_lock(&mut (*android_app).mutex);
//            loge!("locked");
            if (*android_app).input_queue != ptr::null_mut() {
                input::AInputQueue_detachLooper((*android_app).input_queue);
            }
            (*android_app).input_queue = (*android_app).pending_input_queue;
            if (*android_app).input_queue != ptr::null_mut() {
                logi!("Attaching input queue to looper");
                input::AInputQueue_attachLooper(
                    (*android_app).input_queue, (*android_app).looper, LooperId::Input as i32,
                    transmute(0usize), transmute(&mut (*android_app).input_poll_source));
            }
            libc::pthread_cond_broadcast(&mut ((*android_app).cond) as *mut libc::pthread_cond_t);
            libc::pthread_mutex_unlock(&mut (*android_app).mutex);
//            loge!("unlocked");
        },
        AppCmd::InitWindow => {
            logi!("AppCmdInitWindow");
            libc::pthread_mutex_lock(&mut (*android_app).mutex);
//            loge!("locked");
            (*android_app).window = (*android_app).pending_window;
            libc::pthread_cond_broadcast(&mut ((*android_app).cond) as *mut libc::pthread_cond_t);
            libc::pthread_mutex_unlock(&mut (*android_app).mutex);
//            loge!("unlocked");
        },
        AppCmd::TermWindow => {
            logi!("AppCmdTermWindow");
            libc::pthread_cond_broadcast(&mut ((*android_app).cond) as *mut libc::pthread_cond_t);
        },
        AppCmd::Resume | AppCmd::Start | AppCmd::Pause | AppCmd::Stop => {
            logi!("activity_state = {:?}", cmd);
            libc::pthread_mutex_lock(&mut (*android_app).mutex);
//            loge!("locked");
            (*android_app).activity_state = cmd as i32;
            libc::pthread_cond_broadcast(&mut ((*android_app).cond) as *mut libc::pthread_cond_t);
            libc::pthread_mutex_unlock(&mut (*android_app).mutex);
//            loge!("unlocked");
        },
        AppCmd::ConfigChanged => {
            logi!("AppCmdConfigChanged {:?}", *((*android_app).config));
            config::AConfiguration_fromAssetManager(
                (*android_app).config, (*(*android_app).activity).assetManager);
        },
        AppCmd::Destroy => {
            logi!("AppCmdDestroy");
            (*android_app).destroy_requested = 1;
        },
        c @ _ => {
            #[cfg(not(debug_assertions))]
            let _ = c;
            logi!("Unhandled value {:?}", c);
        },
    }
}

unsafe extern fn android_app_post_exec_cmd(android_app: *mut AndroidApp, cmd: AppCmd) {
    match cmd {
        AppCmd::TermWindow => {
            logi!("AppCmdTermWindow");
            libc::pthread_mutex_lock(&mut (*android_app).mutex);
//            loge!("locked");
            (*android_app).window = ptr::null_mut();
            libc::pthread_cond_broadcast(&mut ((*android_app).cond) as *mut libc::pthread_cond_t);
            libc::pthread_mutex_unlock(&mut (*android_app).mutex);
//            loge!("unlocked");
        },
        AppCmd::SaveState => {
            logi!("AppCmdSaveState");
            libc::pthread_mutex_lock(&mut (*android_app).mutex);
//            loge!("locked");
            (*android_app).state_saved = 1;
            libc::pthread_cond_broadcast(&mut ((*android_app).cond) as *mut libc::pthread_cond_t);
            libc::pthread_mutex_unlock(&mut (*android_app).mutex);
//            loge!("unlocked");
        },
        AppCmd::Resume => {
            free_saved_state(android_app);
        },
        _ => {
            logi!("Unexpected value: {:?}", cmd);
        }
    }
}

unsafe extern fn android_app_destroy(android_app: *mut AndroidApp) {
    logi!("android_app_destroy!");
    free_saved_state(android_app);
    libc::pthread_mutex_lock(&mut (*android_app).mutex);
//    loge!("locked");
    if (*android_app).input_queue != ptr::null_mut() {
        input::AInputQueue_detachLooper((*android_app).input_queue);
    }
    config::AConfiguration_delete((*android_app).config);
    (*android_app).destroyed = 1;
    libc::pthread_cond_broadcast(&mut ((*android_app).cond) as *mut libc::pthread_cond_t);
    libc::pthread_mutex_unlock(&mut (*android_app).mutex);
//    loge!("unlocked");
}

unsafe extern fn process_input(app: *mut AndroidApp, source: *mut AndroidPollSource) {
    let _ = source;
    let mut event: *mut input::AInputEvent = ptr::null_mut();
    while input::AInputQueue_getEvent((*app).input_queue, &mut event) >= 0 {
        logi!("New input event: type={:?}", input::AInputEvent_getType(event));
        if input::AInputQueue_preDispatchEvent((*app).input_queue, event) != 0 {
            continue;
        }
        let mut handled = 0 as libc::c_int;
        if (*app).on_input_event != transmute(0usize) {
            handled = ((*app).on_input_event)(app, event);
        }
        input::AInputQueue_finishEvent((*app).input_queue, event, handled);
    }
}

unsafe extern fn process_cmd(app: *mut AndroidApp, source: *mut AndroidPollSource) {
    let _ = source;
    let cmd = android_app_read_cmd(app);
    android_app_pre_exec_cmd(app, transmute(cmd));
    if (*app).on_app_cmd != transmute(0usize) {
        ((*app).on_app_cmd)(app, cmd as i32);
    }
    android_app_post_exec_cmd(app, transmute(cmd));
}

extern fn android_app_entry<CoreApp>(param: *mut libc::c_void) -> *mut libc::c_void
        where CoreApp: CoreAppTrait {
    unsafe {
        let android_app: *mut AndroidApp = transmute(param);
        (*android_app).config = config::AConfiguration_new();
        config::AConfiguration_fromAssetManager(
            (*android_app).config, (*(*android_app).activity).assetManager);
        logi!("Configure is: {:?}", *((*android_app).config));
        (*android_app).cmd_poll_source.id = LooperId::Main as i32;
        (*android_app).cmd_poll_source.app = android_app;
        (*android_app).cmd_poll_source.process = process_cmd;
        (*android_app).input_poll_source.id = LooperId::Input as i32;
        (*android_app).input_poll_source.app = android_app;
        (*android_app).input_poll_source.process = process_input;
        (*android_app).looper = looper::ALooper_prepare(
            looper::ALooperPrepare::AllowNonCallbacks as i32);
        looper::ALooper_addFd(
            (*android_app).looper, (*android_app).msg_read_fd, LooperId::Main as i32,
            looper::ALooperEvent::Input as i32, transmute(0usize),
            transmute(&mut (*android_app).cmd_poll_source));
        libc::pthread_mutex_lock(&mut (*android_app).mutex);
        (*android_app).running = 1;
        libc::pthread_cond_broadcast(&mut ((*android_app).cond));
        libc::pthread_mutex_unlock(&mut (*android_app).mutex);
        {
            let app: *mut Application<CoreApp> = transmute((*android_app).user_data);
            (*app).main(android_app);
        }
        android_app_destroy(android_app);
        ptr::null_mut()
    }
}

pub unsafe extern fn android_app_create<CoreApp>(
    activity: *mut activity::ANativeActivity, saved_state: *mut libc::c_void,
    saved_state_size: libc::size_t, user_data: *mut libc::c_void) -> *mut libc::c_void
    where CoreApp: CoreAppTrait {
    let mut android_app: *mut AndroidApp = transmute(libc::malloc(size_of::<AndroidApp>()));
    libc::memset(transmute(android_app), 0, size_of::<AndroidApp>());
    (*android_app).activity = activity;
    (*android_app).user_data = user_data;
    libc::pthread_mutex_init(&mut (*android_app).mutex, ptr::null_mut());
    libc::pthread_cond_init(&mut (*android_app).cond, ptr::null_mut());
    if saved_state != ptr::null_mut() {
        (*android_app).saved_state = libc::malloc(saved_state_size);
        (*android_app).saved_state_size = saved_state_size;
        libc::memcpy((*android_app).saved_state, saved_state, saved_state_size);
    }
    let mut msg_pipe_fds = [0 as libc::c_int, 2];
    if libc::pipe(msg_pipe_fds.as_mut_ptr()) != 0 {
        logf!("Could not create pipe!");
    }
    (*android_app).msg_read_fd = msg_pipe_fds[0];
    (*android_app).msg_write_fd = msg_pipe_fds[1];
    let mut attr: libc::pthread_attr_t = zeroed();
    libc::pthread_attr_init(&mut attr);
    libc::pthread_attr_setdetachstate(&mut attr, libc::PTHREAD_CREATE_DETACHED);
    libc::pthread_create(
        &mut (*android_app).thread, &attr, android_app_entry::<CoreApp>, transmute(android_app));
    libc::pthread_mutex_lock(&mut (*android_app).mutex);
//    loge!("locked");
    while (*android_app).running != 1 {
        libc::pthread_cond_wait(&mut (*android_app).cond, &mut (*android_app).mutex);
    }
    libc::pthread_mutex_unlock(&mut (*android_app).mutex);
//    loge!("unlocked");
    return transmute(android_app);
}

unsafe extern fn android_app_write_cmd(android_app: *mut AndroidApp, cmd: i8) {
    if libc::write((*android_app).msg_write_fd, transmute(&cmd), 1) != 1 {
        logf!("Failure writing AndroidApp cmd!");
    }
}

unsafe extern fn android_app_set_input(
    android_app: *mut AndroidApp, input_queue: *mut input::AInputQueue) {
    libc::pthread_mutex_lock(&mut (*android_app).mutex);
    (*android_app).pending_input_queue = input_queue;
//    loge!("locked!");
    android_app_write_cmd(android_app, AppCmd::InputChanged as i8);
    while (*android_app).input_queue != (*android_app).pending_input_queue {
        libc::pthread_cond_wait(&mut (*android_app).cond, &mut (*android_app).mutex);
    }
    libc::pthread_mutex_unlock(&mut (*android_app).mutex);
//    loge!("unlocked!");
}

unsafe extern fn android_app_set_window(
    android_app: *mut AndroidApp, window: *mut window::ANativeWindow) {
    libc::pthread_mutex_lock(&mut (*android_app).mutex);
//    loge!("locked");
    if (*android_app).pending_window != ptr::null_mut() {
        android_app_write_cmd(android_app, AppCmd::TermWindow as i8);
    }
    (*android_app).pending_window = window;
    if window != ptr::null_mut() {
        android_app_write_cmd(android_app, AppCmd::InitWindow as i8);
    }
    while (*android_app).window != (*android_app).pending_window {
        libc::pthread_cond_wait(&mut (*android_app).cond, &mut (*android_app).mutex);
    }
    libc::pthread_mutex_unlock(&mut (*android_app).mutex);
//    loge!("unlocked");
}

unsafe extern fn android_app_set_activity_state(android_app: *mut AndroidApp, cmd: i8) {
//    loge!("locked");
    libc::pthread_mutex_lock(&mut (*android_app).mutex);
    android_app_write_cmd(android_app, cmd);
    while (*android_app).activity_state != cmd as i32 {
        libc::pthread_cond_wait(&mut (*android_app).cond, &mut (*android_app).mutex);
    }
    libc::pthread_mutex_unlock(&mut (*android_app).mutex);
//    loge!("unlocked");
}

unsafe extern fn android_app_free(android_app: *mut AndroidApp) {
    libc::pthread_mutex_lock(&mut (*android_app).mutex);
//    loge!("locked");
    android_app_write_cmd(android_app, AppCmd::Destroy as i8);
    while (*android_app).destroyed == 0 {
        libc::pthread_cond_wait(&mut (*android_app).cond, &mut (*android_app).mutex);
    }
    libc::pthread_mutex_unlock(&mut (*android_app).mutex);
//    loge!("unlocked");
    libc::close((*android_app).msg_read_fd);
    libc::close((*android_app).msg_write_fd);
    libc::pthread_cond_destroy(&mut (*android_app).cond);
    libc::pthread_mutex_destroy(&mut (*android_app).mutex);
    libc::free(transmute(android_app));
}

pub unsafe extern fn on_destroy(activity: *mut activity::ANativeActivity) {
    logi!("Destroy: {:?}", activity);
    android_app_free(transmute((*activity).instance));
}

pub unsafe extern fn on_start(activity: *mut activity::ANativeActivity) {
    logi!("Start: {:?}", activity);
    android_app_set_activity_state(transmute((*activity).instance), AppCmd::Start as i8);
}

pub unsafe extern fn on_resume(activity: *mut activity::ANativeActivity) {
    logi!("Resume: {:?}", activity);
    android_app_set_activity_state(transmute((*activity).instance), AppCmd::Resume as i8);
}

pub unsafe extern fn on_save_instance_state(
    activity: *mut activity::ANativeActivity, out_len: *mut libc::size_t) -> *mut libc::c_void {
    let mut android_app: *mut AndroidApp = transmute((*activity).instance);
    let mut saved_state: *mut libc::c_void = ptr::null_mut();
    logi!("SaveInstanceState: {:?}", activity);
    libc::pthread_mutex_lock(&mut (*android_app).mutex);
//    loge!("locked");
    (*android_app).state_saved = 0;
    android_app_write_cmd(android_app, AppCmd::SaveState as i8);
    while (*android_app).state_saved == 0 {
        libc::pthread_cond_wait(&mut (*android_app).cond, &mut (*android_app).mutex);
    }
    if (*android_app).saved_state != ptr::null_mut() {
        saved_state = (*android_app).saved_state;
        *out_len = (*android_app).saved_state_size;
        (*android_app).saved_state = ptr::null_mut();
        (*android_app).saved_state_size = 0;
    }
    libc::pthread_mutex_unlock(&mut (*android_app).mutex);
//    loge!("unlocked");
    return saved_state;
}

pub unsafe extern fn on_pause(activity: *mut activity::ANativeActivity) {
    logi!("Pause: {:?}", activity);
    android_app_set_activity_state(transmute((*activity).instance), AppCmd::Pause as i8);
}

pub unsafe extern fn on_stop(activity: *mut activity::ANativeActivity) {
    logi!("Stop: {:?}", activity);
    android_app_set_activity_state(transmute((*activity).instance), AppCmd::Stop as i8);
}

pub unsafe extern fn on_configuration_changed(activity: *mut activity::ANativeActivity) {
    let android_app: *mut AndroidApp = transmute((*activity).instance);
    logi!("ConfigurationChanged: {:?}", activity);
    android_app_write_cmd(android_app, AppCmd::ConfigChanged as i8);
}

pub unsafe extern fn on_low_memory(activity: *mut activity::ANativeActivity) {
    let android_app: *mut AndroidApp = transmute((*activity).instance);
    logi!("LowMemory: {:?}", activity);
    android_app_write_cmd(android_app, AppCmd::LowMemory as i8);
}

pub unsafe extern fn on_window_focus_changed(
    activity: *mut activity::ANativeActivity, focused: libc::c_int) {
    logi!("WindowFocusChanged: {:?} -- {:?}", activity, focused);
    android_app_write_cmd(transmute((*activity).instance), if focused != 0 {
        AppCmd::GainedFocus
    } else { AppCmd::LostFocus } as i8);
}

pub unsafe extern fn on_native_window_created(
    activity: *mut activity::ANativeActivity, window: *mut window::ANativeWindow) {
    logi!("NativeWindowCreated: {:?} -- {:?}", activity, window);
    android_app_set_window(transmute((*activity).instance), window);
}

pub unsafe extern fn on_native_window_destroyed(
    activity: *mut activity::ANativeActivity, window: *mut window::ANativeWindow) {
    #[cfg(not(debug_assertions))]
    let _ = window;
    logi!("NativeWindowDestroyed: {:?} -- {:?}", activity, window);
    android_app_set_window(transmute((*activity).instance), ptr::null_mut());
}

pub unsafe extern fn on_input_queue_created(
    activity: *mut activity::ANativeActivity, queue: *mut input::AInputQueue) {
    logi!("InputQueueCreated: {:?} -- {:?}", activity, queue);
    android_app_set_input(transmute((*activity).instance), queue);
}

pub unsafe extern fn on_input_queue_destroyed(
    activity: *mut activity::ANativeActivity, queue: *mut input::AInputQueue) {
    #[cfg(not(debug_assertions))]
    let _ = queue;
    logi!("InputQueueDestroyed: {:?} -- {:?}", activity, queue);
    android_app_set_input(transmute((*activity).instance), ptr::null_mut());
}
