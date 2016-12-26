#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]

use std::os::raw::{
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

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ANativeActivityCallbacks {
    onStart: unsafe extern fn(activity: *mut ANativeActivity),
    onResume: unsafe extern fn(activity: *mut ANativeActivity),
    onSaveInstanceState: unsafe extern fn(activity: *mut ANativeActivity, outSize: *mut usize),
    onPause: unsafe extern fn(activity: *mut ANativeActivity),
    onStop: unsafe extern fn(activity: *mut ANativeActivity),
    onDestroy: unsafe extern fn(activity: *mut ANativeActivity),
    onWindowFocusChanged: unsafe extern fn(activity: *mut ANativeActivity, hasFocus: c_int),
    onNativeWindowCreated: unsafe extern fn(activity: *mut ANativeActivity, window: *mut ANativeWindow),
    onNativeWindowResized: unsafe extern fn(activity: *mut ANativeActivity, window: *mut ANativeWindow),
    onNativeWindowRedrawNeeded: unsafe extern fn(activity: *mut ANativeActivity, window: *mut ANativeWindow),
    onNativeWindowDestroyed: unsafe extern fn(activity: *mut ANativeActivity, window: *mut ANativeWindow),
    onInputQueueCreated: unsafe extern fn(activity: *mut ANativeActivity, queue: *mut AInputQueue),
    onInputQueueDestroyed: unsafe extern fn(activity: *mut ANativeActivity, queue: *mut AInputQueue),
    onContentRectChanged: unsafe extern fn(activity: *mut ANativeActivity, rect: *const ARect),
    onConfigurationChanged: unsafe extern fn(activity: *mut ANativeActivity),
    onLowMemory: unsafe extern fn(activity: *mut ANativeActivity),
}

#[no_mangle]
pub unsafe extern fn ANativeActivity_onCreate(activity: *mut ANativeActivity, savedState: *mut c_void, savedStateSize: usize) {
    logdbg!("Native activity created.");
}

#[cfg_attr(target_os = "android", link(name = "android", kind= "dylib"))]
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