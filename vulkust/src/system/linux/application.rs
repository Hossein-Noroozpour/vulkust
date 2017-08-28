extern crate libc;

use std::cell::DebugCell;
use std::sync::Arc;
use super::xcb;
use super::xproto;
use super::super::super::core::application::ApplicationTrait;
use super::super::super::core::event::{Button, Event, Keyboard, Mouse};
use super::super::super::core::asset::manager::Manager as AssetManager;
use super::super::super::render::engine::{EngineTrait as RenderEngineTrait, RenderEngine};
use super::super::os::ApplicationTrait as OsApplicationTrait;
use super::super::file::File;

use std::ptr::null_mut;
use std::os::raw::{c_int, c_uint, c_void};
use std::mem::transmute;
use std::ffi::CString;

pub struct Application<CoreApp>
where
    CoreApp: 'static + ApplicationTrait,
{
    pub asset_manager: Arc<DebugCell<AssetManager>>,
    pub connection: *mut xcb::xcb_connection_t,
    pub screen: *mut xcb::xcb_screen_t,
    pub window: xcb::xcb_window_t,
    pub atom_wm_delete_window: *mut xcb::xcb_intern_atom_reply_t,
    pub core_app: &'static mut CoreApp,
    pub render_engine: &'static mut RenderEngine<CoreApp>,
    pub is_running: bool,
    pub mouse_previous_location: (f64, f64),
    pub window_w: u64,
    pub window_h: u64,
}

