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
use super::application::Application;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AndroidPollSource {
    pub id: i32,
    pub app: *mut AndroidApp,
    pub process: unsafe extern fn(app: *mut AndroidApp, source: *mut AndroidPollSource),
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AndroidApp {
    pub user_data: *mut libc::c_void,
    pub on_app_cmd: unsafe extern fn(app: *mut AndroidApp, cmd: i32),
    pub on_input_event: unsafe extern fn(app: *mut AndroidApp, event: *mut input::AInputEvent) -> i32,
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
//    logerr!("locked");
    if (*android_app).saved_state != ptr::null_mut() {
        libc::free((*android_app).saved_state);
        (*android_app).saved_state = ptr::null_mut();
        (*android_app).saved_state_size = 0;
    }
    libc::pthread_mutex_unlock(&mut (*android_app).mutex);
//    logerr!("unlocked");
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
        logftl!("No data on command pipe!");
    }
}

unsafe extern fn android_app_pre_exec_cmd(android_app: *mut AndroidApp, cmd: AppCmd) {
    match cmd {
        AppCmd::InputChanged => {
            logdbg!("AppCmdInputChanged");
            libc::pthread_mutex_lock(&mut (*android_app).mutex);
//            logerr!("locked");
            if (*android_app).input_queue != ptr::null_mut() {
                input::AInputQueue_detachLooper((*android_app).input_queue);
            }
            (*android_app).input_queue = (*android_app).pending_input_queue;
            if (*android_app).input_queue != ptr::null_mut() {
                logdbg!("Attaching input queue to looper");
                input::AInputQueue_attachLooper(
                    (*android_app).input_queue, (*android_app).looper, LooperId::Input as i32,
                    transmute(0usize), transmute(&mut (*android_app).input_poll_source));
            }
            libc::pthread_cond_broadcast(&mut ((*android_app).cond) as *mut libc::pthread_cond_t);
            libc::pthread_mutex_unlock(&mut (*android_app).mutex);
//            logerr!("unlocked");
        },
        AppCmd::InitWindow => {
            logdbg!("AppCmdInitWindow");
            libc::pthread_mutex_lock(&mut (*android_app).mutex);
//            logerr!("locked");
            (*android_app).window = (*android_app).pending_window;
            libc::pthread_cond_broadcast(&mut ((*android_app).cond) as *mut libc::pthread_cond_t);
            libc::pthread_mutex_unlock(&mut (*android_app).mutex);
//            logerr!("unlocked");
        },
        AppCmd::TermWindow => {
            logdbg!("AppCmdTermWindow");
            libc::pthread_cond_broadcast(&mut ((*android_app).cond) as *mut libc::pthread_cond_t);
        },
        AppCmd::Resume | AppCmd::Start | AppCmd::Pause | AppCmd::Stop => {
            logdbg!(format!("activity_state = {:?}", cmd));
            libc::pthread_mutex_lock(&mut (*android_app).mutex);
//            logerr!("locked");
            (*android_app).activity_state = cmd as i32;
            libc::pthread_cond_broadcast(&mut ((*android_app).cond) as *mut libc::pthread_cond_t);
            libc::pthread_mutex_unlock(&mut (*android_app).mutex);
//            logerr!("unlocked");
        },
        AppCmd::ConfigChanged => {
            logdbg!(format!("AppCmdConfigChanged {:?}", *((*android_app).config)));
            config::AConfiguration_fromAssetManager(
                (*android_app).config, (*(*android_app).activity).assetManager);
        },
        AppCmd::Destroy => {
            logdbg!("AppCmdDestroy");
            (*android_app).destroy_requested = 1;
        },
        c @ _ => {
            logdbg!(format!("Unhandled value {:?}", c));
        },
    }
}

