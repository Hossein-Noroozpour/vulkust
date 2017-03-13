use std::os::raw::{
    c_int,
    c_void,
};

use super::looper::{
    ALooper,
    ALooperCallbackFunc,
};

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum AKeyState {
    Unknown = -1,
    Up = 0,
    Down = 1,
    Virtual = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum AMeta {
    None = 0,
    AltOn = 2,
    LeftOn = 16,
    AltRightOn = 32,
    ShiftOn = 1,
    ShiftLeftOn = 64,
    ShiftRightOn = 128,
    SymOn = 4,
}

pub type AInputEvent = c_void;

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum AInputEventType {
    Key = 1,
    Motion = 2,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum AKeyEventAction {
    Down = 0,
    Up = 1,
    Multiple = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum AKeyEventFlag {
    Here = 1,
    Keyboard = 2,
    KeepTouchMode = 4,
    FromSystem = 8,
    EditorAction = 16,
    Canceled = 32,
    VirtualHardKey = 64,
    LongPress = 128,
    CanceledLongPress = 256,
    Tracking = 512,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum AMotionEventAction {
    Mask = 255,
    PointerIndexMask = 65280,
    Down = 0,
    Up = 1,
    Move = 2,
    Cancel = 3,
    Outside = 4,
    PointerDown = 5,
    PointerUp = 6,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum AMotionEventFlag {
    WindowIsObscured = 1,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum AMotionEventEdgeFlag {
    None = 0,
    Top = 1,
    Bottom = 2,
    Left = 4,
    Right = 8,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum AInputSourceClass {
    Mask = 255,
    Button = 1,
    Pointer = 2,
    Navigation = 4,
    Position = 8,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum AInputSource {
    Unknown = 0,
    Keyboard = 257,
    Dpad = 513,
    Touchscreen = 4098,
    Mouse = 8194,
    Trackball = 65540,
    Touchpad = 1048584,
    Any = 4294967040,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum AInputKeyboardType {
    None = 0,
    NonAlphabetic = 1,
    Alphabetic = 2,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum AinputMotionRange {
    X = 0,
    Y = 1,
    Pressure = 2,
    Size = 3,
    TouchMajor = 4,
    TouchMinor = 5,
    ToolMajor = 6,
    ToolMinor = 7,
    Orientation = 8,
}

pub type AInputQueue = c_void;

#[cfg_attr(target_os = "android", link(name = "android", kind = "dylib"))]
extern {
    pub fn AInputEvent_getType(event: *const AInputEvent) -> i32;
    pub fn AInputEvent_getDeviceId(event: *const AInputEvent) -> i32;
    pub fn AInputEvent_getSource(event: *const AInputEvent) -> i32;
    pub fn AKeyEvent_getAction(key_event: *const AInputEvent) -> i32;
    pub fn AKeyEvent_getFlags(key_event: *const AInputEvent) -> i32;
    pub fn AKeyEvent_getKeyCode(key_event: *const AInputEvent) -> i32;
    pub fn AKeyEvent_getScanCode(key_event: *const AInputEvent) -> i32;
    pub fn AKeyEvent_getMetaState(key_event: *const AInputEvent) -> i32;
    pub fn AKeyEvent_getRepeatCount(key_event: *const AInputEvent) -> i32;
    pub fn AKeyEvent_getDownTime(key_event: *const AInputEvent) -> i64;
    pub fn AKeyEvent_getEventTime(key_event: *const AInputEvent) -> i64;
    pub fn AMotionEvent_getAction(motion_event: *const AInputEvent) -> i32;
    pub fn AMotionEvent_getFlags(motion_event: *const AInputEvent) -> i32;
    pub fn AMotionEvent_getMetaState(motion_event: *const AInputEvent) -> i32;
    pub fn AMotionEvent_getEdgeFlags(motion_event: *const AInputEvent) -> i32;
    pub fn AMotionEvent_getDownTime(motion_event: *const AInputEvent) -> i64;
    pub fn AMotionEvent_getEventTime(motion_event: *const AInputEvent) -> i64;
    pub fn AMotionEvent_getXOffset(motion_event: *const AInputEvent) -> f32;
    pub fn AMotionEvent_getYOffset(motion_event: *const AInputEvent) -> f32;
    pub fn AMotionEvent_getXPrecision(motion_event: *const AInputEvent) -> f32;
    pub fn AMotionEvent_getYPrecision(motion_event: *const AInputEvent) -> f32;
    pub fn AMotionEvent_getPointerCount(motion_event: *const AInputEvent) -> usize;
    pub fn AMotionEvent_getPointerId(motion_event: *const AInputEvent, pointer_index: usize) -> i32;
    pub fn AMotionEvent_getRawX(motion_event: *const AInputEvent, pointer_index: usize) -> f32;
    pub fn AMotionEvent_getRawY(motion_event: *const AInputEvent, pointer_index: usize) -> f32;
    pub fn AMotionEvent_getX(motion_event: *const AInputEvent, pointer_index: usize) -> f32;
    pub fn AMotionEvent_getY(motion_event: *const AInputEvent, pointer_index: usize) -> f32;
    pub fn AMotionEvent_getPressure(motion_event: *const AInputEvent, pointer_index: usize) -> f32;
    pub fn AMotionEvent_getSize(motion_event: *const AInputEvent, pointer_index: usize) -> f32;
    pub fn AMotionEvent_getTouchMajor(motion_event: *const AInputEvent, pointer_index: usize) -> f32;
    pub fn AMotionEvent_getTouchMinor(motion_event: *const AInputEvent, pointer_index: usize) -> f32;
    pub fn AMotionEvent_getToolMajor(motion_event: *const AInputEvent, pointer_index: usize) -> f32;
    pub fn AMotionEvent_getToolMinor(motion_event: *const AInputEvent, pointer_index: usize) -> f32;
    pub fn AMotionEvent_getOrientation(motion_event: *const AInputEvent, pointer_index: usize) -> f32;
    pub fn AMotionEvent_getHistorySize(motion_event: *const AInputEvent) -> usize;
    pub fn AMotionEvent_getHistoricalEventTime(motion_event: *const AInputEvent, history_index: usize) -> i64;
    pub fn AMotionEvent_getHistoricalRawX(motion_event: *const AInputEvent, pointer_index: usize, history_index: usize) -> f32;
    pub fn AMotionEvent_getHistoricalRawY(motion_event: *const AInputEvent, pointer_index: usize, history_index: usize) -> f32;
    pub fn AMotionEvent_getHistoricalX(motion_event: *const AInputEvent, pointer_index: usize, history_index: usize) -> f32;
    pub fn AMotionEvent_getHistoricalY(motion_event: *const AInputEvent, pointer_index: usize, history_index: usize) -> f32;
    pub fn AMotionEvent_getHistoricalPressure(motion_event: *const AInputEvent, pointer_index: usize, history_index: usize) -> f32;
    pub fn AMotionEvent_getHistoricalSize(motion_event: *const AInputEvent, pointer_index: usize, history_index: usize) -> f32;
    pub fn AMotionEvent_getHistoricalTouchMajor(motion_event: *const AInputEvent, pointer_index: usize, history_index: usize) -> f32;
    pub fn AMotionEvent_getHistoricalTouchMinor(motion_event: *const AInputEvent, pointer_index: usize, history_index: usize) -> f32;
    pub fn AMotionEvent_getHistoricalToolMajor(motion_event: *const AInputEvent, pointer_index: usize, history_index: usize) -> f32;
    pub fn AMotionEvent_getHistoricalToolMinor(motion_event: *const AInputEvent, pointer_index: usize, history_index: usize) -> f32;
    pub fn AMotionEvent_getHistoricalOrientation(motion_event: *const AInputEvent, pointer_index: usize, history_index: usize) -> f32;
    pub fn AInputQueue_attachLooper(queue: *mut AInputQueue, looper: *mut ALooper, ident: c_int, callback: ALooperCallbackFunc, data: *mut c_void);
    pub fn AInputQueue_detachLooper(queue: *mut AInputQueue);
    pub fn AInputQueue_hasEvents(queue: *mut AInputQueue) -> i32;
    pub fn AInputQueue_getEvent(queue: *mut AInputQueue, out_event: *mut *mut AInputEvent) -> i32;
    pub fn AInputQueue_preDispatchEvent(queue: *mut AInputQueue, event: *mut AInputEvent) -> i32;
    pub fn AInputQueue_finishEvent(queue: *mut AInputQueue, event: *mut AInputEvent, handled: c_int);
}
