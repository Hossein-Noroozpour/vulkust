use std::mem::zeroed;
use std::os::raw::{c_char, c_int, c_uint, c_void};

bitflags! {
    pub(super) struct EventMask: u32 {
        const NO_EVENT = 0;
        const KEY_PRESS = 1;
        const KEY_RELEASE = 2;
        const BUTTON_PRESS = 4;
        const BUTTON_RELEASE = 8;
        const ENTER_WINDOW = 16;
        const LEAVE_WINDOW = 32;
        const POINTER_MOTION = 64;
        const POINTER_MOTION_HINT = 128;
        const BUTTON_1_MOTION = 256;
        const BUTTON_2_MOTION = 512;
        const BUTTON_3_MOTION = 1024;
        const BUTTON_4_MOTION = 2048;
        const BUTTON_5_MOTION = 4096;
        const BUTTON_MOTION = 8192;
        const KEYMAP_STATE = 16384;
        const EXPOSURE = 32768;
        const VISIBILITY_CHANGE = 65536;
        const STRUCTURE_NOTIFY = 131072;
        const RESIZE_REDIRECT = 262144;
        const SUBSTRUCTURE_NOTIFY = 524288;
        const SUBSTRUCTURE_REDIRECT = 1048576;
        const FOCUS_CHANGE = 2097152;
        const PROPERTY_CHANGE = 4194304;
        const COLOR_MAP_CHANGE = 8388608;
        const OWNER_GRAB_BUTTON = 16777216;
    }
}

bitflags! {
    pub(super) struct CW: u32 {
        const BACK_PIXMAP = 1;
        const BACK_PIXEL = 2;
        const BORDER_PIXMAP = 4;
        const BORDER_PIXEL = 8;
        const BIT_GRAVITY = 16;
        const WIN_GRAVITY = 32;
        const BACKING_STORE = 64;
        const BACKING_PLANES = 128;
        const BACKING_PIXEL = 256;
        const OVERRIDE_REDIRECT = 512;
        const SAVE_UNDER = 1024;
        const EVENT_MASK = 2048;
        const DONT_PROPAGATE = 4096;
        const COLORMAP = 8192;
        const CURSOR = 16384;
    }
}

#[repr(u32)]
#[derive(Copy, Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
pub(super) enum WindowClass {
    _CopyFromParent = 0,
    InputOutput = 1,
    _InputOnly = 2,
}

#[repr(u32)]
#[derive(Copy, Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
pub(super) enum AtomEnum {
    _None = 0,
    _Primary = 1,
    _Secondary = 2,
    _Arc = 3,
    _Atom = 4,
    _Bitmap = 5,
    _Cardinal = 6,
    _Colormap = 7,
    _Cursor = 8,
    _CutBuffer0 = 9,
    _CutBuffer1 = 10,
    _CutBuffer2 = 11,
    _CutBuffer3 = 12,
    _CutBuffer4 = 13,
    _CutBuffer5 = 14,
    _CutBuffer6 = 15,
    _CutBuffer7 = 16,
    _Drawable = 17,
    _Font = 18,
    _Integer = 19,
    _Pixmap = 20,
    _Point = 21,
    _Rectangle = 22,
    _ResourceManager = 23,
    _RgbColorMap = 24,
    _RgbBestMap = 25,
    _RgbBlueMap = 26,
    _RgbDefaultMap = 27,
    _RgbGrayMap = 28,
    _RgbGreenMap = 29,
    _RgbRedMap = 30,
    String = 31,
    _VisualId = 32,
    _Window = 33,
    _WmCommand = 34,
    _WmHints = 35,
    _WmClientMachine = 36,
    _WmIconName = 37,
    _WmIconSize = 38,
    WmName = 39,
    _WmNormalHints = 40,
    _WmSizeHints = 41,
    _WmZoomHints = 42,
    _MinSpace = 43,
    _NormSpace = 44,
    _MaxSpace = 45,
    _EndSpace = 46,
    _SuperscriptX = 47,
    _SuperscriptY = 48,
    _SubscriptX = 49,
    _SubscriptY = 50,
    _UnderlinePosition = 51,
    _UnderlineThickness = 52,
    _StrikeoutAscent = 53,
    _StrikeoutDescent = 54,
    _ItalicAngle = 55,
    _XHeight = 56,
    _QuadWidth = 57,
    _Weight = 58,
    _PointSize = 59,
    _Resolution = 60,
    _Copyright = 61,
    _Notice = 62,
    _FontName = 63,
    _FamilyName = 64,
    _FullName = 65,
    _CapHeight = 66,
    _WmClass = 67,
    _WmTransientFor = 68,
}

