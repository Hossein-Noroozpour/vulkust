#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]

use std::os::raw::{
    c_void,
    c_char,
    c_int,
};
use std::sync::{
    Arc,
    RwLock,
};
use std::mem::transmute;
use super::jni::{
    JavaVM,
    JNIEnv,
    jobject,
};
use super::super::super::core::application::BasicApplication;
use super::application::Application as AndroidApp;
use super::asset::{
    AAssetManager,
};
use super::rect::{
    ARect,
};
use super::input::{
    AInputQueue,
};
use super::window::{
    ANativeWindow,
};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ANativeActivity {
    pub callbacks: *mut ANativeActivityCallbacks,
    pub vm: *mut JavaVM,
    pub env: *mut JNIEnv,
    pub class: jobject,
    pub internalDataPath: *const c_char,
    pub externalDataPath: *const c_char,
    pub sdkVersion: i32,
    pub instance: *mut c_void,
    pub assetManager: *mut AAssetManager,
    pub obbPath: *const c_char,
}

type activity_receiver = unsafe extern fn(activity: *mut ANativeActivity);
type activity_size_receiver = unsafe extern fn(activity: *mut ANativeActivity, size: *mut usize);
type activity_int_receiver = unsafe extern fn(activity: *mut ANativeActivity, hasFocus: c_int);
type activity_window_receiver = unsafe extern fn(activity: *mut ANativeActivity, window: *mut ANativeWindow);
type activity_input_receiver = unsafe extern fn(activity: *mut ANativeActivity, queue: *mut AInputQueue);
type activity_rect_receiver = unsafe extern fn(activity: *mut ANativeActivity, rect: *const ARect);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ANativeActivityCallbacks {
    pub onStart: activity_receiver,
    pub onResume: activity_receiver,
    pub onSaveInstanceState: activity_size_receiver,
    pub onPause: activity_receiver,
    pub onStop: activity_receiver,
    pub onDestroy: activity_receiver,
    pub onWindowFocusChanged: activity_int_receiver,
    pub onNativeWindowCreated: activity_window_receiver,
    pub onNativeWindowResized: activity_window_receiver,
    pub onNativeWindowRedrawNeeded: activity_window_receiver,
    pub onNativeWindowDestroyed: activity_window_receiver,
    pub onInputQueueCreated: activity_input_receiver,
    pub onInputQueueDestroyed: activity_input_receiver,
    pub onContentRectChanged: activity_rect_receiver,
    pub onConfigurationChanged: activity_receiver,
    pub onLowMemory: activity_receiver,
}

#[no_mangle]
pub unsafe extern fn ANativeActivity_onCreate(activity: *mut ANativeActivity, savedState: *mut c_void, savedStateSize: usize) {
    logdbg!("Native activity created.");
    (*activity).instance = android_app_create(activity, savedState, savedStateSize);
}

