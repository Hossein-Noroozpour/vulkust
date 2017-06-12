use std::ptr::null_mut;
use std::mem::transmute;
use super::super::super::system::metal as mtl;
use super::super::super::system::dispatch;

#[derive(Debug)]
pub struct Stage {
    pub function: mtl::Id,
}

impl Stage {
    pub fn new(data: Vec<u8>, device: mtl::Id) -> Self {
        let mut null_error: mtl::Id = null_mut();
        let null_error = mtl::IdPtr { id: &mut null_error };
        let library: mtl::Id = unsafe {
            let queue = dispatch::dispatch_get_main_queue();
            logi!("reached");
            let data_ptr: dispatch::dispatch_data_t = dispatch::dispatch_data_create(
                transmute(data.as_ptr()), data.len(), queue,
                dispatch::DISPATCH_DATA_DESTRUCTOR_DEFAULT);

            logi!("reached");
            msg_send![device, newLibraryWithData:data_ptr error:null_error]
        };
        logi!("reached");
        let s = mtl::NSString::new("main_func");
        logi!("reached");
        Stage {
            function: unsafe { msg_send![library, newFunctionWithName:s.s] },
        }
    }
}
