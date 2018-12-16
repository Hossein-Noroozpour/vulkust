use super::super::super::core::event;
use super::super::super::core::types::Real;
use super::super::apple;
use super::application::Application as OsApp;
use std::mem::transmute;
use std::os::raw::c_void;
use std::sync::{Arc, RwLock};

use objc::runtime::{Object, Sel, BOOL, YES};

pub const CLASS_NAME: &str = "GameViewController";
pub const SUPER_CLASS_NAME: &str = "NSViewController";
pub const DISPLAY_LINK_VAR_NAME: &str = "display_link";
pub const APP_VAR_NAME: &str = "os_app";
pub const APP_DATA_VAR_NAME: &str = "app_data";

struct AppData {
    previous_mouse_position_x: Real,
    previous_mouse_position_y: Real,
}

extern "C" fn display_link_callback(
    _display_link: apple::core_video::CVDisplayLinkRef,
    _in_now: *const apple::core_video::CVTimeStamp,
    _in_output_time: *const apple::core_video::CVTimeStamp,
    _flags_in: apple::core_video::CVOptionFlags,
    _flags_out: *mut apple::core_video::CVOptionFlags,
    display_link_context: *mut c_void,
) -> apple::core_video::CVReturn {
    let os_app: &Arc<RwLock<OsApp>> = unsafe { transmute(display_link_context) };
    vxresult!(os_app.read()).update();
    apple::core_video::KCVRETURN_SUCCESS
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
    }
}

extern "C" fn start_link_display(this: &mut Object, _cmd: Sel) {
    unsafe {
        let display_link: *mut c_void = *this.get_ivar(DISPLAY_LINK_VAR_NAME);
        let display_link: apple::core_video::CVDisplayLinkRef = transmute(display_link);
        let os_app: *mut c_void = *this.get_ivar(APP_VAR_NAME);
        apple::core_video::CVDisplayLinkSetOutputCallback(
            display_link,
            display_link_callback,
            os_app,
        );
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
        let _: () = msg_send![*vxunwrap!(&this.class().superclass()), dealloc];
        let _ = Box::from_raw(os_app);
    }
}

//-(void) keyDown:(NSEvent*) theEvent
extern "C" fn key_down(_this: &mut Object, _cmd: Sel, _event: apple::Id) {
    vxlogi!("keyboard key pressed");
}

// - (void)mouseDown:(NSEvent *)event
extern "C" fn mouse_down(this: &mut Object, _cmd: Sel, e: apple::Id) {
    vxlogi!("PPPPPPP");
    let et: apple::NSUInteger = unsafe { msg_send![e, type] };
    let mut e: Option<event::Type> = None;
    if et == apple::app_kit::NSEventType::NS_EVENT_TYPE_LEFT_MOUSE_DOWN.bits() {
        e = Some(event::Type::Button {
            button: event::Button::Mouse(event::Mouse::Left),
            action: event::ButtonAction::Press,
        });
    }
    let os_app: *mut c_void = unsafe { *this.get_ivar(APP_VAR_NAME) };
    let os_app: &'static mut Arc<RwLock<OsApp>> = unsafe { transmute(os_app) };
    let core_app = vxresult!(os_app.read());
    let core_app = core_app.get_core_app();
    let core_app = vxresult!(vxunwrap!(core_app).read());
    if let Some(e) = e {
        core_app.on_event(event::Event::new(e));
    }
}

// - (void)mouseUp:(NSEvent *)event
extern "C" fn mouse_up(this: &mut Object, _cmd: Sel, e: apple::Id) {
    vxlogi!("RRRRRRRR");
    let et: apple::NSUInteger = unsafe { msg_send![e, type] };
    let mut e: Option<event::Type> = None;
    if et == apple::app_kit::NSEventType::NS_EVENT_TYPE_LEFT_MOUSE_UP.bits() {
        e = Some(event::Type::Button {
            button: event::Button::Mouse(event::Mouse::Left),
            action: event::ButtonAction::Release,
        });
    }
    let os_app: *mut c_void = unsafe { *this.get_ivar(APP_VAR_NAME) };
    let os_app: &'static mut Arc<RwLock<OsApp>> = unsafe { transmute(os_app) };
    let core_app = vxresult!(os_app.read());
    let core_app = core_app.get_core_app();
    let core_app = vxresult!(vxunwrap!(core_app).read());
    if let Some(e) = e {
        core_app.on_event(event::Event::new(e));
    }
}

