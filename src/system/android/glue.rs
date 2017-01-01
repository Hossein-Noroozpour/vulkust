use std::thread::JoinHandle;
use std::sync::{
    Mutex,
    Condvar,
};
use std::ptr;
use std::mem::{
    transmute,
    zeroed,
    size_of,
};
use libc;
use super::config;
use super::looper;
use super::activity;
use super::input;
use super::window;
use super::rect;

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct AndroidPollSource {
    pub id: i32,
    pub app: *mut AndroidApp,
    pub process: unsafe extern fn (app: *mut AndroidApp, source: *mut AndroidPollSource),
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
struct AndroidApp {
    pub user_data: *mut libc::c_void,
    pub on_app_cmd: unsafe extern fn (app: *mut AndroidApp, cmd: i32),
    pub on_input_event: unsafe extern fn (app: *mut AndroidApp, event: *mut AInputEvent) -> i32,
    pub activity: *mut activity::ANativeActivity,
    pub config: *mut config::AConfiguration,
    pub saved_state: *mut libc::c_void,
    pub saved_state_size: libc::size_t,
    pub looper: *mut looper::ALooper,
    pub input_queue: *mut input::AInputQueue,
    pub window: *mut ANativeWindow,
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
    if (*android_app).saved_state != ptr::null_mut() {
        libc::free((*android_app).saved_state);
        (*android_app).saved_state = ptr::null_mut();
        (*android_app).saved_state_size = 0;
    }
    libc::pthread_mutex_lock(&mut (*android_app).mutex);
}

unsafe extern fn android_app_read_cmd(android_app: *mut AndroidApp) -> i8 {
    let mut cmd = 0i8;
    if libc::read((*android_app).msg_read_fd, &mut cmd as *mut libc::c_void, 1) == 1 {
        let cmd: AppCmd = unsafe { transmute(cmd) };
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
        logftl!("No data on command pipe!");
    }
    return AppCmd::InternalError as i8;
}

unsafe extern fn android_app_pre_exec_cmd(android_app: *mut AndroidApp, cmd: AppCmd) {
    match cmd {
        AppCmd::InputChanged => {
            logdbg!("AppCmdInputChanged");
            libc::pthread_mutex_lock(&mut (*android_app).mutex);
            if (*android_app).input_queue != ptr::null_mut() {
                AInputQueue_detachLooper((*android_app).input_queue);
            }
            (*android_app).input_queue = (*android_app).pending_input_queue;
            if (*android_app).input_queue != ptr::null_mut() {
                logdbg!("Attaching input queue to looper");
                AInputQueue_attachLooper(
                    (*android_app).input_queue, (*android_app).looper, LooperId::Input,
                    ptr::null_mut(), &mut (*android_app).input_poll_source as libc::c_void);
            }
            libc::pthread_cond_broadcast(&mut ((*android_app).cond) as *mut libc::pthread_cond_t);
            libc::pthread_mutex_unlock(&mut (*android_app).mutex);
        },
        AppCmd::InitWindow => {
            logdbg!("AppCmdInitWindow");
            libc::pthread_mutex_lock(&mut (*android_app).mutex);
            (*android_app).window = (*android_app).pending_window;
            libc::pthread_cond_broadcast(&mut ((*android_app).cond) as *mut libc::pthread_cond_t);
            libc::pthread_mutex_unlock(&mut (*android_app).mutex);
        },
        AppCmd::TermWindow => {
            logdbg!("AppCmdTermWindow");
            libc::pthread_cond_broadcast(&mut ((*android_app).cond) as *mut libc::pthread_cond_t);
        },
        AppCmd::Resume | AppCmd::Start | AppCmd::Pause | AppCmd::Stop => {
            logdbg!(format!("activity_state = {:?}", cmd));
            libc::pthread_mutex_lock(&mut (*android_app).mutex);
            (*android_app).activity_state = cmd;
            libc::pthread_cond_broadcast(&mut ((*android_app).cond) as *mut libc::pthread_cond_t);
            libc::pthread_mutex_unlock(&mut (*android_app).mutex);
        },
        AppCmd::ConfigChanged => {
            logdbg!(format!("AppCmdConfigChanged", *((*android_app).config)));
            AConfiguration_fromAssetManager((*android_app).config, (*android_app).activity.assetManager);
        },
        AppCmd::Destroy => {
            logdbg!("AppCmdDestroy");
            (*android_app).destroy_requested = 1;
        },
        _ => {
            logftl!("Unexpected behaviour!");
        },
    }
}

unsafe extern fn android_app_post_exec_cmd(android_app: *mut AndroidApp, cmd: AppCmd) {
    match cmd {
        AppCmd::TermWindow => {
            logdbg!("AppCmdTermWindow");
            libc::pthread_mutex_lock(&mut (*android_app).mutex);
            (*android_app).window = ptr::null_mut();
            libc::pthread_cond_broadcast(&mut ((*android_app).cond) as *mut libc::pthread_cond_t);
            libc::pthread_mutex_unlock(&mut (*android_app).mutex);
        },
        AppCmd::SaveState => {
            logdbg!("AppCmdSaveState");
            libc::pthread_mutex_lock(&mut (*android_app).mutex);
            (*android_app).state_saved = 1;
            libc::pthread_cond_broadcast(&mut ((*android_app).cond) as *mut libc::pthread_cond_t);
            libc::pthread_mutex_unlock(&mut (*android_app).mutex);
        },
        AppCmd::Resume => {
            free_saved_state(android_app);
        },
    }
}

unsafe extern fn android_app_destroy(android_app: *mut AndroidApp) {
    logdbg!("android_app_destroy!");
    free_saved_state(android_app);
    libc::pthread_mutex_lock(&mut (*android_app).mutex);
    if (*android_app).input_queue != ptr::null_mut() {
        AInputQueue_detachLooper((*android_app).input_queue);
    }
    AConfiguration_delete((*android_app).config);
    (*android_app).destroyed = 1;
    libc::pthread_cond_broadcast(&mut ((*android_app).cond) as *mut libc::pthread_cond_t);
    libc::pthread_mutex_unlock(&mut (*android_app).mutex);
}

unsafe extern fn process_input(app: *mut AndroidApp, source: *mut AndroidPollSource) {
    let mut event: *mut input::AInputEvent = ptr::null_mut();
    while AInputQueue_getEvent((*app).input_queue, &mut event as *mut *mut input::AInputEvent) >= 0 {
        logdbg!("New input event: type=%d\n", AInputEvent_getType(event));
        if AInputQueue_preDispatchEvent((*app).input_queue, event) {
            continue;
        }
        let mut handled = 0 as libc::c_int;
        if (*app).on_input_event != ptr::null_mut() {
            handled = (*app).on_input_event(app, event);
        }
        AInputQueue_finishEvent((*app).input_queue, event, handled);
    }
}

unsafe extern fn process_cmd(app: *mut AndroidApp, source: *mut AndroidPollSource) {
    let cmd = android_app_read_cmd(app);
    android_app_pre_exec_cmd(app, cmd);
    if (*app).on_app_cmd != ptr::null() {
        (*app).on_app_cmd(app, cmd);
    }
    android_app_post_exec_cmd(app, cmd);
}

unsafe extern fn android_app_entry(param: *mut libc::c_void) -> *mut libc::c_void {
    let android_app: *mut AndroidApp = transmute(param);
    (*android_app).config = AConfiguration_new();
    AConfiguration_fromAssetManager((*android_app).config, (*(*android_app).activity).assetManager);
    logdbg!(format!("Configure is : {:?}", (*(*android_app).config)));
    (*android_app).cmd_poll_source.id = LooperId::Main;
    (*android_app).cmd_poll_source.app = android_app;
    (*android_app).cmd_poll_source.process = process_cmd;
    (*android_app).input_poll_source.id = LooperId::Input;
    (*android_app).input_poll_source.app = android_app;
    (*android_app).input_poll_source.process = process_input;
    (*android_app).looper = looper::ALooper_prepare(looper::ALooperPrepare::AllowNonCallbacks);
    looper::ALooper_addFd(
        (*android_app).looper, (*android_app).msg_read_fd, LooperId::Main,
        looper::ALooperEvent::Input, ptr::null_mut(),
        &mut (*android_app).cmd_poll_source as *mut libc::c_void);
    libc::pthread_mutex_lock(&mut (*android_app).mutex);
    (*android_app).running = 1;
    libc::pthread_cond_broadcast(&mut ((*android_app).cond) as *mut libc::pthread_cond_t);
    libc::pthread_mutex_unlock(&mut (*android_app).mutex);
    // android_main(android_app);!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    android_app_destroy(android_app);
    ptr::null_mut()
}

unsafe extern fn android_app_create(
    activity: *mut ANativeActivity, saved_state: *mut libc::c_void,
    saved_state_size: libc::size_t) -> *mut libc::c_void {
    let mut android_app: *mut AndroidApp = transmute(libc::malloc(size_of::<AndroidApp>()));
    libc::memset(android_app, 0, size_of::<AndroidApp>());
    android_app.activity = activity;
    libc::pthread_mutex_init(&mut (*android_app).mutex, ptr::null_mut());
    libc::pthread_cond_init(&mut (*android_app).cond, ptr::null_mut());
    if saved_state != ptr::null_mut() {
        android_app.saved_state = libc::malloc(saved_state_size);
        android_app.saved_state_size = saved_state_size;
        libc::memcpy(android_app.saved_state, saved_state, saved_state_size);
    }
    let mut msg_pipe_fds = [0 as libc::c_int, 2];
    if libc::pipe(msg_pipe_fds.as_mut_ptr()) {
        logftl!("Could not create pipe!");
        return ptr::null_mut();
    }
    android_app.msg_read_fd = msg_pipe_fds[0];
    android_app.msg_write_fd = msg_pipe_fds[1];
    let mut attr: libc::pthread_attr_t = unsafe { zeroed() };
    libc::pthread_attr_init(&mut attr);
    libc::pthread_attr_setdetachstate(&mut attr, libc::PTHREAD_CREATE_DETACHED);
    libc::pthread_create(&mut android_app.thread, &attr, android_app_entry, android_app);
    libc::pthread_mutex_lock(&mut (*android_app).mutex);
    while (*android_app).running != 1 {
        libc::pthread_cond_wait(&(*android_app).cond, &(*android_app).mutex);
    }
    libc::pthread_mutex_unlock(&(*android_app).mutex);
    return android_app;
}

unsafe extern fn android_app_write_cmd(android_app: *mut AndroidApp, cmd: i8) {
    if libc::write((*android_app).msg_write_fd, &cmd, 1) != 1 {
        logftl!("Failure writing AndroidApp cmd!");
    }
}

unsafe extern fn android_app_set_input(android_app: *mut AndroidApp, input_queue: *mut input::AInputQueue) {
    libc::pthread_mutex_lock(&(*android_app).mutex);
    (*android_app).pending_input_queue = input_queue;
    android_app_write_cmd(android_app, AppCmd::InputChanged);
    while (*android_app).input_queue != (*android_app).pending_input_queue {
        libc::pthread_cond_wait(&(*android_app).cond, &(*android_app).mutex);
    }
    libc::pthread_mutex_unlock(&(*android_app).mutex);
}

unsafe extern fn android_app_set_window(android_app: *mut AndroidApp, window: *mut ANativeWindow) {
    libc::pthread_mutex_lock(&(*android_app).mutex);
    if (*android_app).pending_window != ptr::null_mut() {
        android_app_write_cmd(android_app, AppCmd::TermWindow);
    }
    (*android_app).pending_window = window;
    if window != ptr::null_mut() {
        android_app_write_cmd(android_app, AppCmd::InitWindow);
    }
    while (*android_app).window != (*android_app).pending_window {
        libc::pthread_cond_wait(&(*android_app).cond, &(*android_app).mutex);
    }
    libc::pthread_mutex_unlock(&(*android_app).mutex);
}

unsafe extern fn android_app_set_activity_state(android_app: *mut AndroidApp, cmd: i8) {
    libc::pthread_mutex_lock(&(*android_app).mutex);
    android_app_write_cmd(android_app, cmd);
    while (*android_app).activityState != cmd {
        libc::pthread_cond_wait(&(*android_app).cond, &(*android_app).mutex);
    }
    libc::pthread_mutex_unlock(&(*android_app).mutex);
}

unsafe extern fn android_app_free(android_app: *mut AndroidApp) {
    libc::pthread_mutex_lock(&(*android_app).mutex);
    android_app_write_cmd(android_app, AppCmd::Destroy);
    while (*android_app).destroyed == 0 {
        libc::pthread_cond_wait(&(*android_app).cond, &(*android_app).mutex);
    }
    libc::pthread_mutex_unlock(&(*android_app).mutex);
    libc::close((*android_app).msg_read_fd);
    libc::close((*android_app).msg_write_fd);
    libc::pthread_cond_destroy(&(*android_app).cond);
    libc::pthread_mutex_destroy(&(*android_app).mutex);
    libc::free(android_app);
}

unsafe extern fn on_destroy(activity: *mut ANativeActivity) {
    logdbg!(format!("Destroy: {:?}", activity));
    android_app_free(transmute((*activity).instance));
}

unsafe extern fn on_start(activity: *mut ANativeActivity) {
    logdbg!(format!("Start: {:?}", activity));
    android_app_set_activity_state(transmute((*activity).instance), AppCmd::Start);
}

unsafe extern fn on_resume(activity: *mut ANativeActivity) {
    logdbg!(format!("Resume: {:?}", activity));
    android_app_set_activity_state(transmute((*activity).instance), AppCmd::Resume);
}

unsafe extern fn on_save_instance_state(activity: *mut ANativeActivity, out_len: *mut libc::size_t) -> *mut libc::c_void {
    let mut android_app = transmute((*activity).instance);
    let mut saved_state: *mut libc::c_void = ptr::null_mut();
    logdbg!("SaveInstanceState: {:?}", activity);
    libc::pthread_mutex_lock(&(*android_app).mutex);
    (*android_app).state_saved = 0;
    android_app_write_cmd(android_app, AppCmd::SaveState);
    while (*android_app).state_saved == 0 {
        libc::pthread_cond_wait(&(*android_app).cond, &(*android_app).mutex);
    }
    if (*android_app).saved_state != ptr::null_mut() {
        saved_state = (*android_app).saved_state;
        *out_len = (*android_app).saved_state_size;
        (*android_app).saved_state = ptr::null_mut();
        (*android_app).saved_state_size = 0;
    }
    libc::pthread_mutex_unlock(&(*android_app).mutex);
    return saved_state;
}

unsafe extern fn on_pause(activity: *mut ANativeActivity) {
    logdbg!(format!("Pause: {:?}", activity));
    android_app_set_activity_state(transmute((*activity).instance), AppCmd::Pause);
}

unsafe extern fn on_stop(activity: *mut ANativeActivity) {
    logdbg!(format!("Stop: {:?}", activity));
    android_app_set_activity_state(transmute((*activity).instance), AppCmd::Stop);
}

unsafe extern fn on_configuration_changed(activity: *mut ANativeActivity) {
    let mut android_app: *mut AndroidApp = transmute((*activity).instance);
    logdbg!(format!("ConfigurationChanged: {:?}", activity));
    android_app_write_cmd(android_app, AppCmd::ConfigChanged);
}

unsafe extern fn on_low_memory(activity: *mut ANativeActivity) {
    let mut android_app: *mut AndroidApp = transmute((*activity).instance);
    logdbg!(format!("LowMemory: {:?}", activity));
    android_app_write_cmd(android_app, AppCmd::LowMemory);
}
unsafe extern fn on_window_focus_changed(activity: *mut ANativeActivity, focused: libc::c_int) {
    logdbg!(format!("WindowFocusChanged: {:?} -- {:?}", activity, focused));
    android_app_write_cmd(transmute((*activity).instance), if focused != 0  {
        AppCmd::GainedFocus } else { AppCmd::LostFocus });
}
unsafe extern fn on_native_window_created(activity: *mut ANativeActivity, window: *mut ANativeWindow) {
    logdbg!(format!("NativeWindowCreated: {:?} -- {:?}", activity, window));
    android_app_set_window(transmute((*activity).instance), window);
}

unsafe extern fn on_native_window_destroyed(activity: *mut ANativeActivity, window: *mut ANativeWindow) {
    logdbg!(format!("NativeWindowDestroyed: {:?} -- {:?}", activity, window));
    android_app_set_window(transmute((*activity).instance), ptr::null_mut());
}

unsafe extern fn on_input_queue_created(activity: *mut ANativeActivity, queue: *mut AInputQueue) {
    logdbg!(format!("InputQueueCreated: {:?} -- {:?}", activity, queue));
    android_app_set_input(transmute((*activity).instance), queue);
}

unsafe extern fn on_input_queue_destroyed(activity: *mut ANativeActivity, queue: *mut AInputQueue) {
    logdbg!(format!("InputQueueDestroyed: {:?} -- {:?}", activity, queue));
    android_app_set_input(transmute((*activity).instance), ptr::null_mut());
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern fn ANativeActivity_onCreate(activity: *mut ANativeActivity, saved_state: *mut libc::c_void, saved_state_size: libc::size_t) {
    logdbg!(format!("Creating: {:?}", activity));
    (*(*activity).callbacks).onDestroy = on_destroy;
    (*(*activity).callbacks).onStart = on_start;
    (*(*activity).callbacks).onResume = on_resume;
    (*(*activity).callbacks).onSaveInstanceState = on_save_instance_state;
    (*(*activity).callbacks).onPause = on_pause;
    (*(*activity).callbacks).onStop = on_stop;
    (*(*activity).callbacks).onConfigurationChanged = on_configuration_changed;
    (*(*activity).callbacks).onLowMemory = on_low_memory;
    (*(*activity).callbacks).onWindowFocusChanged = on_window_focus_changed;
    (*(*activity).callbacks).onNativeWindowCreated = on_native_window_created;
    (*(*activity).callbacks).onNativeWindowDestroyed = on_native_window_destroyed;
    (*(*activity).callbacks).onInputQueueCreated = on_input_queue_created;
    (*(*activity).callbacks).onInputQueueDestroyed = on_input_queue_destroyed;

    (*activity).instance = android_app_create(activity, saved_state, saved_state_size);
}