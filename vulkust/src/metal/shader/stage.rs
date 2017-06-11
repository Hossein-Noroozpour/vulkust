use std::ptr::null_mut;
use super::super::super::system::metal as mtl;

#[derive(Debug)]
pub struct Stage {
    pub function: mtl::Id,
}

impl Stage {
    pub fn new(data: Vec<u8>, device: mtl::Id) -> Self {
        let null_error: mtl::Id = null_mut();
        let library: mtl::Id = unsafe {
            msg_send![device, newLibraryWithdata:data.as_ptr() error:null_error]
        };
        let s = mtl::NSString::new("main");
        Stage {
            function: unsafe { msg_send![library, newFunctionWithName:s.s] },
        }
    }
}
