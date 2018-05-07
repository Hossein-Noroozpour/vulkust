use super::super::super::core::constants::{DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH};
use super::super::super::objc::runtime::{Object, Sel, BOOL, YES};
use super::super::apple;
use super::game_view;
use super::game_view_controller;
use std::mem::transmute;
use std::os::raw::c_void;

pub const CLASS_NAME: &str = "AppDelegate";
pub const SUPER_CLASS_NAME: &str = "NSObject";
pub const WINDOW_VAR_NAME: &str = "window";
pub const VIEW_VAR_NAME: &str = "view";
pub const CONTROLLER_VAR_NAME: &str = "controller";
pub const APP_VAR_NAME: &str = "os_app";

#[cfg(debug_assertions)]
fn create_frame() -> apple::NSRect {
    apple::NSRect::new(0.0, 0.0, DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT)
}

#[cfg(not(debug_assertions))]
fn create_frame() -> apple::NSRect {
    let main_screen: apple::Id = unsafe { msg_send![apple::get_class("NSScreen"), mainScreen] };
    let frame: apple::NSRect = unsafe { msg_send![main_screen, frame] };
    apple::NSRect::new(0.0, 0.0, frame.size.width, frame.size.height)
}

extern "C" fn initialize(this: &mut Object, _cmd: Sel) {
    vxlogi!("I'm initialized.");
    let frame = create_frame();
    let style_mask = (apple::NsWindowStyleMask::NS_TITLED_WINDOW_MASK
        | apple::NsWindowStyleMask::NS_CLOSABLE_WINDOW_MASK
        | apple::NsWindowStyleMask::NS_RESIZABLE_WINDOW_MASK
        | apple::NsWindowStyleMask::NS_MINIATURIZABLE_WINDOW_MASK)
        .bits() as apple::NSUInteger;
    let backing = apple::NsBackingStoreType::NS_BACKING_STORE_BUFFERED;
    let window: apple::Id = unsafe {
        msg_send![apple::alloc("NSWindow"),
        initWithContentRect:frame styleMask:style_mask backing:backing defer:YES]
    };
    unsafe {
        let _: () = msg_send![window, center];
    }
    let view = game_view::create_instance(frame);
    let gvc = game_view_controller::create_instance();
    unsafe {
        let os_app: *mut c_void = *this.get_ivar(APP_VAR_NAME);
        (*gvc).set_ivar(game_view_controller::APP_VAR_NAME, os_app);
        this.set_ivar(WINDOW_VAR_NAME, window);
        this.set_ivar(VIEW_VAR_NAME, view);
        this.set_ivar(CONTROLLER_VAR_NAME, gvc);
        let _: () = msg_send![gvc, setView: view];
        let _: () = msg_send![window, setContentView: view];
        let _: () = msg_send![window, setContentViewController: gvc];
        let _: () = msg_send![gvc, gameViewDidLoad];
    }
}

extern "C" fn application_will_finish_launching(this: &Object, _cmd: Sel, _n: apple::Id) {
    vxlogi!("Reached");
}

extern "C" fn application_did_finish_launching(_this: &Object, _cmd: Sel, _n: apple::Id) {
    vxlogi!("Reached");
}

extern "C" fn application_will_terminate(_this: &Object, _cmd: Sel, _n: apple::Id) {
    vxlogi!("Reached");
}

extern "C" fn application_should_terminate_after_last_window_closed(
    _this: &Object,
    _cmd: Sel,
    _sender: apple::Id,
) -> BOOL {
    vxlogi!("Reached");
    return YES;
}

pub fn register() {
    let ns_object_class = apple::get_class(SUPER_CLASS_NAME);
    let mut app_delegate_class = apple::dec_class(CLASS_NAME, ns_object_class);
    app_delegate_class.add_ivar::<apple::Id>(WINDOW_VAR_NAME);
    app_delegate_class.add_ivar::<apple::Id>(VIEW_VAR_NAME);
    app_delegate_class.add_ivar::<apple::Id>(CONTROLLER_VAR_NAME);
    app_delegate_class.add_ivar::<*mut c_void>(APP_VAR_NAME);

    unsafe {
        app_delegate_class.add_method(
            sel!(initialize),
            initialize as extern "C" fn(&mut Object, Sel),
        );
        app_delegate_class.add_method(
            sel!(applicationWillFinishLaunching:),
            application_will_finish_launching as extern "C" fn(&Object, Sel, apple::Id),
        );
        app_delegate_class.add_method(
            sel!(applicationDidFinishLaunching:),
            application_did_finish_launching as extern "C" fn(&Object, Sel, apple::Id),
        );
        app_delegate_class.add_method(
            sel!(applicationWillTerminate:),
            application_will_terminate as extern "C" fn(&Object, Sel, apple::Id),
        );
        app_delegate_class.add_method(
            sel!(applicationShouldTerminateAfterLastWindowClosed:),
            application_should_terminate_after_last_window_closed
                as extern "C" fn(&Object, Sel, apple::Id) -> BOOL,
        );
    }
    app_delegate_class.register();
}

pub fn create_instance() -> apple::Id {
    apple::get_instance(CLASS_NAME)
}
