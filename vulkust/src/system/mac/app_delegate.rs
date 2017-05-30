use super::super::super::objc;
use super::super::super::objc::runtime::{Object, Sel, YES};
use super::types::{
    Id, NSRect, NSUInteger,
    NS_TITLED_WINDOW_MASK,
    NS_CLOSABLE_WINDOW_MASK,
    NS_RESIZABLE_WINDOW_MASK,
    NS_MINIATURIZABLE_WINDOW_MASK,
    NS_BACKING_STORE_BUFFERED,
};

extern fn init(_this: &Object, _cmd: Sel) -> Id {
    let ns_object_class = get_class!("NSObject");
    let this: Id = unsafe { msg_send![ns_object_class, init] };
    let ns_screen_class = get_class!("NSScreen");
    let main_screen: Id = unsafe { msg_send![ns_screen_class, mainScreen] };
    let frame: NSRect = unsafe { *(*main_screen).get_ivar("frame") };
    let style_mask = (
        NS_TITLED_WINDOW_MASK |
        NS_CLOSABLE_WINDOW_MASK |
        NS_RESIZABLE_WINDOW_MASK |
        NS_MINIATURIZABLE_WINDOW_MASK).bits() as NSUInteger;
    let backing = NS_BACKING_STORE_BUFFERED;
    let window = get_class!("NSWindow");
    let window: Id = unsafe { msg_send![window, alloc] };
    let window: Id = unsafe { msg_send![window,
        initWithContentRect:frame styleMask:style_mask backing:backing defer:YES] };
    unsafe {(*this).set_ivar("window", window);}
    unsafe { msg_send![window, center]; }





    return this as Id;
}

extern fn application_will_finish_launching(this: &Object, _cmd: Sel, _notification: Id) {
    let window: Id = unsafe { *this.get_ivar("window") };
    unsafe { msg_send![window, makeKeyAndOrderFront:this]; }
}

extern fn application_did_finish_launching(_this: &Object, _cmd: Sel, _notification: Id) {
    // TODO: do your app intialization in here
}

extern fn application_will_terminate(_this: &Object, _cmd: Sel, _notification: Id) {
    // TODO: do your termination in here
}

pub fn register() {
    let ns_object_class = get_class!("NSObject");
    let mut app_delegate_class = dec_class!("AppDelegate", ns_object_class);
    app_delegate_class.add_ivar::<Id>("window");
    unsafe {
        app_delegate_class.add_method(
            sel!(init),
            init as extern fn(&Object, Sel) -> Id);
        app_delegate_class.add_method(
            sel!(applicationWillFinishLaunching),
            application_will_finish_launching as extern fn(&Object, Sel, notification: Id));
        app_delegate_class.add_method(
            sel!(applicationDidFinishLaunching),
            application_did_finish_launching as extern fn(&Object, Sel, notification: Id));
        app_delegate_class.add_method(
            sel!(applicationWillTerminate),
            application_will_terminate as extern fn(&Object, Sel, notification: Id));
    }
    app_delegate_class.register();
}
