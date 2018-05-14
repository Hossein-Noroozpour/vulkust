use super::super::super::objc::runtime::{Object, Sel, YES};
use super::super::apple;
use super::application::Application as OsApp;
use std::mem::{transmute, transmute_copy};
use std::os::raw::c_void;
use std::ptr::null_mut;
use std::sync::{Arc, RwLock};

pub const CLASS_NAME: &str = "GameViewController";
pub const SUPER_CLASS_NAME: &str = "UIViewController";
pub const DISPLAY_LINK_VAR_NAME: &str = "display_link";
pub const APP_VAR_NAME: &str = "os_app";

// -(void) renderFrame
extern "C" fn render_frame(this: &mut Object, _cmd: Sel) {
    vxlogi!("Reached");
}

//- (void)viewDidLoad
extern "C" fn game_view_did_load(this: &mut Object, _cmd: Sel) {
    let _: () = unsafe { msg_send![this, viewDidLoad] };
    let view: apple::Id = unsafe { msg_send![this, view] };
    let main_screen: apple::Id = unsafe { msg_send![apple::get_class("UIScreen"), mainScreen] };
    let native_scale: apple::core_graphics::CGFloat =
        unsafe { msg_send![main_screen, nativeScale] };
    let _: () = unsafe { msg_send![view, setContentScaleFactor: native_scale] };
    let display_link: apple::Id = unsafe {
        let this: apple::Id = transmute_copy(this);
        msg_send![ apple::get_class("CADisplayLink"), 
            displayLinkWithTarget:this selector:sel!(renderFrame) ]
    };
    unsafe {
        this.set_ivar(DISPLAY_LINK_VAR_NAME, display_link);
    }
    let fps: apple::NSInteger = 30;
    let _: () = unsafe { msg_send![display_link, setPreferredFramesPerSecond: fps] };
    let cur_loop: apple::Id = unsafe { msg_send![apple::get_class("NSRunLoop"), currentRunLoop] };
    let _: () = unsafe {
        msg_send![display_link, addToRunLoop:cur_loop forMode: apple::NSDefaultRunLoopMode]
    };
}

pub fn register() {
    let mut self_class = apple::dec_class_s(CLASS_NAME, SUPER_CLASS_NAME);
    self_class.add_ivar::<apple::Id>(DISPLAY_LINK_VAR_NAME);
    self_class.add_ivar::<*mut c_void>(APP_VAR_NAME);

    unsafe {
        self_class.add_method(
            sel!(renderFrame),
            render_frame as extern "C" fn(&mut Object, Sel),
        );
        self_class.add_method(
            sel!(gameViewDidLoad),
            game_view_did_load as extern "C" fn(&mut Object, Sel),
        );
    }
    self_class.register();
}

pub fn create_instance() -> apple::Id {
    apple::get_instance(CLASS_NAME)
}