#[cfg_attr(target_os = "android", link(name = "android", kind = "dylib"))]
extern {
    pub fn ANativeActivity_finish(activity: *mut ANativeActivity);
    pub fn ANativeActivity_setWindowFormat(activity: *mut ANativeActivity, format: i32);
    pub fn ANativeActivity_setWindowFlags(activity: *mut ANativeActivity, addFlags: u32, removeFlags: u32);
    pub fn ANativeActivity_showSoftInput(activity: *mut ANativeActivity, flags: u32);
    pub fn ANativeActivity_hideSoftInput(activity: *mut ANativeActivity, flags: u32);
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum ShowSoftInputFlagBits {
    IMPLICIT = 0x0001,
    FORCED = 0x0002,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum HideSoftInputFlagBits {
    IMPLICIT_ONLY = 0x0001,
    NOT_ALWAYS = 0x0002,
}

unsafe extern fn on_start(activity: *mut ANativeActivity) {
    let app: *mut AndroidApp = transmute((*activity).instance);
    (*app).on_start(activity);
}

unsafe extern fn on_resume(activity: *mut ANativeActivity) {
    let app: *mut AndroidApp = transmute((*activity).instance);
    (*app).on_resume(activity);
}

unsafe extern fn on_save_instance_state(activity: *mut ANativeActivity, size: *mut usize) {
    let app: *mut AndroidApp = transmute((*activity).instance);
    (*app).on_save_instance_state(activity, size);
}

unsafe extern fn on_pause(activity: *mut ANativeActivity) {
    let app: *mut AndroidApp = transmute((*activity).instance);
    (*app).on_pause(activity);
}

unsafe extern fn on_stop(activity: *mut ANativeActivity) {
    let app: *mut AndroidApp = transmute((*activity).instance);
    (*app).on_stop(activity);
}

unsafe extern fn on_destroy(activity: *mut ANativeActivity) {
    let app: *mut AndroidApp = transmute((*activity).instance);
    (*app).on_destroy(activity);
}

unsafe extern fn on_window_focus_changed(activity: *mut ANativeActivity, has_focus: c_int) {
    let app: *mut AndroidApp = transmute((*activity).instance);
    (*app).on_window_focus_changed(activity, has_focus as i64);
}

unsafe extern fn on_native_window_created(activity: *mut ANativeActivity, window: *mut ANativeWindow) {
    let app: *mut AndroidApp = transmute((*activity).instance);
    (*app).on_native_window_created(activity, window);
}

unsafe extern fn on_native_window_resized(activity: *mut ANativeActivity, window: *mut ANativeWindow) {
    let app: *mut AndroidApp = transmute((*activity).instance);
    (*app).on_native_window_resized(activity, window);
}

unsafe extern fn on_native_window_redraw_needed(activity: *mut ANativeActivity, window: *mut ANativeWindow) {
    let app: *mut AndroidApp = transmute((*activity).instance);
    (*app).on_native_window_redraw_needed(activity, window);
}

unsafe extern fn on_native_window_destroyed(activity: *mut ANativeActivity, window: *mut ANativeWindow) {
    let app: *mut AndroidApp = transmute((*activity).instance);
    (*app).on_native_window_created(activity, window);
}

unsafe extern fn on_input_queue_created(activity: *mut ANativeActivity, queue: *mut AInputQueue) {
    let app: *mut AndroidApp = transmute((*activity).instance);
    (*app).on_input_queue_created(activity, queue);
}

unsafe extern fn on_input_queue_destroyed(activity: *mut ANativeActivity, queue: *mut AInputQueue) {
    let app: *mut AndroidApp = transmute((*activity).instance);
    (*app).on_input_queue_destroyed(activity, queue);
}

unsafe extern fn on_content_rect_changed(activity: *mut ANativeActivity, rect: *const ARect) {
    let app: *mut AndroidApp = transmute((*activity).instance);
    (*app).on_content_rect_changed(activity, rect);
}

unsafe extern fn on_configuration_changed(activity: *mut ANativeActivity) {
    let app: *mut AndroidApp = transmute((*activity).instance);
    (*app).on_configuration_changed(activity);
}

unsafe extern fn on_low_memory(activity: *mut ANativeActivity) {
    let app: *mut AndroidApp = transmute((*activity).instance);
    (*app).on_low_memory(activity);
}

fn android_app_create(activity: *mut ANativeActivity, savedState: *mut c_void, savedStateSize: usize) -> *mut c_void {
    unsafe {
        (*(*activity).callbacks).onStart = on_start;
        (*(*activity).callbacks).onResume = on_resume;
        (*(*activity).callbacks).onSaveInstanceState = on_save_instance_state;
        (*(*activity).callbacks).onPause = on_pause;
        (*(*activity).callbacks).onStop = on_stop;
        (*(*activity).callbacks).onDestroy = on_destroy;
        (*(*activity).callbacks).onWindowFocusChanged = on_window_focus_changed;
        (*(*activity).callbacks).onNativeWindowCreated = on_native_window_created;
        (*(*activity).callbacks).onNativeWindowResized = on_native_window_resized;
        (*(*activity).callbacks).onNativeWindowRedrawNeeded = on_native_window_redraw_needed;
        (*(*activity).callbacks).onNativeWindowDestroyed = on_native_window_destroyed;
        (*(*activity).callbacks).onInputQueueCreated = on_input_queue_created;
        (*(*activity).callbacks).onInputQueueDestroyed = on_input_queue_destroyed;
        (*(*activity).callbacks).onContentRectChanged = on_content_rect_changed;
        (*(*activity).callbacks).onConfigurationChanged = on_configuration_changed;
        (*(*activity).callbacks).onLowMemory = on_low_memory;
    }
    let mut app = AndroidApp::new(activity);
    unsafe {
        transmute(&mut app)
    }
}
