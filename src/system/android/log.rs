#[cfg(debug_assertions)]
use std::os::raw::{
    c_int,
    c_char,
};

#[cfg(debug_assertions)]
#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum Priority {
    Unknown = 0,
    Default = 1,
    Verbose = 2,
    Debug   = 3,
    Info    = 4,
    Warn    = 5,
    Error   = 6,
    Fatal   = 7,
    Silent  = 8,
}

#[cfg(debug_assertions)]
#[cfg_attr(target_os = "android", link(name = "log", kind= "dylib"))]
extern "C" {
    pub fn __android_log_write(priority: c_int, tag: *const c_char, text: *const c_char) -> c_int;
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! logerr {
    ( $x:expr ) => {
        unsafe {
            ::system::android::log::__android_log_write(
                log::Priority::Error as ::std::os::raw::c_int,
                "vulkust-rust\0".as_ptr(),
                format!("Msg: {:?} in file: {} in line: {}\0", $x, file!(), line!()).as_ptr());
        }
    }
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! logerr {
    ( $x:expr ) => {
        $x
    }
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! logdbg {
    ( $x:expr ) => {
        unsafe {
            ::system::android::log::__android_log_write(
                log::Priority::Debug as ::std::os::raw::c_int,
                "vulkust-rust\0".as_ptr(),
                format!("Msg: {:?} in file: {} in line: {}\0", $x, file!(), line!()).as_ptr());
        }
    }
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! logdbg {
    ( $x:expr ) => {
        $x
    }
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! logftl {
    ( $x:expr ) => {
        unsafe {
            ::system::android::log::__android_log_write(
                log::Priority::Fatal as ::std::os::raw::c_int,
                "vulkust-rust\0".as_ptr(),
                format!("Msg: {:?} in file: {} in line: {}\0", $x, file!(), line!()).as_ptr());
        }
        panic!("Exit");
    }
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! logftl {
    ( $x:expr ) => {
        panic!("Exit {:?}", $x);
    }
}