// - (void)mouseMoved:(NSEvent *)event
extern "C" fn mouse_moved(this: &mut Object, _cmd: Sel, _event: apple::Id) {
    let app_data: *mut c_void = unsafe { *this.get_ivar(APP_DATA_VAR_NAME) };
    let app_data: &'static mut AppData = unsafe { transmute(app_data) };
    let mp = get_uniform_mouse_position();
    if mp.0 == app_data.previous_mouse_position_x && mp.1 == app_data.previous_mouse_position_y {
        return;
    }
    let os_app: *mut c_void = unsafe { *this.get_ivar(APP_VAR_NAME) };
    let os_app: &'static mut Arc<RwLock<OsApp>> = unsafe { transmute(os_app) };
    let core_app = vxresult!(os_app.read());
    let core_app = core_app.get_core_app();
    let core_app = vxresult!(vxunwrap!(core_app).read());
    core_app.on_event(event::Event::new(event::Type::Move(event::Move::Mouse {
        previous: (
            app_data.previous_mouse_position_x,
            app_data.previous_mouse_position_y,
        ),
        current: mp,
        delta: (
            mp.0 - app_data.previous_mouse_position_x,
            mp.1 - app_data.previous_mouse_position_y,
        ),
    })));
    app_data.previous_mouse_position_x = mp.0;
    app_data.previous_mouse_position_y = mp.1;
}

// - (void)mouseDragged:(NSEvent *)event
extern "C" fn mouse_dragged(this: &mut Object, cmd: Sel, e: apple::Id) {
    mouse_moved(this, cmd, e);
}

// -(BOOL) acceptsFirstResponder { return YES; }
extern "C" fn accepts_first_responder(_this: &mut Object, _cmd: Sel) -> BOOL {
    vxlogi!("Reached");
    YES
}

// -(BOOL) acceptsMouseMovedEvents
extern "C" fn accepts_mouse_moved_events(_this: &mut Object, _cmd: Sel) -> BOOL {
    vxlogi!("Reached");
    YES
}

pub fn register() {
    let mut self_class = apple::dec_class_s(CLASS_NAME, SUPER_CLASS_NAME);
    self_class.add_ivar::<*mut c_void>(DISPLAY_LINK_VAR_NAME);
    self_class.add_ivar::<*mut c_void>(APP_VAR_NAME);
    self_class.add_ivar::<*mut c_void>(APP_DATA_VAR_NAME);

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
        self_class.add_method(
            sel!(mouseDown:),
            mouse_down as extern "C" fn(&mut Object, Sel, apple::Id),
        );
        self_class.add_method(
            sel!(mouseUp:),
            mouse_up as extern "C" fn(&mut Object, Sel, apple::Id),
        );
        self_class.add_method(
            sel!(mouseMoved:),
            mouse_moved as extern "C" fn(&mut Object, Sel, apple::Id),
        );
        self_class.add_method(
            sel!(mouseDragged:),
            mouse_dragged as extern "C" fn(&mut Object, Sel, apple::Id),
        );
        self_class.add_method(
            sel!(acceptsFirstResponder),
            accepts_first_responder as extern "C" fn(&mut Object, Sel) -> BOOL,
        );
        self_class.add_method(
            sel!(acceptsMouseMovedEvents),
            accepts_mouse_moved_events as extern "C" fn(&mut Object, Sel) -> BOOL,
        );
    }
    self_class.register();
}

pub fn create_instance() -> apple::Id {
    let instance = apple::get_instance(CLASS_NAME);
    let ump = get_uniform_mouse_position();
    let app_data = Box::into_raw(Box::new(AppData {
        previous_mouse_position_x: ump.0,
        previous_mouse_position_y: ump.1,
    }));
    unsafe {
        let app_data: *mut c_void = transmute(app_data);
        (*instance).set_ivar(APP_DATA_VAR_NAME, app_data);
    }
    return instance;
}

pub fn get_uniform_mouse_position() -> (Real, Real) {
    let sr = apple::app_kit::get_screen_rect();
    let mp = apple::app_kit::get_mouse_position();
    return (
        (mp.x / sr.size.height) as Real,
        (mp.y / sr.size.height) as Real,
    );
}
