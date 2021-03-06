#[cfg(any(target_os = "linux", target_os = "ios", target_os = "macos", target_os = "android"))]
extern crate libc;
use std::ffi::CString;
use std::mem::transmute_copy;
use std::ptr::null_mut;

#[cfg(any(target_os = "linux", target_os = "ios", target_os = "macos", target_os = "android"))]
pub struct Linker {
    link: *mut libc::c_void,
}

impl Linker {
    #[cfg(any(target_os = "linux", target_os = "ios", target_os = "macos", target_os = "android"))]
    pub fn new(library_name: &str) -> Self {
        let cs = CString::new(library_name).unwrap();
        Linker {
            link: unsafe { libc::dlopen(cs.as_ptr(), libc::RTLD_NOW | libc::RTLD_LOCAL) },
        }
    }

    #[cfg(any(target_os = "linux", target_os = "ios", target_os = "macos", target_os = "android"))]
    pub fn is_ok(&self) -> bool {
        self.link != null_mut()
    }

    #[cfg(any(target_os = "linux", target_os = "ios", target_os = "macos", target_os = "android"))]
    fn get_fun_ptr(&self, name: &str) -> *mut libc::c_void {
        let cs = CString::new(name).unwrap();
        unsafe { libc::dlsym(self.link, cs.as_ptr()) }
    }

    pub fn get_function<F>(&self, name: &str) -> Option<F>
    where
        F: Sized,
    {
        let f = self.get_fun_ptr(name);
        if f == null_mut() {
            return None;
        } else {
            unsafe {
                return Some(transmute_copy(&f));
            }
        }
    }
}
