use super::super::super::core::application::ApplicationTrait as CoreAppTrait;
use super::super::super::render::engine::Engine as RenderEngine;
use std::mem::transmute;
use std::os::raw::c_void;
use std::ptr::null_mut;
use std::sync::{Arc, RwLock, Weak};

pub struct Application {
    pub core_app: Arc<RwLock<CoreAppTrait>>,
    pub itself: Option<Weak<RwLock<Application>>>,
    pub view: *mut c_void,
    pub renderer: Option<Arc<RwLock<RenderEngine>>>,
}

impl Application {
    pub fn new(core_app: Arc<RwLock<CoreAppTrait>>) -> Self {
        Application {
            core_app,
            itself: None,
            view: null_mut(),
            renderer: None,
        }
    }

    pub fn set_itself(&mut self, itself: Weak<RwLock<Application>>) {
        self.itself = Some(itself);
    }

    pub fn update(&self) {
        vxresult!(vxunwrap!(self.renderer).write()).update();
    }
}

impl Drop for Application {
    fn drop(&mut self) {}
}

#[no_mangle]
pub extern "C" fn vulkust_deallocate(context: *mut c_void) {
    let os_app: *mut Arc<RwLock<Application>> = unsafe { transmute(context) };
    unsafe {
        let _ = Box::from_raw(os_app);
    }
    vxlogi!("Reached");
}

#[no_mangle]
pub extern "C" fn vulkust_set_view(context: *mut c_void, view: *mut c_void) {
    let os_app: &'static Arc<RwLock<Application>> = unsafe { transmute(context) };
    vxresult!(os_app.write()).view = view;
    let core_app = vxresult!(os_app.read()).core_app.clone();
    let renderer = Some(Arc::new(RwLock::new(RenderEngine::new(core_app, os_app))));
    let renderer_w = Arc::downgrade(&renderer);
    vxresult!(renderer.write()).set_myself(renderer_w);
    vxresult!(os_app.write()).renderer = renderer;
    vxlogi!("Reached");
}

#[no_mangle]
pub extern "C" fn vulkust_render(context: *mut c_void) {
    let os_app: &'static Arc<RwLock<Application>> = unsafe { transmute(context) };
    let os_app = vxresult!(os_app.read());
    os_app.update();
}
