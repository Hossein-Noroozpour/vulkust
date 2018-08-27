use super::super::super::core::application::Application as CoreAppTrait;
use super::super::super::core::constants::{DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH};
use super::super::super::core::event::{Button, Event, Keyboard, Mouse, Type as EventType, Window};
use super::super::super::core::event;
use super::super::super::core::types::Real;
use super::super::super::libc;
use super::super::super::render::engine::Engine as RenderEngine;
use super::xcb;
use super::xproto;
// use super::super::super::core::asset::manager::Manager as AssetManager;
// use super::super::file::File;

use std::ffi::CString;
use std::mem::transmute;
use std::os::raw::{c_int, c_uint};
use std::ptr::null_mut;
use std::sync::{Arc, RwLock};

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Application {
    pub renderer: Option<Arc<RwLock<RenderEngine>>>,
    pub core_app: Option<Arc<RwLock<CoreAppTrait>>>,
    pub connection: *mut xcb::xcb_connection_t,
    pub screen: *mut xcb::xcb_screen_t,
    pub window: xcb::xcb_window_t,
    pub atom_wm_delete_window: *mut xcb::xcb_intern_atom_reply_t,
    pub window_width: Real,
    pub window_height: Real,
    pub window_aspect_ratio: Real,
    pub current_mouse_position: Arc<RwLock<(Real, Real)>>,
}