unsafe extern fn android_app_post_exec_cmd(android_app: *mut AndroidApp, cmd: AppCmd) {
    match cmd {
        AppCmd::TermWindow => {
            logdbg!("AppCmdTermWindow");
            libc::pthread_mutex_lock(&mut (*android_app).mutex);
//            logerr!("locked");
            (*android_app).window = ptr::null_mut();
            libc::pthread_cond_broadcast(&mut ((*android_app).cond) as *mut libc::pthread_cond_t);
            libc::pthread_mutex_unlock(&mut (*android_app).mutex);
//            logerr!("unlocked");
        },
        AppCmd::SaveState => {
            logdbg!("AppCmdSaveState");
            libc::pthread_mutex_lock(&mut (*android_app).mutex);
//            logerr!("locked");
            (*android_app).state_saved = 1;
            libc::pthread_cond_broadcast(&mut ((*android_app).cond) as *mut libc::pthread_cond_t);
            libc::pthread_mutex_unlock(&mut (*android_app).mutex);
//            logerr!("unlocked");
        },
        AppCmd::Resume => {
            free_saved_state(android_app);
        },
        _ => {
            logdbg!(format!("Unexpected value: {:?}", cmd));
        }
    }
}

unsafe extern fn android_app_destroy(android_app: *mut AndroidApp) {
    logdbg!("android_app_destroy!");
    free_saved_state(android_app);
    libc::pthread_mutex_lock(&mut (*android_app).mutex);
//    logerr!("locked");
    if (*android_app).input_queue != ptr::null_mut() {
        input::AInputQueue_detachLooper((*android_app).input_queue);
    }
    config::AConfiguration_delete((*android_app).config);
    (*android_app).destroyed = 1;
    libc::pthread_cond_broadcast(&mut ((*android_app).cond) as *mut libc::pthread_cond_t);
    libc::pthread_mutex_unlock(&mut (*android_app).mutex);
//    logerr!("unlocked");
}

