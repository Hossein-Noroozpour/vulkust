extern crate libc;

#[cfg(target_os = "linux")]
use ::system::xcb::{
    xcb_cw_t,
    xcb_flush,
    xcb_setup_t,
    xcb_connect,
    xcb_get_setup,
    xcb_map_window,
    xcb_generate_id,
    xcb_screen_next,
    xcb_intern_atom,
    xcb_prop_mode_t,
    xcb_connection_t,
    xcb_event_mask_t,
    xcb_create_window,
    xcb_window_class_t,
    xcb_config_window_t,
    xcb_generic_error_t,
    xcb_change_property,
    xcb_configure_window,
    xcb_intern_atom_reply,
    xcb_screen_iterator_t,
    xcb_intern_atom_cookie_t,
    xcb_setup_roots_iterator,

    XCB_COPY_FROM_PARENT,
};

use ::system::vulkan::{
    VkRect2D,
    VkExtent2D,
    VkOffset2D,
};

use std::default::Default;
use std::ffi::CString;
use std::mem::transmute;
use std::os::raw::{
    c_int,
    c_uint,
    c_char,
    c_void,
};

pub struct Window {

}

impl Window {
    #[cfg(target_os = "linux")]
    pub fn new(width: u32, height: u32) -> Self {
        let mut setup = 0 as *const xcb_setup_t;
        let mut iter = xcb_screen_iterator_t::default();
        let mut screen = 0 as c_int;
        let xcb_connection = unsafe { xcb_connect(0 as *const c_char, &mut screen as *mut c_int) };
        if xcb_connection == (0 as *mut xcb_connection_t) {
            panic!("Cannot find a compatible Vulkan ICD.");
        }
        setup = unsafe {xcb_get_setup(xcb_connection) };
        iter = unsafe { xcb_setup_roots_iterator(setup) };
        for _ in 0..screen {
            unsafe { xcb_screen_next(&mut iter as *mut xcb_screen_iterator_t); }
        }
        let mut xcb_screen = iter.data;
        let dimensions = VkRect2D {
            offset: VkOffset2D {
                x: 0,
                y: 0,
            },
            extent: VkExtent2D {
                width: width,
                height: height
            },
        };
        let mut value_mask: c_uint = 0;
        let mut value_list = [0 as c_uint; 32];
        let mut xcb_window = unsafe { xcb_generate_id(xcb_connection) };
        value_mask = (xcb_cw_t::XCB_CW_BACK_PIXEL as c_uint) |
            (xcb_cw_t::XCB_CW_EVENT_MASK as c_uint);
        value_list[0] = unsafe { (*xcb_screen).black_pixel };
        value_list[1] = (xcb_event_mask_t::XCB_EVENT_MASK_KEY_RELEASE as c_uint) |
            (xcb_event_mask_t::XCB_EVENT_MASK_EXPOSURE as c_uint);
        unsafe {
            xcb_create_window(
                xcb_connection, XCB_COPY_FROM_PARENT as u8, xcb_window, (*xcb_screen).root,
                dimensions.offset.x as i16, dimensions.offset.y as i16,
                dimensions.extent.width as u16, dimensions.extent.height as u16, 0,
                xcb_window_class_t::XCB_WINDOW_CLASS_INPUT_OUTPUT as u16, (*xcb_screen).root_visual,
                value_mask, value_list.as_ptr() as *const u32);
        }
        let wm_protocols = CString::new("WM_PROTOCOLS").unwrap();
        let mut cookie = unsafe { xcb_intern_atom(xcb_connection, 1, 12, wm_protocols.as_ptr()) };
        let reply = unsafe { xcb_intern_atom_reply(
            xcb_connection, cookie, 0 as *mut *mut xcb_generic_error_t) };
        let wm_delete_window = CString::new("WM_DELETE_WINDOW").unwrap();
        let cookie2 = unsafe { xcb_intern_atom(xcb_connection, 0, 16, wm_delete_window.as_ptr()) };
        let xcb_atom_window_reply = unsafe { xcb_intern_atom_reply(
            xcb_connection, cookie2, 0 as *mut *mut xcb_generic_error_t) };
        unsafe {
            xcb_change_property(
                xcb_connection, xcb_prop_mode_t::XCB_PROP_MODE_REPLACE as u8, xcb_window,
                (*reply).atom, 4, 32, 1, transmute(&(*xcb_atom_window_reply).atom));
        }
        unsafe { libc::free(reply as *mut libc::c_void) };
        unsafe { xcb_map_window(xcb_connection, xcb_window) };
        let coords = [100 as c_uint; 2];
        unsafe {
            xcb_configure_window(
                xcb_connection, xcb_window,
                (xcb_config_window_t::XCB_CONFIG_WINDOW_X as u16) |
                    (xcb_config_window_t::XCB_CONFIG_WINDOW_Y as u16),
                coords.as_ptr() as *const u32);
        }
        unsafe {
            xcb_flush(xcb_connection);
        }
        Window {}
    }
}