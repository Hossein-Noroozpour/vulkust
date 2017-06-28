extern crate libc;

use super::xcb;
use super::xproto;
use super::super::super::core::application::ApplicationTrait;
use super::super::super::core::asset::manager::Manager as AssetManager;
use super::super::super::render::engine::{RenderEngine, EngineTrait as RenderEngineTrait};
use super::super::os::ApplicationTrait as OsApplicationTrait;
use super::super::file::File;

use std::ptr::null_mut;
use std::os::raw::{c_int, c_void};
use std::mem::transmute;
use std::ffi::CString;

pub struct Application<CoreApp>
where
    CoreApp: ApplicationTrait,
{
    pub asset_manager: AssetManager,
    pub connection: *mut xcb::xcb_connection_t,
    pub screen: *mut xcb::xcb_screen_t,
    pub window: xcb::xcb_window_t,
    pub atom_wm_delete_window: *mut xcb::xcb_intern_atom_reply_t,
    pub core_app: *mut CoreApp,
    pub render_engine: *mut RenderEngine<CoreApp>,
    pub is_running: bool,
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
            core_app: null_mut(),
            render_engine: null_mut(),
            is_running: true,
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
        let value_mask = xcb::xcb_cw_t::XCB_CW_BACK_PIXEL as u32 |
            xcb::xcb_cw_t::XCB_CW_EVENT_MASK as u32;
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
    fn set_core_app(&mut self, c: *mut CoreApp) {
        self.core_app = c;
    }
    fn set_rnd_eng(&mut self, r: *mut RenderEngine<CoreApp>) {
        self.render_engine = r;
    }

    fn initialize(&mut self) -> bool {
        self.asset_manager.initialize();
        true
    }

    fn execute(&mut self) -> bool {
        unsafe {
            xcb::xcb_flush(self.connection);
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
            unsafe {
                (*(self.core_app)).update();
            }
            unsafe {
                (*(self.render_engine)).update();
            }
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
                        ._bindgen_data_
                          [0] == (*self.atom_wm_delete_window).atom))
            {
                self.is_running = false;
            }
        }
    }
}
