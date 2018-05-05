use super::super::apple;
use super::app_delegate;
use super::game_view;
use super::game_view_controller;
use std::mem::transmute;
use std::os::raw::c_void;
use std::ptr::null_mut;

pub struct Application {
    pub ns_app: apple::Id,
    pub app_dlg: apple::Id,
    pub gvc: apple::Id,
    pub ns_view: apple::Id,
    pub ns_autopool: apple::NsAutoReleasePool,
}

impl Application {
    fn new() -> Self {
        let ns_autopool = apple::NsAutoReleasePool::new();
        app_delegate::register();
        game_view::register();
        game_view_controller::register();
        let ns_app = apple::get_class("NSApplication");
        let ns_app: apple::Id = unsafe { msg_send![ns_app, sharedApplication] };
        let app_dlg = app_delegate::create_instance();
        unsafe {
            let _: () = msg_send![app_delegate, initialize];
        }
        let view: apple::Id = unsafe { msg_send![self.game_view_controller, view] };
        unsafe {
            let _: () = msg_send![view, setDelegate:self.game_view_controller];
        }
        Application {
            ns_application,
            app_delegate,
            game_view_controller,
            ns_view,
            ns_autorelease_pool,
        }
    }
    fn set_core_app(&mut self, c: *mut CoreApp) {
        self.core_app = c;
    }
    fn set_rnd_eng(&mut self, r: *mut RenderEngine<CoreApp>) {
        self.render_engine = r;
    }
    fn execute(&mut self) -> bool {
        unsafe {
            let _: () = msg_send![self.game_view_controller, metalViewDidLoad];
            let _: () = msg_send![self.ns_application, setDelegate:self.app_delegate];
            let _: () = msg_send![self.ns_application, run];
        }
        true
    }
    fn get_mouse_position(&mut self) -> (f64, f64) {
        logf!("Unimplemented!");
    }
}
