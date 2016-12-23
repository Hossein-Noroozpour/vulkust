use libc::{
    c_int,
    dlopen,
    c_void,
    RTLD_LAZY,
};

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

type WriteFunPtr = unsafe extern "C" fn(priority: c_int, tag: *const u8, text: *const u8) -> c_int;

pub struct Log {
    write: WriteFunPtr,
    dylib_handle: *const c_void,
}

impl Log {
    pub fn new() {
        let dylib_handle = unsafe {dlopen("/system/lib/liblog.so".as_ptr(), RTLD_LAZY)};
        if dylib_handle == 0 as *mut c_void {
            panic!("Android log shared library not found!");
        }
    }
}