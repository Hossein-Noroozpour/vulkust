use super::super::super::core::application::ApplicationTrait as CoreAppTrait;
use super::super::super::render::engine::Engine as RenderEngine;
use std::mem::transmute;
use std::os::raw::c_void;
use std::ptr::null_mut;
use std::sync::{Arc, RwLock};

pub struct Application {
    pub core_app: Arc<RwLock<CoreAppTrait>>,
    pub itself: Option<Arc<RwLock<Application>>>,
    pub view: *mut c_void,
    // pub render_engine: Option<Arc<RwLock<RenderEngine>>>,
}

impl Application {
    pub fn new(core_app: Arc<RwLock<CoreAppTrait>>) -> Self {
        Application {
            core_app,
            itself: None,
            view: null_mut(),
        }
    }

    pub fn set_itself(&mut self, itself: Arc<RwLock<Application>>) {
        self.itself = Some(itself);
    }

    pub fn update(&self) {}
}

impl Drop for Application {
    fn drop(&mut self) {}
}

#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn vulkust_deallocate(context: *mut c_void) {
    let os_app: *mut Arc<RwLock<Application>> = unsafe { transmute(context) };
    unsafe {
        let _ = Box::from_raw(os_app);
    }
    vxlogi!("Reached");
}

#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn vulkust_set_view(context: *mut c_void, view: *mut c_void) {
    let os_app: &'static Arc<RwLock<Application>> = unsafe { transmute(context) };
    let mut os_app = vxresult!(os_app.write());
    os_app.view = view;
    vxlogi!("Reached");
}

#[allow(dead_code)]
#[no_mangle]
pub extern "C" fn vulkust_render(context: *mut c_void) {
    let os_app: &'static Arc<RwLock<Application>> = unsafe { transmute(context) };
    let os_app = vxresult!(os_app.read());
    os_app.update();
    vxlogi!("Reached");
}
