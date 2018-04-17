extern crate libc;

use super::xcb;
use super::xproto;
use super::super::super::core::constants::{DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH};
use super::super::super::core::event::{Button, Event, Type as EventType, Keyboard, Mouse};
// use super::super::super::core::asset::manager::Manager as AssetManager;
// use super::super::file::File;

use std::ptr::null_mut;
use std::os::raw::c_int;
use std::mem::transmute;
use std::ffi::CString;

pub struct Application {
    pub connection: *mut xcb::xcb_connection_t,
    pub screen: *mut xcb::xcb_screen_t,
    pub window: xcb::xcb_window_t,
    pub atom_wm_delete_window: *mut xcb::xcb_intern_atom_reply_t,
}

impl Application {
    pub fn new() -> Self {
        let mut scr = 0 as c_int;
        let connection: *mut xcb::xcb_connection_t = unsafe { 
            xcb::xcb_connect(null_mut(), &mut scr) 
        };
        if connection == null_mut() {
            vxlogf!("Could not find a compatible Vulkan ICD!");
        }
        let setup = unsafe { xcb::xcb_get_setup(connection) };
        let mut iter = unsafe { xcb::xcb_setup_roots_iterator(setup) };
        for _ in 0..scr {
            unsafe {
                xcb::xcb_screen_next(&mut iter);
            }
        }
        let screen: *mut xcb::xcb_screen_t = iter.data;
        let window: xcb::xcb_window_t = unsafe { transmute(xcb::xcb_generate_id(connection)) };
        let mut value_list = vec![0u32; 32];
        value_list[0] = unsafe { (*screen).black_pixel };
        value_list[1] = xcb::xcb_event_mask_t::XCB_EVENT_MASK_KEY_RELEASE as u32 |
            xcb::xcb_event_mask_t::XCB_EVENT_MASK_KEY_PRESS as u32 |
            xcb::xcb_event_mask_t::XCB_EVENT_MASK_EXPOSURE as u32 |
            xcb::xcb_event_mask_t::XCB_EVENT_MASK_STRUCTURE_NOTIFY as u32 |
            xcb::xcb_event_mask_t::XCB_EVENT_MASK_POINTER_MOTION as u32 |
            xcb::xcb_event_mask_t::XCB_EVENT_MASK_BUTTON_PRESS as u32 |
            xcb::xcb_event_mask_t::XCB_EVENT_MASK_BUTTON_RELEASE as u32;
        let value_mask =
            xcb::xcb_cw_t::XCB_CW_BACK_PIXEL as u32 | 
            xcb::xcb_cw_t::XCB_CW_EVENT_MASK as u32;
        unsafe {
            xcb::xcb_create_window(
                connection, xcb::XCB_COPY_FROM_PARENT as u8,
                window, (*screen).root, 
                0, 0, 
                DEFAULT_WINDOW_WIDTH as u16, DEFAULT_WINDOW_HEIGHT as u16, 
                0,
                xcb::xcb_window_class_t::XCB_WINDOW_CLASS_INPUT_OUTPUT as u16,
                (*screen).root_visual, value_mask, value_list.as_ptr(),
            );
        }
        /* Magic code that will send notification when window is destroyed */
        let cs = CString::new("WM_PROTOCOLS".to_string().into_bytes()).unwrap();
        let cookie = unsafe { xcb::xcb_intern_atom(connection, 1, 12, cs.as_ptr()) };
        let reply = unsafe { xcb::xcb_intern_atom_reply(connection, cookie, null_mut()) };
        let cs = CString::new("WM_DELETE_WINDOW".to_string().into_bytes()).unwrap();
        let cookie2 = unsafe { xcb::xcb_intern_atom(connection, 0, 16, cs.as_ptr()) };
        let atom_wm_delete_window: *mut xcb::xcb_intern_atom_reply_t = unsafe { 
            xcb::xcb_intern_atom_reply(
                connection, cookie2, null_mut()
            )
        };
        let mouse_previous_state = (0.0, 0.0);
        let window_aspects = (DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT);
        unsafe {
            xcb::xcb_change_property(
                connection,
                xproto::xcb_prop_mode_t::XCB_PROP_MODE_REPLACE as u8,
                window,
                (*reply).atom,
                4,
                32,
                1,
                transmute(&((*atom_wm_delete_window).atom)),
            );
        }
        let cs = CString::new("Vulkust Game Engine Demo".to_string().into_bytes()).unwrap();
        unsafe {
            xcb::xcb_change_property(
                connection,
                xproto::xcb_prop_mode_t::XCB_PROP_MODE_REPLACE as u8,
                window,
                xcb::xcb_atom_enum_t::XCB_ATOM_WM_NAME as u32,
                xcb::xcb_atom_enum_t::XCB_ATOM_STRING as u32,
                8,
                cs.as_bytes_with_nul().len() as u32,
                transmute(cs.as_ptr()),
            );
        }
        unsafe {
            libc::free(transmute(reply));
        }
        unsafe {
            xcb::xcb_map_window(connection, window);
        }
        Application {
            connection,
            screen,
            window,
            atom_wm_delete_window,
        }
    }