unsafe extern fn process_input(app: *mut AndroidApp, source: *mut AndroidPollSource) {
    let _ = source;
    let mut event: *mut input::AInputEvent = ptr::null_mut();
    while input::AInputQueue_getEvent((*app).input_queue, &mut event) >= 0 {
        logdbg!(format!("New input event: type={:?}", input::AInputEvent_getType(event)));
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

extern fn android_app_entry(param: *mut libc::c_void) -> *mut libc::c_void {
    unsafe {
        let android_app: *mut AndroidApp = transmute(param);
        (*android_app).config = config::AConfiguration_new();
        config::AConfiguration_fromAssetManager(
            (*android_app).config, (*(*android_app).activity).assetManager);
        logdbg!("Configure is :");
        logdbg!(*(*android_app).config);
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
//        logerr!("locked");
        (*android_app).running = 1;
        libc::pthread_cond_broadcast(&mut ((*android_app).cond));
        libc::pthread_mutex_unlock(&mut (*android_app).mutex);
//        logerr!("unlocked");
        (*android_app).user_data = libc::malloc(size_of::<Application>());
        libc::memset((*android_app).user_data, 0, size_of::<Application>());
        let app: *mut Application = transmute((*android_app).user_data);
        (*app).initialize();
        (*app).main(android_app);
        // android_main(android_app);!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        android_app_destroy(android_app);
        ptr::null_mut()
    }
}

unsafe extern fn android_app_create(
    activity: *mut activity::ANativeActivity, saved_state: *mut libc::c_void,
    saved_state_size: libc::size_t) -> *mut libc::c_void {
    let mut android_app: *mut AndroidApp = transmute(libc::malloc(size_of::<AndroidApp>()));
    libc::memset(transmute(android_app), 0, size_of::<AndroidApp>());
    (*android_app).activity = activity;
    libc::pthread_mutex_init(&mut (*android_app).mutex, ptr::null_mut());
    libc::pthread_cond_init(&mut (*android_app).cond, ptr::null_mut());
    if saved_state != ptr::null_mut() {
        (*android_app).saved_state = libc::malloc(saved_state_size);
        (*android_app).saved_state_size = saved_state_size;
        libc::memcpy((*android_app).saved_state, saved_state, saved_state_size);
    }
    let mut msg_pipe_fds = [0 as libc::c_int, 2];
    if libc::pipe(msg_pipe_fds.as_mut_ptr()) != 0 {
        logftl!("Could not create pipe!");
    }
    (*android_app).msg_read_fd = msg_pipe_fds[0];
    (*android_app).msg_write_fd = msg_pipe_fds[1];
    let mut attr: libc::pthread_attr_t = zeroed();
    libc::pthread_attr_init(&mut attr);
    libc::pthread_attr_setdetachstate(&mut attr, libc::PTHREAD_CREATE_DETACHED);
    libc::pthread_create(&mut (*android_app).thread, &attr, android_app_entry, transmute(android_app));
    libc::pthread_mutex_lock(&mut (*android_app).mutex);
//    logerr!("locked");
    while (*android_app).running != 1 {
        libc::pthread_cond_wait(&mut (*android_app).cond, &mut (*android_app).mutex);
    }
    libc::pthread_mutex_unlock(&mut (*android_app).mutex);
//    logerr!("unlocked");
    return transmute(android_app);
}

unsafe extern fn android_app_write_cmd(android_app: *mut AndroidApp, cmd: i8) {
    if libc::write((*android_app).msg_write_fd, transmute(&cmd), 1) != 1 {
        logftl!("Failure writing AndroidApp cmd!");
    }
}

unsafe extern fn android_app_set_input(android_app: *mut AndroidApp, input_queue: *mut input::AInputQueue) {
    libc::pthread_mutex_lock(&mut (*android_app).mutex);
    (*android_app).pending_input_queue = input_queue;
//    logerr!("locked!");
    android_app_write_cmd(android_app, AppCmd::InputChanged as i8);
    while (*android_app).input_queue != (*android_app).pending_input_queue {
        libc::pthread_cond_wait(&mut (*android_app).cond, &mut (*android_app).mutex);
    }
    libc::pthread_mutex_unlock(&mut (*android_app).mutex);
//    logerr!("unlocked!");
}

unsafe extern fn android_app_set_window(android_app: *mut AndroidApp, window: *mut window::ANativeWindow) {
    libc::pthread_mutex_lock(&mut (*android_app).mutex);
//    logerr!("locked");
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
//    logerr!("unlocked");
}

unsafe extern fn android_app_set_activity_state(android_app: *mut AndroidApp, cmd: i8) {
//    logerr!("locked");
    libc::pthread_mutex_lock(&mut (*android_app).mutex);
    android_app_write_cmd(android_app, cmd);
    while (*android_app).activity_state != cmd as i32 {
        libc::pthread_cond_wait(&mut (*android_app).cond, &mut (*android_app).mutex);
    }
    libc::pthread_mutex_unlock(&mut (*android_app).mutex);
//    logerr!("unlocked");
}

unsafe extern fn android_app_free(android_app: *mut AndroidApp) {
    libc::pthread_mutex_lock(&mut (*android_app).mutex);
//    logerr!("locked");
    android_app_write_cmd(android_app, AppCmd::Destroy as i8);
    while (*android_app).destroyed == 0 {
        libc::pthread_cond_wait(&mut (*android_app).cond, &mut (*android_app).mutex);
    }
    libc::pthread_mutex_unlock(&mut (*android_app).mutex);
//    logerr!("unlocked");
    libc::close((*android_app).msg_read_fd);
    libc::close((*android_app).msg_write_fd);
    libc::pthread_cond_destroy(&mut (*android_app).cond);
    libc::pthread_mutex_destroy(&mut (*android_app).mutex);
    libc::free(transmute(android_app));
}

unsafe extern fn on_destroy(activity: *mut activity::ANativeActivity) {
    logdbg!(format!("Destroy: {:?}", activity));
    android_app_free(transmute((*activity).instance));
}

unsafe extern fn on_start(activity: *mut activity::ANativeActivity) {
    logdbg!(format!("Start: {:?}", activity));
    android_app_set_activity_state(transmute((*activity).instance), AppCmd::Start as i8);
}

unsafe extern fn on_resume(activity: *mut activity::ANativeActivity) {
    logdbg!(format!("Resume: {:?}", activity));
    android_app_set_activity_state(transmute((*activity).instance), AppCmd::Resume as i8);
}

unsafe extern fn on_save_instance_state(
    activity: *mut activity::ANativeActivity, out_len: *mut libc::size_t) -> *mut libc::c_void {
    let mut android_app: *mut AndroidApp = transmute((*activity).instance);
    let mut saved_state: *mut libc::c_void = ptr::null_mut();
    logdbg!(format!("SaveInstanceState: {:?}", activity));
    libc::pthread_mutex_lock(&mut (*android_app).mutex);
//    logerr!("locked");
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
//    logerr!("unlocked");
    return saved_state;
}

unsafe extern fn on_pause(activity: *mut activity::ANativeActivity) {
    logdbg!(format!("Pause: {:?}", activity));
    android_app_set_activity_state(transmute((*activity).instance), AppCmd::Pause as i8);
}

unsafe extern fn on_stop(activity: *mut activity::ANativeActivity) {
    logdbg!(format!("Stop: {:?}", activity));
    android_app_set_activity_state(transmute((*activity).instance), AppCmd::Stop as i8);
}

unsafe extern fn on_configuration_changed(activity: *mut activity::ANativeActivity) {
    let android_app: *mut AndroidApp = transmute((*activity).instance);
    logdbg!(format!("ConfigurationChanged: {:?}", activity));
    android_app_write_cmd(android_app, AppCmd::ConfigChanged as i8);
}

unsafe extern fn on_low_memory(activity: *mut activity::ANativeActivity) {
    let android_app: *mut AndroidApp = transmute((*activity).instance);
    logdbg!(format!("LowMemory: {:?}", activity));
    android_app_write_cmd(android_app, AppCmd::LowMemory as i8);
}

unsafe extern fn on_window_focus_changed(
    activity: *mut activity::ANativeActivity, focused: libc::c_int) {
    logdbg!(format!("WindowFocusChanged: {:?} -- {:?}", activity, focused));
    android_app_write_cmd(transmute((*activity).instance), if focused != 0 {
        AppCmd::GainedFocus
    } else { AppCmd::LostFocus } as i8);
}

unsafe extern fn on_native_window_created(
    activity: *mut activity::ANativeActivity, window: *mut window::ANativeWindow) {
    logdbg!(format!("NativeWindowCreated: {:?} -- {:?}", activity, window));
    android_app_set_window(transmute((*activity).instance), window);
}

unsafe extern fn on_native_window_destroyed(
    activity: *mut activity::ANativeActivity, window: *mut window::ANativeWindow) {
    logdbg!(format!("NativeWindowDestroyed: {:?} -- {:?}", activity, window));
    android_app_set_window(transmute((*activity).instance), ptr::null_mut());
}

unsafe extern fn on_input_queue_created(
    activity: *mut activity::ANativeActivity, queue: *mut input::AInputQueue) {
    logdbg!(format!("InputQueueCreated: {:?} -- {:?}", activity, queue));
    android_app_set_input(transmute((*activity).instance), queue);
}

unsafe extern fn on_input_queue_destroyed(
    activity: *mut activity::ANativeActivity, queue: *mut input::AInputQueue) {
    logdbg!(format!("InputQueueDestroyed: {:?} -- {:?}", activity, queue));
    android_app_set_input(transmute((*activity).instance), ptr::null_mut());
}

#[allow(dead_code, non_snake_case)]
#[no_mangle]
pub unsafe extern fn ANativeActivity_onCreate(
    activity: *mut activity::ANativeActivity, saved_state: *mut libc::c_void,
    saved_state_size: libc::size_t) {
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