#[cfg(not(feature = "no-log"))]
use std::os::raw::{c_char, c_int};

#[cfg(not(feature = "no-log"))]
#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum Priority {
    Unknown = 0,
    Default = 1,
    Verbose = 2,
    Debug = 3,
    Info = 4,
    Warn = 5,
    Error = 6,
    Fatal = 7,
    Silent = 8,
}

#[cfg(not(feature = "no-log"))]
#[cfg_attr(target_os = "android", link(name = "log", kind = "dylib"))]
extern "C" {
    pub fn __android_log_write(priority: c_int, tag: *const c_char, text: *const c_char) -> c_int;
}

#[cfg(not(feature = "no-log"))]
pub fn print(priority: Priority, text: &String) {
    use std::ffi::CString;
    let tag = CString::new("vulkust").unwrap();
    let text = CString::new(text.as_str()).unwrap();
    unsafe {
        __android_log_write(priority as c_int, tag.as_ptr(), text.as_ptr());
    }
}
