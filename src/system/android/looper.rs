use std::os::raw::{
    c_int,
    c_void,
};

pub type ALooper = c_void;


#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum ALooperPrepare {
    AllowNonCallbacks = 1,
}


#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum ALooperPoll {
    Wake = -1,
    Callback = -2,
    Timeout = -3,
    Error = -4,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum ALooperEvent {
    Input = 1,
    Output = 2,
    Error = 4,
    Hangup = 8,
    Invalid = 16,
}

pub type ALooperCallbackFunc = unsafe extern fn(fd: c_int, events: c_int, data: *mut c_void) -> c_int;

#[cfg_attr(target_os = "android", link(name = "android", kind = "dylib"))]
extern {
    pub fn ALooper_forThread() -> *mut ALooper;
    pub fn ALooper_prepare(opts: c_int) -> *mut ALooper;
    pub fn ALooper_acquire(looper: *mut ALooper);
    pub fn ALooper_release(looper: *mut ALooper);
    pub fn ALooper_pollOnce(timeout_millis: c_int, out_fd: *mut c_int, out_events: *mut c_int, out_data: *mut *mut c_void) -> c_int;
    pub fn ALooper_pollAll(timeout_millis: c_int, out_fd: *mut c_int, out_events: *mut c_int, out_data: *mut *mut c_void) -> c_int;
    pub fn ALooper_wake(looper: *mut ALooper);
    pub fn ALooper_addFd(looper: *mut ALooper, fd: c_int, ident: c_int, events: c_int, callback: ALooperCallbackFunc, data: *mut c_void) -> c_int;
    pub fn ALooper_removeFd(looper: *mut ALooper, fd: c_int) -> c_int;
}