impl<CoreApp> OsApplicationTrait<CoreApp> for Application<CoreApp>
where
    CoreApp: ApplicationTrait,
{
    fn new(args: *const c_void) -> Self {
        let _ = args;
        let mut scr = 0 as c_int;
        let mut this = Application {
            asset_manager: AssetManager::new(File::new(&"data.gx3d".to_string())),
            connection: unsafe { xcb::xcb_connect(null_mut(), &mut scr) },
            screen: null_mut(),
            window: 0 as _,
            atom_wm_delete_window: null_mut(),
            core_app: unsafe { transmute(0usize) },
            render_engine: unsafe { transmute(0usize) },
            is_running: true,
            mouse_previous_location: (0f64, 0f64),
            window_w: 1000,
            window_h: 700,
        };
        if this.connection == null_mut() {
            logf!("Could not find a compatible Vulkan ICD!");
        }
        let setup = unsafe { xcb::xcb_get_setup(this.connection) };
        let mut iter = unsafe { xcb::xcb_setup_roots_iterator(setup) };
        for _ in 0..scr {
            unsafe {
                xcb::xcb_screen_next(&mut iter);
            }
        }
        this.screen = iter.data;
        let mut value_list = vec![0u32; 32];
        this.window = unsafe { transmute(xcb::xcb_generate_id(this.connection)) };
        let value_mask =
            xcb::xcb_cw_t::XCB_CW_BACK_PIXEL as u32 | xcb::xcb_cw_t::XCB_CW_EVENT_MASK as u32;
        value_list[0] = unsafe { (*this.screen).black_pixel };
        value_list[1] = xcb::xcb_event_mask_t::XCB_EVENT_MASK_KEY_RELEASE as u32 |
            xcb::xcb_event_mask_t::XCB_EVENT_MASK_KEY_PRESS as u32 |
            xcb::xcb_event_mask_t::XCB_EVENT_MASK_EXPOSURE as u32 |
            xcb::xcb_event_mask_t::XCB_EVENT_MASK_STRUCTURE_NOTIFY as u32 |
            xcb::xcb_event_mask_t::XCB_EVENT_MASK_POINTER_MOTION as u32 |
            xcb::xcb_event_mask_t::XCB_EVENT_MASK_BUTTON_PRESS as u32 |
            xcb::xcb_event_mask_t::XCB_EVENT_MASK_BUTTON_RELEASE as u32;

        unsafe {
            xcb::xcb_create_window(
                this.connection,
                xcb::XCB_COPY_FROM_PARENT as u8,
                this.window,
                (*this.screen).root,
                0,
                0,
                1000,
                700,
                0,
                xcb::xcb_window_class_t::XCB_WINDOW_CLASS_INPUT_OUTPUT as u16,
                (*this.screen).root_visual,
                value_mask,
                value_list.as_ptr(),
            );
        }
        /* Magic code that will send notification when window is destroyed */
        let cs = CString::new("WM_PROTOCOLS".to_string().into_bytes()).unwrap();
        let cookie = unsafe { xcb::xcb_intern_atom(this.connection, 1, 12, cs.as_ptr()) };
        let reply = unsafe { xcb::xcb_intern_atom_reply(this.connection, cookie, null_mut()) };
        let cs = CString::new("WM_DELETE_WINDOW".to_string().into_bytes()).unwrap();
        let cookie2 = unsafe { xcb::xcb_intern_atom(this.connection, 0, 16, cs.as_ptr()) };
        this.atom_wm_delete_window =
            unsafe { xcb::xcb_intern_atom_reply(this.connection, cookie2, null_mut()) };
        unsafe {
            xcb::xcb_change_property(
                this.connection,
                xproto::xcb_prop_mode_t::XCB_PROP_MODE_REPLACE as u8,
                this.window,
                (*reply).atom,
                4,
                32,
                1,
                transmute(&((*this.atom_wm_delete_window).atom)),
            );
        }
        let cs = CString::new("Gearoenix Nu-Frag Game Engine".to_string().into_bytes()).unwrap();
        unsafe {
            xcb::xcb_change_property(
                this.connection,
                xproto::xcb_prop_mode_t::XCB_PROP_MODE_REPLACE as u8,
                this.window,
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
            xcb::xcb_map_window(this.connection, this.window);
        }
        this
    }
    fn set_core_app(&mut self, c: &'static mut CoreApp) {
        self.core_app = c;
    }
    fn set_rnd_eng(&mut self, r: &'static mut RenderEngine<CoreApp>) {
        self.render_engine = r;
    }

    fn initialize(&mut self) -> bool {
        self.asset_manager.initialize();
        true
    }

    fn get_mouse_position(&self) -> (f64, f64) {
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

    fn get_window_ratio(&self) -> f64 {
        unsafe { (*self.screen).width_in_pixels as f64 / (*self.screen).height_in_pixels as f64 }
    }

    fn execute(&mut self) -> bool {
        unsafe {
            xcb::xcb_flush(self.connection);
            self.mouse_previous_location = self.get_mouse_position();
            logi!("mouse position: {:?}", self.mouse_previous_location);
        }
        while self.is_running {
            loop {
                let event = unsafe { xcb::xcb_poll_for_event(self.connection) };
                if event == null_mut() {
                    break;
                }
                self.handle_event(event);
                unsafe {
                    libc::free(transmute(event));
                }
            }
            self.core_app.update();
            self.render_engine.update();
        }
        return true;
    }
}

impl<CoreApp> Application<CoreApp>
where
    CoreApp: ApplicationTrait,
{
    fn handle_event(&mut self, e: *mut xcb::xcb_generic_event_t) {
        unsafe {
            if (xproto::XCB_DESTROY_NOTIFY as u8 == ((*e).response_type & 0x7f)) ||
                ((xproto::XCB_CLIENT_MESSAGE as u8 == ((*e).response_type & 0x7f)) &&
                    ((*transmute::<
                        *mut xcb::xcb_generic_event_t,
                        *mut xcb::xcb_client_message_event_t,
                    >(e)).data
                        .data[0] == (*self.atom_wm_delete_window).atom))
            {
                self.is_running = false;
            }
        }
        match unsafe { (*e).response_type as c_uint & 0x7F } {
            xproto::XCB_CLIENT_MESSAGE => {
                let client_msg: &mut xcb::xcb_client_message_event_t = unsafe { transmute(e) };
                if client_msg.data.data[0] == unsafe { (*self.atom_wm_delete_window).atom } {
                    self.is_running = false;
                }
            }
            xproto::XCB_MOTION_NOTIFY => {
                let pos = self.get_mouse_position();
                let e = Event::MouseMove {
                    delta_x: pos.0 - self.mouse_previous_location.0,
                    delta_y: self.mouse_previous_location.1 - pos.1,
                };
                self.mouse_previous_location = pos;
                self.core_app.on_event(e);
            }
            xproto::XCB_BUTTON_PRESS => {
                let press: &mut xcb::xcb_button_press_event_t = unsafe { transmute(e) };
                let m: xcb::xcb_button_index_t = unsafe { transmute(press.detail as u32) };
                let m = match m {
                    xcb::xcb_button_index_t::XCB_BUTTON_INDEX_1 => Mouse::Left,
                    xcb::xcb_button_index_t::XCB_BUTTON_INDEX_2 => Mouse::Middle,
                    xcb::xcb_button_index_t::XCB_BUTTON_INDEX_3 => Mouse::Right,
                    _ => {
                        loge!("Unknown mouse button pressed.");
                        Mouse::Left
                    }
                };
                let e = Event::Press {
                    button: Button::Mouse(m),
                };
                self.core_app.on_event(e);
            }
            xproto::XCB_BUTTON_RELEASE => {
                let release: &mut xcb::xcb_button_release_event_t = unsafe { transmute(e) };
                let m: xcb::xcb_button_index_t = unsafe { transmute(release.detail as u32) };
                let m = match m {
                    xcb::xcb_button_index_t::XCB_BUTTON_INDEX_1 => Mouse::Left,
                    xcb::xcb_button_index_t::XCB_BUTTON_INDEX_2 => Mouse::Middle,
                    xcb::xcb_button_index_t::XCB_BUTTON_INDEX_3 => Mouse::Right,
                    _ => {
                        loge!("Unknown mouse button pressed.");
                        Mouse::Left
                    }
                };
                let e = Event::Release {
                    button: Button::Mouse(m),
                };
                self.core_app.on_event(e);
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
                        logi!("Unknown key: {:?} presse", k);
                        Keyboard::W
                    }
                });
                let e = if a == xproto::XCB_KEY_RELEASE {
                    Event::Release { button: b }
                } else {
                    Event::Press { button: b }
                };
                self.core_app.on_event(e);
            }
            xproto::XCB_DESTROY_NOTIFY => {
                self.is_running = false;
            }
            xproto::XCB_CONFIGURE_NOTIFY => {
                let cfg_event: &xcb::xcb_configure_notify_event_t = unsafe { transmute(e) };
                if cfg_event.width as u64 != self.window_w ||
                    cfg_event.height as u64 != self.window_h
                {
                    if cfg_event.width > 0 && cfg_event.height > 0 {
                        self.window_w = cfg_event.width as u64;
                        self.window_h = cfg_event.height as u64;
                        let e = Event::WindowSize {
                            w: self.window_w as f64,
                            h: self.window_h as f64,
                        };
                        self.render_engine.on_event(e);
                        self.core_app.on_event(e);
                    }
                }
            }
            c @ _ => {
                logi!("Uncontrolled event: {:?}", c);
            }
        }
    }
}
