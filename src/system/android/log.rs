use libc::{
    c_int,
    c_void,
    c_char,
    dlopen,
    dlclose,
    dlsym,
    dlerror,
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
    write_fun_ptr: WriteFunPtr,
    dylib_handle: *mut c_void,
}

impl Log {
    pub fn new() -> Self {
        let dylib_handle = unsafe {
            dlopen("/system/lib/liblog.so".as_ptr() as *const c_char, RTLD_LAZY)
        };
        if dylib_handle == 0 as *mut c_void {
            panic!("Android log shared library not found!");
        }
        let write = unsafe {
            dlerror();
            dlsym(dylib_handle, "__android_log_write".as_ptr() as *const c_char) as WriteFunPtr
        };
        if 0 as *mut c_char != unsafe { dlerror() } {
            panic!("Android log write function not found!");
        }
        Log {
            write_fun_ptr: write,
            dylib_handle: dylib_handle,
        }
    }

    pub fn write(&self, s: &str) {
        unsafe {
            (self.write_fun_ptr)(
                Priority::Debug as c_int, "vulkust-rust".as_ptr() as *const c_char,
                s.as_ptr() as *const c_char);
        }
    }
}

impl Drop for Log {
    fn drop(&mut self) {
        unsafe {dlclose(self.dylib_handle);}
    }
}