#[repr(u32)]
#[derive(Copy, Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
pub(super) enum ButtonIndex {
    _IndexAny = 0,
    _Index1 = 1,
    _Index2 = 2,
    _Index3 = 3,
    _Index4 = 4,
    _Index5 = 5,
}

#[derive(Copy, Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) enum Connection {}

pub(crate) type Window = u32;
pub(super) type ColorMap = u32;
pub(super) type VisualId = u32;
pub(super) type Atom = u32;
pub(super) type KeyCode = u8;
pub(super) type Button = u8;
pub(super) type TimeStamp = u32;
pub(super) type ButtonReleaseEvent = ButtonPressEvent;
pub(super) type KeyReleaseEvent = KeyPressEvent;

#[repr(C)]
#[derive(Copy, Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
pub(super) struct Screen {
    pub(super) root: Window,
    pub(super) default_colormap: ColorMap,
    pub(super) white_pixel: u32,
    pub(super) black_pixel: u32,
    pub(super) current_input_masks: u32,
    pub(super) width_in_pixels: u16,
    pub(super) height_in_pixels: u16,
    pub(super) width_in_millimeters: u16,
    pub(super) height_in_millimeters: u16,
    pub(super) min_installed_maps: u16,
    pub(super) max_installed_maps: u16,
    pub(super) root_visual: VisualId,
    pub(super) backing_stores: u8,
    pub(super) save_unders: u8,
    pub(super) root_depth: u8,
    pub(super) allowed_depths_len: u8,
}

