use super::super::super::core::application::ApplicationTrait as CoreAppTrait;
use super::super::super::libc;
use super::super::super::render::engine::Engine as RenderEngine;
use super::super::os::application::Application as OsApp;
use super::activity;
use super::config;
use super::input;
use super::looper;
use super::rect;
use super::window;
use std::mem::{size_of, transmute, zeroed};
use std::ptr;
use std::sync::{Arc, RwLock};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct AndroidPollSource {
    pub id: i32,
    pub app: *mut AndroidApp,
    pub process: unsafe extern "C" fn(app: *mut AndroidApp, source: *mut AndroidPollSource),
}

impl Drop for AndroidPollSource {
    fn drop(&mut self) {
        vxloge!("Unexpected deletion of AndroidPollSource struct.");
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct AndroidApp {
    pub on_app_cmd: extern fn(app: *mut AndroidApp, cmd: i32),
    pub on_input_event:
        unsafe extern "C" fn(app: *mut AndroidApp, event: *mut input::AInputEvent) -> i32,
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
    pub os_app: Option<Arc<RwLock<OsApp>>>,
}

impl Drop for AndroidApp {
    fn drop(&mut self) {
        vxloge!("Unexpected deletion of AndroidApp struct.");
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

unsafe extern "C" fn free_saved_state(android_app: *mut AndroidApp) {
    libc::pthread_mutex_lock(&mut (*android_app).mutex);
    //    vxloge!("locked");
    if (*android_app).saved_state != ptr::null_mut() {
        libc::free((*android_app).saved_state);
        (*android_app).saved_state = ptr::null_mut();
        (*android_app).saved_state_size = 0;
    }
    libc::pthread_mutex_unlock(&mut (*android_app).mutex);
    //    vxloge!("unlocked");
}

unsafe extern "C" fn android_app_read_cmd(android_app: *mut AndroidApp) -> i8 {
    let mut cmd = 0i8;
    if libc::read((*android_app).msg_read_fd, transmute(&mut cmd), 1) == 1 {
        let cmd: AppCmd = transmute(cmd);
        match cmd {
            AppCmd::SaveState => {
                free_saved_state(android_app);
            }
            _ => {
                return cmd as i8;
            }
        }
        return cmd as i8;
    } else {
        vxloge!("No data on command pipe!");
    }
    0
}

unsafe extern "C" fn android_app_pre_exec_cmd(android_app: *mut AndroidApp, cmd: AppCmd) {
    match cmd {
        AppCmd::InputChanged => {
            vxlogi!("AppCmdInputChanged");
            libc::pthread_mutex_lock(&mut (*android_app).mutex);
            //            vxloge!("locked");
            if (*android_app).input_queue != ptr::null_mut() {
                input::AInputQueue_detachLooper((*android_app).input_queue);
            }
            (*android_app).input_queue = (*android_app).pending_input_queue;
            if (*android_app).input_queue != ptr::null_mut() {
                vxlogi!("Attaching input queue to looper");
                input::AInputQueue_attachLooper(
                    (*android_app).input_queue,
                    (*android_app).looper,
                    LooperId::Input as i32,
                    transmute(0usize),
                    transmute(&mut (*android_app).input_poll_source),
                );
            }
            libc::pthread_cond_broadcast(&mut ((*android_app).cond) as *mut libc::pthread_cond_t);
            libc::pthread_mutex_unlock(&mut (*android_app).mutex);
            //            vxloge!("unlocked");
        }
        AppCmd::InitWindow => {
            vxlogi!("AppCmdInitWindow");
            libc::pthread_mutex_lock(&mut (*android_app).mutex);
            //            vxloge!("locked");
            (*android_app).window = (*android_app).pending_window;
            libc::pthread_cond_broadcast(&mut ((*android_app).cond) as *mut libc::pthread_cond_t);
            libc::pthread_mutex_unlock(&mut (*android_app).mutex);
            //            vxloge!("unlocked");
        }
        AppCmd::TermWindow => {
            vxlogi!("AppCmdTermWindow");
            libc::pthread_cond_broadcast(&mut ((*android_app).cond) as *mut libc::pthread_cond_t);
        }
        AppCmd::Resume | AppCmd::Start | AppCmd::Pause | AppCmd::Stop => {
            vxlogi!("activity_state = {:?}", cmd);
            libc::pthread_mutex_lock(&mut (*android_app).mutex);
            //            vxloge!("locked");
            (*android_app).activity_state = cmd as i32;
            libc::pthread_cond_broadcast(&mut ((*android_app).cond) as *mut libc::pthread_cond_t);
            libc::pthread_mutex_unlock(&mut (*android_app).mutex);
            //            vxloge!("unlocked");
        }
        AppCmd::ConfigChanged => {
            vxlogi!("AppCmdConfigChanged {:?}", *((*android_app).config));
            config::AConfiguration_fromAssetManager(
                (*android_app).config,
                (*(*android_app).activity).assetManager,
            );
        }
        AppCmd::Destroy => {
            vxlogi!("AppCmdDestroy");
            (*android_app).destroy_requested = 1;
        }
        c @ _ => {
            vxlogi!("Unhandled value {:?}", c);
        }
    }
}

unsafe extern "C" fn android_app_post_exec_cmd(android_app: *mut AndroidApp, cmd: AppCmd) {
    match cmd {
        AppCmd::TermWindow => {
            vxlogi!("AppCmdTermWindow");
            libc::pthread_mutex_lock(&mut (*android_app).mutex);
            //            vxloge!("locked");
            (*android_app).window = ptr::null_mut();
            libc::pthread_cond_broadcast(&mut ((*android_app).cond) as *mut libc::pthread_cond_t);
            libc::pthread_mutex_unlock(&mut (*android_app).mutex);
            //            vxloge!("unlocked");
        }
        AppCmd::SaveState => {
            vxlogi!("AppCmdSaveState");
            libc::pthread_mutex_lock(&mut (*android_app).mutex);
            //            vxloge!("locked");
            (*android_app).state_saved = 1;
            libc::pthread_cond_broadcast(&mut ((*android_app).cond) as *mut libc::pthread_cond_t);
            libc::pthread_mutex_unlock(&mut (*android_app).mutex);
            //            vxloge!("unlocked");
        }
        AppCmd::Resume => {
            free_saved_state(android_app);
        }
        _ => {
            vxlogi!("Unexpected value: {:?}", cmd);
        }
    }
}

unsafe extern "C" fn android_app_destroy(android_app: *mut AndroidApp) {
    vxlogi!("android_app_destroy!");
    free_saved_state(android_app);
    libc::pthread_mutex_lock(&mut (*android_app).mutex);
    //    vxloge!("locked");
    if (*android_app).input_queue != ptr::null_mut() {
        input::AInputQueue_detachLooper((*android_app).input_queue);
    }
    config::AConfiguration_delete((*android_app).config);
    (*android_app).destroyed = 1;
    libc::pthread_cond_broadcast(&mut ((*android_app).cond) as *mut libc::pthread_cond_t);
    libc::pthread_mutex_unlock(&mut (*android_app).mutex);
    //    vxloge!("unlocked");
}

unsafe extern "C" fn process_input(app: *mut AndroidApp, source: *mut AndroidPollSource) {
    let _ = source;
    let mut event: *mut input::AInputEvent = ptr::null_mut();
    while input::AInputQueue_getEvent((*app).input_queue, &mut event) >= 0 {
        vxlogi!(
            "New input event: type={:?}",
            input::AInputEvent_getType(event)
        );
        if input::AInputQueue_preDispatchEvent((*app).input_queue, event) != 0 {
            continue;
        }
        let mut handled = 0 as libc::c_int;
        if (*app).on_input_event != transmute(0usize) {
            handled = ((*app).on_input_event)(app, event);
            vxloge!("Reached");
        }
        input::AInputQueue_finishEvent((*app).input_queue, event, handled);
        vxloge!("Reached");
    }
    vxloge!("Reached");
}

unsafe extern "C" fn process_cmd(app: *mut AndroidApp, source: *mut AndroidPollSource) {
    let _ = source;
    let cmd = android_app_read_cmd(app);
    android_app_pre_exec_cmd(app, transmute(cmd));
    if (*app).on_app_cmd != transmute(0usize) {
        ((*app).on_app_cmd)(app, cmd as i32);
    }
    android_app_post_exec_cmd(app, transmute(cmd));
}

extern "C" fn android_app_entry(param: *mut libc::c_void) -> *mut libc::c_void {
    unsafe {
        let android_app: *mut AndroidApp = transmute(param);
        (*android_app).config = config::AConfiguration_new();
        config::AConfiguration_fromAssetManager(
            (*android_app).config,
            (*(*android_app).activity).assetManager,
        );
        vxlogi!("Configure is: {:?}", *((*android_app).config));
        (*android_app).cmd_poll_source.id = LooperId::Main as i32;
        (*android_app).cmd_poll_source.app = android_app;
        (*android_app).cmd_poll_source.process = process_cmd;
        (*android_app).input_poll_source.id = LooperId::Input as i32;
        (*android_app).input_poll_source.app = android_app;
        (*android_app).input_poll_source.process = process_input;
        (*android_app).looper =
            looper::ALooper_prepare(looper::ALooperPrepare::AllowNonCallbacks as i32);
        looper::ALooper_addFd(
            (*android_app).looper,
            (*android_app).msg_read_fd,
            LooperId::Main as i32,
            looper::ALooperEvent::Input as i32,
            transmute(0usize),
            transmute(&mut (*android_app).cmd_poll_source),
        );
        libc::pthread_mutex_lock(&mut (*android_app).mutex);
        (*android_app).running = 1;
        libc::pthread_cond_broadcast(&mut ((*android_app).cond));
        libc::pthread_mutex_unlock(&mut (*android_app).mutex);
        {
            let os_app = vxunwrap!((*android_app).os_app);
            vxresult!(os_app.read()).initialize();
            let core_app = vxunwrap!(vxresult!(os_app.read()).core_app).clone();
            let renderer = Arc::new(RwLock::new(RenderEngine::new(core_app, os_app)));
            vxresult!(os_app.write()).set_renderer(renderer);
            vxresult!(os_app.read()).run();
        }
        android_app_destroy(android_app);
        ptr::null_mut()
    }
}

pub fn android_app_create(
    activity: *mut activity::ANativeActivity,
    saved_state: *mut libc::c_void,
    saved_state_size: libc::size_t,
    core_app: Arc<RwLock<CoreAppTrait>>,
) {
    unsafe {
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
    }
    let android_app: *mut AndroidApp = unsafe {
        transmute(libc::malloc(size_of::<AndroidApp>()))
    };
    unsafe {
        libc::memset(transmute(android_app), 0, size_of::<AndroidApp>());
        (*android_app).on_input_event = default_on_input_event;
        (*android_app).activity = activity;
    }
    let os_app = Arc::new(RwLock::new(OsApp::new(core_app, android_app)));
    unsafe {
        (*android_app).os_app = Some(os_app);
        libc::pthread_mutex_init(&mut (*android_app).mutex, ptr::null_mut());
        libc::pthread_cond_init(&mut (*android_app).cond, ptr::null_mut());
    }
    if saved_state != ptr::null_mut() {
        unsafe {
            (*android_app).saved_state_size = saved_state_size;
            (*android_app).saved_state = libc::malloc(saved_state_size);
            libc::memcpy((*android_app).saved_state, saved_state, saved_state_size);
        }
    }
    let mut msg_pipe_fds = [0 as libc::c_int, 2];
    if unsafe { libc::pipe(msg_pipe_fds.as_mut_ptr()) } != 0 {
        vxlogf!("Could not create pipe!");
    }
    unsafe {
        (*android_app).msg_read_fd = msg_pipe_fds[0];
        (*android_app).msg_write_fd = msg_pipe_fds[1];
    }
    let mut attr: libc::pthread_attr_t = unsafe { zeroed() };
    unsafe {
        libc::pthread_attr_init(&mut attr);
        libc::pthread_attr_setdetachstate(&mut attr, libc::PTHREAD_CREATE_DETACHED);
        libc::pthread_create(
            &mut (*android_app).thread,
            &attr,
            android_app_entry,
            transmute(android_app),
        );
        libc::pthread_mutex_lock(&mut (*android_app).mutex);
    }
    while unsafe { (*android_app).running != 1 } {
        unsafe {
            libc::pthread_cond_wait(&mut (*android_app).cond, &mut (*android_app).mutex);
        }
    }
    unsafe {
        libc::pthread_mutex_unlock(&mut (*android_app).mutex);
        (*activity).instance = transmute(android_app);
    }
}

unsafe extern "C" fn android_app_write_cmd(android_app: *mut AndroidApp, cmd: i8) {
    if libc::write((*android_app).msg_write_fd, transmute(&cmd), 1) != 1 {
        vxlogf!("Failure writing AndroidApp cmd!");
    }
}

unsafe extern "C" fn android_app_set_input(
    android_app: *mut AndroidApp,
    input_queue: *mut input::AInputQueue,
) {
    libc::pthread_mutex_lock(&mut (*android_app).mutex);
    (*android_app).pending_input_queue = input_queue;
    //    vxloge!("locked!");
    android_app_write_cmd(android_app, AppCmd::InputChanged as i8);
    while (*android_app).input_queue != (*android_app).pending_input_queue {
        libc::pthread_cond_wait(&mut (*android_app).cond, &mut (*android_app).mutex);
    }
    libc::pthread_mutex_unlock(&mut (*android_app).mutex);
    //    vxloge!("unlocked!");
}

unsafe extern "C" fn android_app_set_window(
    android_app: *mut AndroidApp,
    window: *mut window::ANativeWindow,
) {
    libc::pthread_mutex_lock(&mut (*android_app).mutex);
    //    vxloge!("locked");
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
    //    vxloge!("unlocked");
}

unsafe extern "C" fn android_app_set_activity_state(android_app: *mut AndroidApp, cmd: i8) {
    //    vxloge!("locked");
    libc::pthread_mutex_lock(&mut (*android_app).mutex);
    android_app_write_cmd(android_app, cmd);
    while (*android_app).activity_state != cmd as i32 {
        libc::pthread_cond_wait(&mut (*android_app).cond, &mut (*android_app).mutex);
    }
    libc::pthread_mutex_unlock(&mut (*android_app).mutex);
    //    vxloge!("unlocked");
}

unsafe extern "C" fn android_app_free(android_app: *mut AndroidApp) {
    libc::pthread_mutex_lock(&mut (*android_app).mutex);
    //    vxloge!("locked");
    android_app_write_cmd(android_app, AppCmd::Destroy as i8);
    while (*android_app).destroyed == 0 {
        libc::pthread_cond_wait(&mut (*android_app).cond, &mut (*android_app).mutex);
    }
    libc::pthread_mutex_unlock(&mut (*android_app).mutex);
    //    vxloge!("unlocked");
    libc::close((*android_app).msg_read_fd);
    libc::close((*android_app).msg_write_fd);
    libc::pthread_cond_destroy(&mut (*android_app).cond);
    libc::pthread_mutex_destroy(&mut (*android_app).mutex);
    (*android_app).os_app = None;
    libc::free(transmute(android_app));
}

pub unsafe extern "C" fn on_destroy(activity: *mut activity::ANativeActivity) {
    vxlogi!("Destroy: {:?}", activity);
    android_app_free(transmute((*activity).instance));
}

pub unsafe extern "C" fn on_start(activity: *mut activity::ANativeActivity) {
    vxlogi!("Start: {:?}", activity);
    android_app_set_activity_state(transmute((*activity).instance), AppCmd::Start as i8);
}

pub unsafe extern "C" fn on_resume(activity: *mut activity::ANativeActivity) {
    vxlogi!("Resume: {:?}", activity);
    android_app_set_activity_state(transmute((*activity).instance), AppCmd::Resume as i8);
}

pub unsafe extern "C" fn on_save_instance_state(
    activity: *mut activity::ANativeActivity,
    out_len: *mut libc::size_t,
) -> *mut libc::c_void {
    let android_app: *mut AndroidApp = transmute((*activity).instance);
    let mut saved_state: *mut libc::c_void = ptr::null_mut();
    vxlogi!("SaveInstanceState: {:?}", activity);
    libc::pthread_mutex_lock(&mut (*android_app).mutex);
    //    vxloge!("locked");
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
    //    vxloge!("unlocked");
    return saved_state;
}

pub unsafe extern "C" fn on_pause(activity: *mut activity::ANativeActivity) {
    vxlogi!("Pause: {:?}", activity);
    android_app_set_activity_state(transmute((*activity).instance), AppCmd::Pause as i8);
}

pub unsafe extern "C" fn on_stop(activity: *mut activity::ANativeActivity) {
    vxlogi!("Stop: {:?}", activity);
    android_app_set_activity_state(transmute((*activity).instance), AppCmd::Stop as i8);
}

pub unsafe extern "C" fn on_configuration_changed(activity: *mut activity::ANativeActivity) {
    let android_app: *mut AndroidApp = transmute((*activity).instance);
    vxlogi!("ConfigurationChanged: {:?}", activity);
    android_app_write_cmd(android_app, AppCmd::ConfigChanged as i8);
}

pub unsafe extern "C" fn on_low_memory(activity: *mut activity::ANativeActivity) {
    let android_app: *mut AndroidApp = transmute((*activity).instance);
    vxlogi!("LowMemory: {:?}", activity);
    android_app_write_cmd(android_app, AppCmd::LowMemory as i8);
}

pub unsafe extern "C" fn on_window_focus_changed(
    activity: *mut activity::ANativeActivity,
    focused: libc::c_int,
) {
    vxlogi!("WindowFocusChanged: {:?} -- {:?}", activity, focused);
    android_app_write_cmd(
        transmute((*activity).instance),
        if focused != 0 {
            AppCmd::GainedFocus
        } else {
            AppCmd::LostFocus
        } as i8,
    );
}

pub unsafe extern "C" fn on_native_window_created(
    activity: *mut activity::ANativeActivity,
    window: *mut window::ANativeWindow,
) {
    vxlogi!("NativeWindowCreated: {:?} -- {:?}", activity, window);
    android_app_set_window(transmute((*activity).instance), window);
}

pub unsafe extern "C" fn on_native_window_destroyed(
    activity: *mut activity::ANativeActivity,
    window: *mut window::ANativeWindow,
) {
    vxlogi!("NativeWindowDestroyed: {:?} -- {:?}", activity, window);
    android_app_set_window(transmute((*activity).instance), ptr::null_mut());
}

pub unsafe extern "C" fn on_input_queue_created(
    activity: *mut activity::ANativeActivity,
    queue: *mut input::AInputQueue,
) {
    vxlogi!("InputQueueCreated: {:?} -- {:?}", activity, queue);
    android_app_set_input(transmute((*activity).instance), queue);
}

pub unsafe extern "C" fn on_input_queue_destroyed(
    activity: *mut activity::ANativeActivity,
    queue: *mut input::AInputQueue,
) {
    vxlogi!("InputQueueDestroyed: {:?} -- {:?}", activity, queue);
    android_app_set_input(transmute((*activity).instance), ptr::null_mut());
}

unsafe extern "C" fn default_on_input_event(
    _app: *mut AndroidApp,
    _event: *mut input::AInputEvent,
) -> i32 {
    0
}
