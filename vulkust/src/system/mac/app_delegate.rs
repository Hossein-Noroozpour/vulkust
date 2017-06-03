use super::super::super::objc::runtime::{Object, Sel, YES, BOOL};
use super::super::metal as mtl;
use super::game_view_controller as gvc;

pub const CLASS_NAME: &str = "AppDelegate";
pub const SUPER_CLASS_NAME: &str = "NSObject";
pub const WINDOW_VAR_NAME: &str = "window";

extern fn initialize(this: &mut Object, _cmd: Sel) {
    let main_screen: mtl::Id = unsafe { msg_send![mtl::get_class("NSScreen"), mainScreen] };
    let frame: mtl::NSRect = unsafe { msg_send![main_screen, frame] };
    let frame = mtl::NSRect::new(0.0, 0.0, frame.size.width / 2.0, frame.size.height / 2.0);
    let style_mask = (
        mtl::NS_TITLED_WINDOW_MASK |
        mtl::NS_CLOSABLE_WINDOW_MASK |
        mtl::NS_RESIZABLE_WINDOW_MASK |
        mtl::NS_MINIATURIZABLE_WINDOW_MASK).bits() as mtl::NSUInteger;
    let backing = mtl::NS_BACKING_STORE_BUFFERED;
    let window: mtl::Id = unsafe { msg_send![mtl::alloc("NSWindow"),
        initWithContentRect:frame styleMask:style_mask backing:backing defer:YES] };
    unsafe { (*this).set_ivar("window", window); }
    unsafe { msg_send![window, center]; }
    let device = mtl::create_system_default_device();
    let game_view = mtl::get_instance(gvc::CLASS_NAME);
    mtl::set_ivar(game_view, gvc::DEVICE_VAR_NAME, device);
    let metal_view: mtl::Id = unsafe { msg_send![mtl::alloc("MTKView"), initWithFrame:frame device:device] };
    let clear_color = mtl::ClearColor::new(0.0, 0.0, 0.0, 1.0);
    let pixel_format = mtl::PIXEL_FORMAT_BGRA8_UNORM;
    let depth_stencil_format = mtl::PIXEL_FORMAT_DEPTH32_FLOAT;
    unsafe {
        msg_send![metal_view, setClearColor:clear_color];
        msg_send![metal_view, setColorPixelFormat:pixel_format];
        msg_send![metal_view, setDepthStencilPixelFormat:depth_stencil_format];
        msg_send![game_view, setView:metal_view];
        msg_send![window, setContentView:metal_view];
        msg_send![window, setContentViewController:game_view];
        msg_send![game_view, viewDidLoad];
    }
    logi!("Reached.");
}

extern fn application_will_finish_launching(this: &Object, _cmd: Sel, _n: mtl::Id) {
    let window: mtl::Id = unsafe { *this.get_ivar("window") };
    unsafe { msg_send![window, makeKeyAndOrderFront:this]; }
}

extern fn application_did_finish_launching(_this: &Object, _cmd: Sel, _n: mtl::Id) {
    // TODO: do your app intialization in here
}

extern fn application_will_terminate(_this: &Object, _cmd: Sel, _n: mtl::Id) {
    // TODO: do your termination in here
}

extern fn application_should_terminate_after_last_window_closed(
    _this: &Object, _cmd: Sel, _sender: mtl::Id) -> BOOL {
    return YES;
}

pub fn register() {
    let ns_object_class = mtl::get_class(SUPER_CLASS_NAME);
    let mut app_delegate_class = mtl::dec_class(CLASS_NAME, ns_object_class);
    app_delegate_class.add_ivar::<mtl::Id>(WINDOW_VAR_NAME);
    unsafe {
        app_delegate_class.add_method(
            sel!(initialize),
            initialize as extern fn(&mut Object, Sel));
        app_delegate_class.add_method(
            sel!(applicationWillFinishLaunching:),
            application_will_finish_launching as extern fn(&Object, Sel, mtl::Id));
        app_delegate_class.add_method(
            sel!(applicationDidFinishLaunching:),
            application_did_finish_launching as extern fn(&Object, Sel, mtl::Id));
        app_delegate_class.add_method(
            sel!(applicationWillTerminate:),
            application_will_terminate as extern fn(&Object, Sel, mtl::Id));
        app_delegate_class.add_method(
            sel!(applicationShouldTerminateAfterLastWindowClosed:),
            application_should_terminate_after_last_window_closed
            as extern fn(&Object, Sel, mtl::Id) -> BOOL);
    }
    app_delegate_class.register();
}