impl Default for Screen {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
pub(super) struct InternAtomReply {
    pub(super) response_type: u8,
    pub(super) pad0: u8,
    pub(super) sequence: u16,
    pub(super) length: u32,
    pub(super) atom: Atom,
}

impl Default for InternAtomReply {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
pub(super) struct Setup {
    pub(super) status: u8,
    pub(super) pad0: u8,
    pub(super) protocol_major_version: u16,
    pub(super) protocol_minor_version: u16,
    pub(super) length: u16,
    pub(super) release_number: u32,
    pub(super) resource_id_base: u32,
    pub(super) resource_id_mask: u32,
    pub(super) motion_buffer_size: u32,
    pub(super) vendor_len: u16,
    pub(super) maximum_request_length: u16,
    pub(super) roots_len: u8,
    pub(super) pixmap_formats_len: u8,
    pub(super) image_byte_order: u8,
    pub(super) bitmap_format_bit_order: u8,
    pub(super) bitmap_format_scanline_unit: u8,
    pub(super) bitmap_format_scanline_pad: u8,
    pub(super) min_keycode: KeyCode,
    pub(super) max_keycode: KeyCode,
    pub(super) pad1: [u8; 4usize],
}

impl Default for Setup {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
pub(super) struct ScreenIterator {
    pub(super) data: *mut Screen,
    pub(super) rem: c_int,
    pub(super) index: c_int,
}

impl Default for ScreenIterator {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
pub(super) struct VoidCookie {
    pub(super) sequence: c_uint,
}

impl Default for VoidCookie {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
pub(super) struct InternAtomCookie {
    pub(super) sequence: c_uint,
}

impl Default for InternAtomCookie {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
pub(super) struct GenericError {
    pub(super) response_type: u8,
    pub(super) error_code: u8,
    pub(super) sequence: u16,
    pub(super) resource_id: u32,
    pub(super) minor_code: u16,
    pub(super) major_code: u8,
    pub(super) pad0: u8,
    pub(super) pad: [u32; 5usize],
    pub(super) full_sequence: u32,
}

impl Default for GenericError {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
pub(super) struct GenericEvent {
    pub(super) response_type: u8,
    pub(super) pad0: u8,
    pub(super) sequence: u16,
    pub(super) pad: [u32; 7usize],
    pub(super) full_sequence: u32,
}

impl Default for GenericEvent {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
pub(super) struct ClientMessageEvent {
    pub(super) response_type: u8,
    pub(super) format: u8,
    pub(super) sequence: u16,
    pub(super) window: Window,
    pub(super) type_: Atom,
    pub(super) data: ClientMessageData,
}

impl Default for ClientMessageEvent {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
pub(super) struct ClientMessageData {
    pub(super) data: [u32; 5usize],
}

impl Default for ClientMessageData {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
pub(super) struct ButtonPressEvent {
    pub(super) response_type: u8,
    pub(super) detail: Button,
    pub(super) sequence: u16,
    pub(super) time: TimeStamp,
    pub(super) root: Window,
    pub(super) event: Window,
    pub(super) child: Window,
    pub(super) root_x: i16,
    pub(super) root_y: i16,
    pub(super) event_x: i16,
    pub(super) event_y: i16,
    pub(super) state: u16,
    pub(super) same_screen: u8,
    pub(super) pad0: u8,
}

impl Default for ButtonPressEvent {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
pub(super) struct KeyPressEvent {
    pub(super) response_type: u8,
    pub(super) detail: KeyCode,
    pub(super) sequence: u16,
    pub(super) time: TimeStamp,
    pub(super) root: Window,
    pub(super) event: Window,
    pub(super) child: Window,
    pub(super) root_x: i16,
    pub(super) root_y: i16,
    pub(super) event_x: i16,
    pub(super) event_y: i16,
    pub(super) state: u16,
    pub(super) same_screen: u8,
    pub(super) pad0: u8,
}

impl Default for KeyPressEvent {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
pub(super) struct ConfigureNotifyEvent {
    pub(super) response_type: u8,
    pub(super) pad0: u8,
    pub(super) sequence: u16,
    pub(super) event: Window,
    pub(super) window: Window,
    pub(super) above_sibling: Window,
    pub(super) x: i16,
    pub(super) y: i16,
    pub(super) width: u16,
    pub(super) height: u16,
    pub(super) border_width: u16,
    pub(super) override_redirect: u8,
    pub(super) pad1: u8,
}

impl Default for ConfigureNotifyEvent {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
pub(super) struct QueryPointerCookie {
    pub(super) sequence: c_uint,
}

impl Default for QueryPointerCookie {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
pub(super) struct QueryPointerReply {
    pub(super) response_type: u8,
    pub(super) same_screen: u8,
    pub(super) sequence: u16,
    pub(super) length: u32,
    pub(super) root: Window,
    pub(super) child: Window,
    pub(super) root_x: i16,
    pub(super) root_y: i16,
    pub(super) win_x: i16,
    pub(super) win_y: i16,
    pub(super) mask: u16,
    pub(super) pad0: [u8; 2usize],
}

impl Default for QueryPointerReply {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}

pub(super) const COPY_FROM_PARENT: u64 = 0;

#[link(name = "xcb", kind = "dylib")]
extern "C" {
    pub(super) fn xcb_connect(displayname: *const c_char, screenp: *mut c_int) -> *mut Connection;
    pub(super) fn xcb_get_setup(c: *mut Connection) -> *const Setup;
    pub(super) fn xcb_setup_roots_iterator(R: *const Setup) -> ScreenIterator;
    pub(super) fn xcb_screen_next(i: *mut ScreenIterator);
    pub(super) fn xcb_generate_id(c: *mut Connection) -> u32;
    pub(super) fn xcb_create_window(
        c: *mut Connection,
        depth: u8,
        wid: Window,
        parent: Window,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        class: u16,
        visual: VisualId,
        value_mask: u32,
        value_list: *const u32,
    ) -> VoidCookie;
    pub(super) fn xcb_intern_atom(
        c: *mut Connection,
        only_if_exists: u8,
        name_len: u16,
        name: *const c_char,
    ) -> InternAtomCookie;
    pub(super) fn xcb_intern_atom_reply(
        c: *mut Connection,
        cookie: InternAtomCookie,
        e: *mut *mut GenericError,
    ) -> *mut InternAtomReply;
    pub(super) fn xcb_change_property(
        c: *mut Connection,
        mode: u8,
        window: Window,
        property: Atom,
        type_: Atom,
        format: u8,
        data_len: u32,
        data: *const c_void,
    ) -> VoidCookie;
    pub(super) fn xcb_map_window(c: *mut Connection, window: Window) -> VoidCookie;
    pub(super) fn xcb_flush(c: *mut Connection) -> c_int;
    pub(super) fn xcb_poll_for_event(c: *mut Connection) -> *mut GenericEvent;
    pub(super) fn xcb_query_pointer(c: *mut Connection, window: Window) -> QueryPointerCookie;
    pub(super) fn xcb_query_pointer_reply(
        c: *mut Connection,
        cookie: QueryPointerCookie,
        e: *mut *mut GenericError,
    ) -> *mut QueryPointerReply;
}
