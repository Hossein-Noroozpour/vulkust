#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]

use libc::{
    c_void,
    c_char,
    c_int,
};
use super::jni::{
    JavaVM,
    JNIEnv,
    jobject,
};
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
type activity_size_receiver = unsafe extern fn(activity: *mut ANativeActivity, size: *mut usize) -> *mut c_void;
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