use super::super::super::core::application::ApplicationTrait as CoreAppTrait;
use super::super::super::render::engine::Engine as RenderEngine;
use super::super::apple;
use super::app_delegate;
use super::game_view;
use super::game_view_controller;
use std::mem::transmute;
use std::os::raw::c_void;
use std::ptr::null_mut;
use std::sync::{Arc, RwLock};

pub struct Application {
    pub app: apple::Id,
    pub app_dlg: apple::Id,
    pub auto_release_pool: Option<apple::NsAutoReleasePool>,
    pub core_app: Arc<RwLock<CoreAppTrait>>,
    pub render_engine: Option<Arc<RwLock<RenderEngine>>>,
}

impl Application {
    pub fn new(core_app: Arc<RwLock<CoreAppTrait>>) -> Self {
        let auto_release_pool = Some(apple::NsAutoReleasePool::new());
        app_delegate::register();
        game_view::register();
        game_view_controller::register();
        let app = apple::get_class("NSApplication");
        let app: apple::Id = unsafe { msg_send![app, sharedApplication] };
        let app_dlg = app_delegate::create_instance();
        // let render_engine = Arc::new(RwLock::new(RenderEngine::new(&core_app)));
        let render_engine = None;
        Application {
            app,
            app_dlg,
            auto_release_pool,
            core_app,
            render_engine,
        }
    }

    pub fn initialize(&self, itself: Arc<RwLock<Application>>) {
        unsafe {
            let itself_ptr: *mut c_void = transmute(Box::into_raw(Box::new(itself.clone())));
            (*self.app_dlg)
                .set_ivar(app_delegate::APP_VAR_NAME, itself_ptr);
            let _: () = msg_send![self.app_dlg, initialize];
            let _: () = msg_send![self.app, setDelegate:self.app_dlg];
        };
        // vxresult!(self.render_engine.write()).initilize(&itself);
        // vxresult!(self.core_app.write()).initilize(&itself, &self.render_engine);
        unsafe {
            let gvc: apple::Id = *(*self.app_dlg).get_ivar(app_delegate::CONTROLLER_VAR_NAME);
            let _: () = msg_send![gvc, startLinkDisplay];
            let _: () = msg_send![self.app, run];
        }
    }

    pub fn update(&self) {
        vxlogi!("reached");
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        // maybe some day I need here to do some deinitialization
        // but if it was not needed that day remove the Option from autorelease
        self.auto_release_pool = None;
    }
}