impl Application {
    pub fn new(core_app: Arc<RwLock<CoreAppTrait>>) -> Self {
        let mut scr = 0 as c_int;
        let connection: *mut xcb::xcb_connection_t =
            unsafe { xcb::xcb_connect(null_mut(), &mut scr) };
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
        value_list[1] = xcb::xcb_event_mask_t::XCB_EVENT_MASK_KEY_RELEASE as u32
            | xcb::xcb_event_mask_t::XCB_EVENT_MASK_KEY_PRESS as u32
            | xcb::xcb_event_mask_t::XCB_EVENT_MASK_EXPOSURE as u32
            | xcb::xcb_event_mask_t::XCB_EVENT_MASK_STRUCTURE_NOTIFY as u32
            | xcb::xcb_event_mask_t::XCB_EVENT_MASK_POINTER_MOTION as u32
            | xcb::xcb_event_mask_t::XCB_EVENT_MASK_BUTTON_PRESS as u32
            | xcb::xcb_event_mask_t::XCB_EVENT_MASK_BUTTON_RELEASE as u32
            | xcb::xcb_event_mask_t::XCB_EVENT_MASK_RESIZE_REDIRECT as u32;
        let value_mask =
            xcb::xcb_cw_t::XCB_CW_BACK_PIXEL as u32 | xcb::xcb_cw_t::XCB_CW_EVENT_MASK as u32;
        let window_width = DEFAULT_WINDOW_WIDTH as u16;
        let window_height = DEFAULT_WINDOW_HEIGHT as u16;
        unsafe {
            xcb::xcb_create_window(
                connection,
                xcb::XCB_COPY_FROM_PARENT as u8,
                window,
                (*screen).root,
                0,
                0,
                window_width,
                window_height,
                0,
                xcb::xcb_window_class_t::XCB_WINDOW_CLASS_INPUT_OUTPUT as u16,
                (*screen).root_visual,
                value_mask,
                value_list.as_ptr(),
            );
        }
        let window_width = window_width as f32;
        let window_height = window_height as f32;
        let window_aspect_ratio = window_width / window_height;
        /* Magic code that will send notification when window is destroyed */
        let cs = CString::new("WM_PROTOCOLS".to_string().into_bytes()).unwrap();
        let cookie = unsafe { xcb::xcb_intern_atom(connection, 1, 12, cs.as_ptr()) };
        let reply = unsafe { xcb::xcb_intern_atom_reply(connection, cookie, null_mut()) };
        if reply == null_mut() {
            vxlogf!("Reply is null.");
        }
        let cs = CString::new("WM_DELETE_WINDOW".to_string().into_bytes()).unwrap();
        let cookie2 = unsafe { xcb::xcb_intern_atom(connection, 0, 16, cs.as_ptr()) };
        let atom_wm_delete_window: *mut xcb::xcb_intern_atom_reply_t =
            unsafe { xcb::xcb_intern_atom_reply(connection, cookie2, null_mut()) };
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
            libc::free(transmute(reply));
            xcb::xcb_map_window(connection, window);
            xcb::xcb_flush(connection);
        }
        let current_mouse_position = Arc::new(RwLock::new(get_mouse_position(connection, window, screen)));
        Application {
            renderer: None,
            core_app: Some(core_app),
            connection,
            screen,
            window,
            atom_wm_delete_window,
            window_width,
            window_height,
            window_aspect_ratio,
            current_mouse_position,
        }
    }

    pub fn initialize(_itself: &Arc<RwLock<Application>>) {}

    pub fn set_renderer(&mut self, renderer: Arc<RwLock<RenderEngine>>) {
        self.renderer = Some(renderer);
    }

    pub fn run(&self) {
        'main_loop: loop {
            {
                let renderer = vxresult!(vxunwrap!(&self.renderer).read());
                let mut timing = vxresult!(renderer.timing.write());
                timing.update();
            }
            let events = self.fetch_events();
            for e in events {
                match e.event_type {
                    EventType::Quit => {
                        // todo
                        // terminate core
                        // terminate renderer
                        // terminate audio engine
                        // terminate physic engine
                        break 'main_loop;
                    }
                    _ => (),
                }
                vxresult!(vxunwrap!(&self.core_app).read()).on_event(e);
            }
            vxresult!(vxunwrap!(&self.core_app).write()).update();
            vxresult!(vxunwrap!(&self.renderer).read()).update();
        }
    }

    pub fn get_mouse_position(&self) -> (Real, Real) {
        get_mouse_position(self.connection, self.window, self.screen)
    }

    pub fn get_window_ratio(&self) -> f64 {
        unsafe { (*self.screen).width_in_pixels as f64 / (*self.screen).height_in_pixels as f64 }
    }

    pub fn fetch_events(&self) -> Vec<Event> {
        let mut events = Vec::new();
        loop {
            let xcb_event = unsafe { xcb::xcb_poll_for_event(self.connection) };
            if xcb_event == null_mut() {
                break;
            }
            let e = self.translate(xcb_event);
            if let Some(e) = e {
                events.push(Event::new(e));
            }
            unsafe {
                libc::free(transmute(xcb_event));
            }
        }
        return events;
    }

    fn translate(&self, e: *mut xcb::xcb_generic_event_t) -> Option<EventType> {
        unsafe {
            if (xproto::XCB_DESTROY_NOTIFY as u8 == ((*e).response_type & 0x7f))
                || ((xproto::XCB_CLIENT_MESSAGE as u8 == ((*e).response_type & 0x7f))
                    && ((*transmute::<
                        *mut xcb::xcb_generic_event_t,
                        *mut xcb::xcb_client_message_event_t,
                    >(e))
                        .data
                        .data[0] == (*self.atom_wm_delete_window).atom))
            {
                return Some(EventType::Quit);
            }
        }
        match unsafe { (*e).response_type as c_uint & 0x7F } {
            xproto::XCB_CLIENT_MESSAGE => {
                let client_msg: &mut xcb::xcb_client_message_event_t = unsafe { transmute(e) };
                if client_msg.data.data[0] == unsafe { (*self.atom_wm_delete_window).atom } {
                    return Some(EventType::Quit);
                }
            }
            xproto::XCB_MOTION_NOTIFY => {
                let pos = self.get_mouse_position();
                let pre = *vxresult!(self.current_mouse_position.read());
                *vxresult!(self.current_mouse_position.write()) = pos;
                return Some(EventType::Move(event::Move::Mouse {
                    previous: pre,
                    current: pos,
                    delta: (pos.0 - pre.0, pos.1 - pre.1),
                }));
            }
            xproto::XCB_BUTTON_PRESS => {
                let press: &mut xcb::xcb_button_press_event_t = unsafe { transmute(e) };
                let m: xcb::xcb_button_index_t = unsafe { transmute(press.detail as u32) };
                let m = match m {
                    xcb::xcb_button_index_t::XCB_BUTTON_INDEX_1 => Mouse::Left,
                    xcb::xcb_button_index_t::XCB_BUTTON_INDEX_2 => Mouse::Middle,
                    xcb::xcb_button_index_t::XCB_BUTTON_INDEX_3 => Mouse::Right,
                    _ => {
                        vxlogi!("Unknown mouse button pressed.");
                        Mouse::Left
                    }
                };
                return Some(EventType::Button {
                    button: Button::Mouse(m),
                    action: event::ButtonAction::Press,
                });
            }
            xproto::XCB_BUTTON_RELEASE => {
                let release: &mut xcb::xcb_button_release_event_t = unsafe { transmute(e) };
                let m: xcb::xcb_button_index_t = unsafe { transmute(release.detail as u32) };
                let m = match m {
                    xcb::xcb_button_index_t::XCB_BUTTON_INDEX_1 => Mouse::Left,
                    xcb::xcb_button_index_t::XCB_BUTTON_INDEX_2 => Mouse::Middle,
                    xcb::xcb_button_index_t::XCB_BUTTON_INDEX_3 => Mouse::Right,
                    _ => {
                        vxloge!("Unknown mouse button pressed.");
                        Mouse::Left
                    }
                };
                return Some(EventType::Button {
                    button: Button::Mouse(m),
                    action: event::ButtonAction::Release,
                });
            }
            a @ xproto::XCB_KEY_PRESS | a @ xproto::XCB_KEY_RELEASE => {
                let key_event: &xcb::xcb_key_release_event_t = unsafe { transmute(e) };
                let b = Button::Keyboard(match key_event.detail {
                    xproto::KEY_W => Keyboard::W,
                    xproto::KEY_S => Keyboard::S,
                    xproto::KEY_A => Keyboard::A,
                    xproto::KEY_D => Keyboard::D,
                    // xproto::KEY_P => { Keyboard::P },
                    xproto::KEY_F1 => Keyboard::Function(1),
                    k @ _ => {
                        vxlogi!("Unknown key: {:?} presse", k);
                        Keyboard::W
                    }
                });
                return Some(if a == xproto::XCB_KEY_RELEASE {
                    EventType::Button { 
                        button: b,
                        action: event::ButtonAction::Release,
                    }
                } else {
                    EventType::Button { 
                        button: b,
                        action: event::ButtonAction::Press,
                    }
                });
            }
            xproto::XCB_DESTROY_NOTIFY => {
                return Some(EventType::Quit);
            }
            xproto::XCB_CONFIGURE_NOTIFY => {
                let cfg_event: &xcb::xcb_configure_notify_event_t = unsafe { transmute(e) };
                // if cfg_event.width as Real != self.window_aspects.0 ||
                //     cfg_event.height as Real != self.window_aspects.1
                // {
                if cfg_event.width > 0 && cfg_event.height > 0 {
                    return Some(EventType::Window(Window::SizeChange {
                        w: cfg_event.width as Real,
                        h: cfg_event.height as Real,
                        ratio: (cfg_event.width as Real) / (cfg_event.height as Real),
                        pre_w: 0.0,
                        pre_h: 0.0,
                        pre_ratio: 0.0,
                    }));
                }
                // }
            }
            c @ _ => {
                vxlogi!("Uncontrolled event: {:?}", c);
            }
        }
        return None;
    }

    pub fn get_window_aspect_ratio(&self) -> f32 {
        self.window_aspect_ratio
    }
}

fn get_mouse_position(
    connection: *mut xcb::xcb_connection_t, 
    window: xcb::xcb_window_t, 
    screen: *mut xcb::xcb_screen_t) -> (Real, Real) {
    unsafe {
        let coockie = xcb::xcb_query_pointer(connection, window);
        let reply: &mut xcb::xcb_query_pointer_reply_t = transmute(
            xcb::xcb_query_pointer_reply(connection, coockie, null_mut()),
        );
        let x = reply.root_x as Real / (*screen).width_in_pixels as Real;
        let y = reply.root_y as Real / (*screen).height_in_pixels as Real;
        libc::free(transmute(reply));
        (x, y)
    }
}