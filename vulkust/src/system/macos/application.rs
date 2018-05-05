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
    pub view: apple::Id,
    pub auto_release_pool: apple::NsAutoReleasePool,
}

impl Application {
    pub fn new() -> Self {
        let auto_release_pool = apple::NsAutoReleasePool::new();
        app_delegate::register();
        game_view::register();
        game_view_controller::register();
        let app = apple::get_class("NSApplication");
        let app: apple::Id = unsafe { msg_send![app, sharedApplication] };
        let app_dlg = app_delegate::create_instance();
        let view: apple::Id = unsafe {
            let _: () = msg_send![app_dlg, initialize];
            let _: () = msg_send![app, setDelegate:app_dlg];
            *(*app_dlg).get_ivar(app_delegate::VIEW_VAR_NAME)
        };
        Application {
            app,
            app_dlg,
            view,
            auto_release_pool,
        }
    }

    pub fn initialize(&self, itself: Arc<RwLock<Application>>) -> bool {
        unsafe {
            let _: () = msg_send![self.app, run];
        }
        true
    }
}
