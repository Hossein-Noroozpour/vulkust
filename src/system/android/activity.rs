#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]

use std::os::raw::{
    c_void,
    c_char,
    c_int,
};

pub struct ANativeActivity {
    callbacks: *mut ANativeActivityCallbacks,
    vm: *mut JavaVM,
    env: *mut JNIEnv,
    class: jobject,
    internalDataPath: *const c_char,
    externalDataPath: *const c_char,
    sdkVersion: i32,
    instance: *mut c_void,
    assetManager: *mut AAssetManager,
    obbPath: *const c_char,
}

pub struct ANativeActivityCallbacks {
    onStart: *mut unsafe extern fn(activity: *mut ANativeActivity),
    onResume: *mut unsafe extern fn(activity: *mut ANativeActivity),
    onSaveInstanceState: *mut unsafe extern fn(activity: *mut ANativeActivity, outSize: *mut usize),
    onPause: *mut unsafe extern fn(activity: *mut ANativeActivity),
    onStop: *mut unsafe extern fn(activity: *mut ANativeActivity),
    onDestroy: *mut unsafe extern fn(activity: *mut ANativeActivity),
    onWindowFocusChanged: *mut unsafe extern fn(activity: *mut ANativeActivity, hasFocus: c_int),
    onNativeWindowCreated: *mut unsafe extern fn(activity: *mut ANativeActivity, window: *mut ANativeWindow),
    onNativeWindowResized: *mut unsafe extern fn(activity: *mut ANativeActivity, window: *mut ANativeWindow),
    onNativeWindowRedrawNeeded: *mut unsafe extern fn(activity: *mut ANativeActivity, window: *mut ANativeWindow),
    onNativeWindowDestroyed: *mut unsafe extern fn(activity: *mut ANativeActivity, window: *mut ANativeWindow),
    onInputQueueCreated: *mut unsafe extern fn(activity: *mut ANativeActivity, queue: *mut AInputQueue),
    onInputQueueDestroyed: *mut unsafe extern fn(activity: *mut ANativeActivity, queue: *mut AInputQueue),
    onContentRectChanged: *mut unsafe extern fn(activity: *mut ANativeActivity, rect: *const ARect),
    onConfigurationChanged: *mut unsafe extern fn(activity: *mut ANativeActivity),
    onLowMemory: *mut unsafe extern fn(activity: *mut ANativeActivity),
}

#[no_mangle]
pub unsafe extern fn ANativeActivity_onCreate(activity: *mut ANativeActivity, savedState: *mut c_void, savedStateSize: usize) {}

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