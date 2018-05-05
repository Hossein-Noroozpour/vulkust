use super::super::super::core::constants::{DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH};
use super::super::super::objc::runtime::{Object, Sel, BOOL, YES};
use super::super::apple;
use super::game_view_controller as gvc;
use std::mem::transmute;

pub const CLASS_NAME: &str = "AppDelegate";
pub const SUPER_CLASS_NAME: &str = "NSObject";
pub const WINDOW_VAR_NAME: &str = "window";
pub const APP_VAR_NAME: &str = "vukust_os_app";

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
    let style_mask = (apple::NS_TITLED_WINDOW_MASK | apple::NS_CLOSABLE_WINDOW_MASK
        | apple::NS_RESIZABLE_WINDOW_MASK
        | apple::NS_MINIATURIZABLE_WINDOW_MASK)
        .bits() as apple::NSUInteger;
    let backing = apple::NS_BACKING_STORE_BUFFERED;
    let window: apple::Id = unsafe {
        msg_send![apple::alloc("NSWindow"),
        initWithContentRect:frame styleMask:style_mask backing:backing defer:YES]
    };
    unsafe {
        let _: () = msg_send![window, center];
    }
    let ns_view: apple::Id = unsafe { msg_send![window, contentView] };
    unsafe {
        (*app).metal_view = metal_view;
    }
    let clear_color = apple::ClearColor::new(0.05, 0.05, 0.05, 1.0);
    let pixel_format = apple::PIXEL_FORMAT_BGRA8_UNORM;
    let depth_stencil_format = apple::PIXEL_FORMAT_DEPTH32_FLOAT;
    unsafe {
        let _: () = msg_send![metal_view, setClearColor: clear_color];
        let _: () = msg_send![metal_view, setColorPixelFormat: pixel_format];
        let _: () = msg_send![metal_view, setDepthStencilPixelFormat: depth_stencil_format];
        let _: () = msg_send![metal_view, setDelegate: game_view];
        let _: () = msg_send![game_view, setView: metal_view];
        let _: () = msg_send![window, setContentView: metal_view];
        let _: () = msg_send![window, setContentViewController: game_view];
        let _: () = msg_send![game_view, viewDidLoad];
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
    app_delegate_class.add_ivar::<apple::NSUInteger>(APP_VAR_NAME);

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