    pub fn get_mouse_position(&self) -> (f64, f64) {
        unsafe {
            let coockie = xcb::xcb_query_pointer(self.connection, self.window);
            let reply: &mut xcb::xcb_query_pointer_reply_t = transmute(
                xcb::xcb_query_pointer_reply(self.connection, coockie, null_mut()),
            );
            let x = reply.root_x as f64 / (*self.screen).width_in_pixels as f64;
            let y = reply.root_y as f64 / (*self.screen).height_in_pixels as f64;
            libc::free(transmute(reply));
            (x, y)
        }
    }

    pub fn get_window_ratio(&self) -> f64 {
        unsafe { (*self.screen).width_in_pixels as f64 / (*self.screen).height_in_pixels as f64 }
    }

    pub fn finalize(&self) {
        unsafe {
            xcb::xcb_flush(self.connection);
        }
    }

    pub fn fetch_events(&self) -> Vec<Event> {
        let mut events = Vec::new();
        loop {
            let event = unsafe { xcb::xcb_poll_for_event(self.connection) };
            if event == null_mut() {
                break;
            }
            // events.push(self.translate(event));
            unsafe {
                libc::free(transmute(event));
            }
        }
        return events;
    }

    // fn handle_event(&self, e: *mut xcb::xcb_generic_event_t) {
    //     unsafe {
    //         if (xproto::XCB_DESTROY_NOTIFY as u8 == ((*e).response_type & 0x7f)) ||
    //             ((xproto::XCB_CLIENT_MESSAGE as u8 == ((*e).response_type & 0x7f)) &&
    //                 ((*transmute::<
    //                     *mut xcb::xcb_generic_event_t,
    //                     *mut xcb::xcb_client_message_event_t,
    //                 >(e)).data
    //                     .data[0] == (*self.atom_wm_delete_window).atom))
    //         {
    //             self.is_running = false;
    //         }
    //     }
    //     match unsafe { (*e).response_type as c_uint & 0x7F } {
    //         xproto::XCB_CLIENT_MESSAGE => {
    //             let client_msg: &mut xcb::xcb_client_message_event_t = unsafe { transmute(e) };
    //             if client_msg.data.data[0] == unsafe { (*self.atom_wm_delete_window).atom } {
    //                 self.is_running = false;
    //             }
    //         }
    //         xproto::XCB_MOTION_NOTIFY => {
    //             let pos = self.get_mouse_position();
    //             let e = EventType::MouseMove {
    //                 delta_x: pos.0 - self.mouse_previous_state.0,
    //                 delta_y: self.mouse_previous_state.1 - pos.1,
    //             };
    //             self.mouse_previous_state = pos;
    //             self.core_app.read().unwrap().on_event(Event::new(e));
    //         }
    //         xproto::XCB_BUTTON_PRESS => {
    //             let press: &mut xcb::xcb_button_press_event_t = unsafe { transmute(e) };
    //             let m: xcb::xcb_button_index_t = unsafe { transmute(press.detail as u32) };
    //             let m = match m {
    //                 xcb::xcb_button_index_t::XCB_BUTTON_INDEX_1 => Mouse::Left,
    //                 xcb::xcb_button_index_t::XCB_BUTTON_INDEX_2 => Mouse::Middle,
    //                 xcb::xcb_button_index_t::XCB_BUTTON_INDEX_3 => Mouse::Right,
    //                 _ => {
    //                     vxloge!("Unknown mouse button pressed.");
    //                     Mouse::Left
    //                 }
    //             };
    //             let e = EventType::Press {
    //                 button: Button::Mouse(m),
    //             };
    //             self.core_app.read().unwrap().on_event(Event::new(e));
    //         }
    //         xproto::XCB_BUTTON_RELEASE => {
    //             let release: &mut xcb::xcb_button_release_event_t = unsafe { transmute(e) };
    //             let m: xcb::xcb_button_index_t = unsafe { transmute(release.detail as u32) };
    //             let m = match m {
    //                 xcb::xcb_button_index_t::XCB_BUTTON_INDEX_1 => Mouse::Left,
    //                 xcb::xcb_button_index_t::XCB_BUTTON_INDEX_2 => Mouse::Middle,
    //                 xcb::xcb_button_index_t::XCB_BUTTON_INDEX_3 => Mouse::Right,
    //                 _ => {
    //                     vxloge!("Unknown mouse button pressed.");
    //                     Mouse::Left
    //                 }
    //             };
    //             let e = EventType::Release {
    //                 button: Button::Mouse(m),
    //             };
    //             self.core_app.read().unwrap().on_event(Event::new(e));
    //         }
    //         a @ xproto::XCB_KEY_PRESS | a @ xproto::XCB_KEY_RELEASE => {
    //             let key_event: &xcb::xcb_key_release_event_t = unsafe { transmute(e) };
    //             let b = Button::Keyboard(match key_event.detail {
    //                 xproto::KEY_W => Keyboard::W,
    //                 xproto::KEY_S => Keyboard::S,
    //                 xproto::KEY_A => Keyboard::A,
    //                 xproto::KEY_D => Keyboard::D,
    //                 // xproto::KEY_P => { Keyboard::P },
    //                 xproto::KEY_F1 => Keyboard::Function(1),
    //                 k @ _ => {
    //                     vxlogi!("Unknown key: {:?} presse", k);
    //                     Keyboard::W
    //                 }
    //             });
    //             let e = if a == xproto::XCB_KEY_RELEASE {
    //                 EventType::Release { button: b }
    //             } else {
    //                 EventType::Press { button: b }
    //             };
    //             self.core_app.read().unwrap().on_event(Event::new(e));
    //         }
    //         xproto::XCB_DESTROY_NOTIFY => {
    //             self.is_running = false;
    //         }
    //         xproto::XCB_CONFIGURE_NOTIFY => {
    //             let cfg_event: &xcb::xcb_configure_notify_event_t = unsafe { transmute(e) };
    //             if cfg_event.width as Real != self.window_aspects.0 ||
    //                 cfg_event.height as Real != self.window_aspects.1
    //             {
    //                 if cfg_event.width > 0 && cfg_event.height > 0 {
    //                     self.window_aspects.0 = cfg_event.width as Real;
    //                     self.window_aspects.1 = cfg_event.height as Real;
    //                     let e = EventType::WindowSize {
    //                         w: self.window_aspects.0 as f64,
    //                         h: self.window_aspects.1 as f64,
    //                     };
    //                     self.renderer.read().unwrap().on_event(Event::new(e));
    //                     self.core_app.read().unwrap().on_event(Event::new(e));
    //                 }
    //             }
    //         }
    //         c @ _ => {
    //             vxlogi!("Uncontrolled event: {:?}", c);
    //         }
    //     }
    // }
}
