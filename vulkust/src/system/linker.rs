#[cfg(target_os = "linux")]
extern crate libc;
use std::ffi::CString;
use std::ptr::null_mut;

#[cfg(target_os = "linux")]
pub struct Linker {
    link: *mut libc::c_void,
}

macro_rules! make_link {
    (
        $struct_name:ident [
            $($function_name:ident $function_type:ty)*
        ]
    ) => (
        pub struct $struct_name {
            concat_idents!($function_name, _ptr): Option<$function_type>,
        }
    )
}

impl Linker {
    pub fn new(library_name: &str) -> Self {
        let cs = CString::new(library_name).unwrap();
        Linker {
            link: unsafe { libc::dlopen(cs.as_ptr(), libc::RTLD_NOW | libc::RTLD_LOCAL) },
        }
    }

    pub fn is_ok(&self) -> bool {
        self.link == null_mut()
    }
}
