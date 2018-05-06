use super::super::super::objc::runtime::{Object, Sel, YES};
use super::super::apple;
use super::application::Application as OsApp;
use std::mem::transmute;
use std::os::raw::c_void;
use std::ptr::null_mut;
use std::sync::{Arc, RwLock};

pub const CLASS_NAME: &str = "GameViewController";
pub const SUPER_CLASS_NAME: &str = "NSViewController";
pub const DISPLAY_LINK_VAR_NAME: &str = "display_link";
pub const APP_VAR_NAME: &str = "os_app";

extern "C" fn display_link_callback(
    _display_link: apple::core_video::CVDisplayLinkRef,
    _in_now: *const apple::core_video::CVTimeStamp,
    _in_output_time: *const apple::core_video::CVTimeStamp,
    _flags_in: apple::core_video::CVOptionFlags,
    _flags_out: *mut apple::core_video::CVOptionFlags,
    display_link_context: *mut c_void,
) -> apple::core_video::CVReturn {
    let os_app: &Arc<RwLock<OsApp>> = unsafe { transmute(display_link_context) };
    vxresult(os_app.read()).update();
    apple::core_video::KCVReturnSuccess
}

//- (void)gameViewDidLoad
extern "C" fn game_view_did_load(this: &mut Object, _cmd: Sel) {
    let _: () = unsafe { msg_send![this, viewDidLoad] };
    let view: apple::Id = unsafe { msg_send![this, view] };
    let _: () = unsafe { msg_send![view, setWantsLayer: YES] };
    let mut display_link = 0 as apple::core_video::CVDisplayLinkRef;
    unsafe {
        apple::core_video::CVDisplayLinkCreateWithActiveCGDisplays(&mut display_link);
        let display_link_var: *mut c_void = transmute(display_link);
        this.set_ivar(DISPLAY_LINK_VAR_NAME, display_link_var);
        let os_app = *this.get_ivar(APP_VAR_NAME);
        apple::core_video::CVDisplayLinkSetOutputCallback(
            display_link,
            display_link_callback,
            os_app,
        );
        apple::core_video::CVDisplayLinkStart(display_link);
    }
}

extern "C" fn start_link_display(this: &mut Object, _cmd: Sel) {
    unsafe {
        let display_link: *mut c_void = *this.get_ivar(DISPLAY_LINK_VAR_NAME);
        let display_link: apple::core_video::CVDisplayLinkRef = transmute(display_link);
        apple::core_video::CVDisplayLinkStart(display_link);
    }
}

// -(void) dealloc
extern "C" fn deallocate(this: &mut Object, _cmd: Sel) {
    let display_link: *mut c_void = unsafe { *this.get_ivar(DISPLAY_LINK_VAR_NAME) };
    let display_link: apple::core_video::CVDisplayLinkRef = unsafe { transmute(display_link) };
    unsafe {
        apple::core_video::CVDisplayLinkRelease(display_link);
        let os_app: *mut c_void = *this.get_ivar(APP_VAR_NAME);
        let os_app: *mut Arc<RwLock<OsApp>> = transmute(os_app);
        let _: () = msg_send![*vxunwrap!(this.class().superclass()), dealloc];
        let _ = Box::from_raw(os_app);
    }
}

//-(void) keyDown:(NSEvent*) theEvent
extern "C" fn key_down(_this: &mut Object, _cmd: Sel, _event: apple::Id) {}

pub fn register() {
    let mut self_class = apple::dec_class_s(CLASS_NAME, SUPER_CLASS_NAME);
    self_class.add_ivar::<*mut c_void>(DISPLAY_LINK_VAR_NAME);
    self_class.add_ivar::<*mut c_void>(APP_VAR_NAME);

    unsafe {
        self_class.add_method(
            sel!(gameViewDidLoad),
            game_view_did_load as extern "C" fn(&mut Object, Sel),
        );
        self_class.add_method(
            sel!(startLinkDisplay),
            start_link_display as extern "C" fn(&mut Object, Sel),
        );
        self_class.add_method(sel!(dealloc), deallocate as extern "C" fn(&mut Object, Sel));
        self_class.add_method(
            sel!(keyDown:),
            key_down as extern "C" fn(&mut Object, Sel, apple::Id),
        );
    }
    self_class.register();
}

pub fn create_instance() -> apple::Id {
    apple::get_instance(CLASS_NAME)
}
