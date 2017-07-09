use std::ptr::null_mut;
use std::mem::transmute;
use super::super::super::core::application::ApplicationTrait;
use super::super::super::system::metal as mtl;
use super::super::super::system::metal::dispatch;
use super::super::super::system::os::OsApplication;

#[derive(Debug)]
pub struct Stage {
    pub function: mtl::Id,
}

impl Stage {
    pub fn new<CoreApp>(
        data: Vec<u8>, os_app: *mut OsApplication<CoreApp>, stage_name: &str) -> Self
    where
        CoreApp: ApplicationTrait,
    {
        let device = unsafe { (*os_app).metal_device };
        let mut error = mtl::NSError::null();
        let library: mtl::Id = unsafe {
            let queue = dispatch::dispatch_get_main_queue();
            let data_ptr: dispatch::dispatch_data_t = dispatch::dispatch_data_create(
                transmute(data.as_ptr()),
                data.len(),
                queue,
                dispatch::DISPATCH_DATA_DESTRUCTOR_DEFAULT,
            );
            msg_send![device, newLibraryWithData:data_ptr error:error.as_ptr()]
        };
        if library == null_mut() || error.is_error() {
            logf!("Creating metal library failed with error {}", error);
        }
        let s = mtl::NSString::new(&("main_".to_string() + stage_name + "_func"));
        Stage { function: unsafe { msg_send![library, newFunctionWithName:s.s] } }
    }
}
