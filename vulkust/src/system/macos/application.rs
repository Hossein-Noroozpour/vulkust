use super::super::super::core::application::ApplicationTrait as CoreAppTrait;
use super::super::super::objc::runtime::YES;
use super::super::super::render::engine::Engine as RenderEngine;
use super::super::apple;
use super::app_delegate;
use super::game_view;
use super::game_view_controller;
use std::mem::transmute;
use std::os::raw::c_void;
use std::sync::{Arc, RwLock};

pub struct Application {
    pub app: apple::Id,
    pub app_dlg: apple::Id,
    pub controller: apple::Id,
    pub auto_release_pool: Option<apple::NsAutoReleasePool>,
    pub core_app: Option<Arc<RwLock<CoreAppTrait>>>,
    pub renderer: Option<Arc<RwLock<RenderEngine>>>,
    pub view: *mut c_void,
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
        let view = unsafe {
            let _: () = msg_send![app_dlg, initialize];
            let _: () = msg_send![app, setDelegate: app_dlg];
            let view: apple::Id = *(*app_dlg).get_ivar(app_delegate::VIEW_VAR_NAME);
            transmute(view)
        };
        let controller = unsafe { *(*app_dlg).get_ivar(app_delegate::VIEW_VAR_NAME) };
        let renderer = None;
        let core_app = Some(core_app);
        Application {
            app,
            app_dlg,
            auto_release_pool,
            core_app,
            renderer,
            view,
            controller,
        }
    }

    pub fn initialize(_itself: &Arc<RwLock<Application>>) {}

    pub fn set_renderer(&mut self, renderer: Arc<RwLock<RenderEngine>>) {
        self.renderer = Some(renderer);
    }

    pub fn run(&self) {
        unsafe {
            {
                let os_app = vxresult!(vxunwrap!(self.renderer).read()).os_app.upgrade();
                let os_app = Box::into_raw(Box::new(vxunwrap!(os_app).clone()));
                let os_app: *mut c_void = transmute(os_app);
                (*self.app_dlg).set_ivar(app_delegate::APP_VAR_NAME, os_app);
                let gvc: apple::Id = *(*self.app_dlg).get_ivar(app_delegate::CONTROLLER_VAR_NAME);
                (*gvc).set_ivar(game_view_controller::APP_VAR_NAME, os_app);
                let _: () = msg_send![gvc, startLinkDisplay];
            }
            let _: () = msg_send![self.app, activateIgnoringOtherApps: YES];
            let _: () = msg_send![self.app, run];
            vxlogi!("reached");
        }
    }

    pub fn update(&self) {
        vxresult!(vxunwrap!(self.renderer).read()).update();
        // vxlogi!("reached");
    }

    pub fn get_window_aspect_ratio(&self) -> f32 {
        let view: apple::Id = unsafe { transmute(self.view) };
        let frame: apple::NSRect = unsafe { msg_send![view, frame] };
        frame.size.width as f32 / frame.size.height as f32
    }

    pub fn set_title(&self, title: &str) {
        app_delegate::set_title(self.app_dlg, title);
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        // maybe some day I need here to do some deinitialization
        // but if it was not needed that day remove the Option from autorelease
        self.auto_release_pool = None;
    }
}
