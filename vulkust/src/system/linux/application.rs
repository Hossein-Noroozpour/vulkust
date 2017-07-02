extern crate libc;

use super::xcb;
use super::xproto;
use super::super::super::core::application::ApplicationTrait;
use super::super::super::core::event::Event;
use super::super::super::core::asset::manager::Manager as AssetManager;
use super::super::super::render::engine::{RenderEngine, EngineTrait as RenderEngineTrait};
use super::super::os::ApplicationTrait as OsApplicationTrait;
use super::super::file::File;

use std::ptr::null_mut;
use std::os::raw::{c_int, c_void, c_uint};
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
    pub mouse_previous_location: (f64, f64),
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
            mouse_previous_location: (0f64, 0f64),
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
                    >(e)).data.data[0] == (*self.atom_wm_delete_window).atom))
            {
                self.is_running = false;
            }
        }
        match unsafe { (*e).response_type as c_uint & 0x7F } {
            xproto::XCB_CLIENT_MESSAGE => {
                let client_msg: &mut xcb::xcb_client_message_event_t = unsafe {transmute(e)};
                if client_msg.data.data[0] == unsafe { (*self.atom_wm_delete_window).atom } {
                    self.is_running = false;
                }
            },
            xproto::XCB_MOTION_NOTIFY => {
        		let motion: &mut xcb::xcb_motion_notify_event_t = unsafe { transmute(e) };
                let loc = self.mouse_previous_location;
                let event = Event::MouseMove{
                    delta_x: motion.event_x as f64 - loc.0,
                    delta_y: loc.1 - motion.event_y as f64,
                };
                self.mouse_previous_location = (motion.event_x as f64, motion.event_y as f64);
                unsafe { (*self.core_app).on_event(event); }
        	// 	if (mouseButtons.left)
        	// 	{
        	// 		rotation.x += (mousePos.y - (float)motion->event_y) * 1.25f;
        	// 		rotation.y -= (mousePos.x - (float)motion->event_x) * 1.25f;
        	// 		camera.rotate(glm::vec3((mousePos.y - (float)motion->event_y) * camera.rotationSpeed, -(mousePos.x - (float)motion->event_x) * camera.rotationSpeed, 0.0f));
        	// 		viewUpdated = true;
        	// 	}
        	// 	if (mouseButtons.right)
        	// 	{
        	// 		zoom += (mousePos.y - (float)motion->event_y) * .005f;
        	// 		camera.translate(glm::vec3(-0.0f, 0.0f, (mousePos.y - (float)motion->event_y) * .005f * zoomSpeed));
        	// 		viewUpdated = true;
        	// 	}
        	// 	if (mouseButtons.middle)
        	// 	{
        	// 		cameraPos.x -= (mousePos.x - (float)motion->event_x) * 0.01f;
        	// 		cameraPos.y -= (mousePos.y - (float)motion->event_y) * 0.01f;
        	// 		camera.translate(glm::vec3(-(mousePos.x - (float)(float)motion->event_x) * 0.01f, -(mousePos.y - (float)motion->event_y) * 0.01f, 0.0f));
        	// 		viewUpdated = true;
        	// 		mousePos.x = (float)motion->event_x;
        	// 		mousePos.y = (float)motion->event_y;
        	// 	}
        	// 	mousePos = glm::vec2((float)motion->event_x, (float)motion->event_y);
            },
        	// break;
        	// case XCB_BUTTON_PRESS:
        	// {
        	// 	xcb_button_press_event_t *press = (xcb_button_press_event_t *)event;
        	// 	if (press->detail == XCB_BUTTON_INDEX_1)
        	// 		mouseButtons.left = true;
        	// 	if (press->detail == XCB_BUTTON_INDEX_2)
        	// 		mouseButtons.middle = true;
        	// 	if (press->detail == XCB_BUTTON_INDEX_3)
        	// 		mouseButtons.right = true;
        	// }
        	// break;
        	// case XCB_BUTTON_RELEASE:
        	// {
        	// 	xcb_button_press_event_t *press = (xcb_button_press_event_t *)event;
        	// 	if (press->detail == XCB_BUTTON_INDEX_1)
        	// 		mouseButtons.left = false;
        	// 	if (press->detail == XCB_BUTTON_INDEX_2)
        	// 		mouseButtons.middle = false;
        	// 	if (press->detail == XCB_BUTTON_INDEX_3)
        	// 		mouseButtons.right = false;
        	// }
        	// break;
        	// case XCB_KEY_PRESS:
        	// {
        	// 	const xcb_key_release_event_t *keyEvent = (const xcb_key_release_event_t *)event;
        	// 	switch (keyEvent->detail)
        	// 	{
        	// 		case KEY_W:
        	// 			camera.keys.up = true;
        	// 			break;
        	// 		case KEY_S:
        	// 			camera.keys.down = true;
        	// 			break;
        	// 		case KEY_A:
        	// 			camera.keys.left = true;
        	// 			break;
        	// 		case KEY_D:
        	// 			camera.keys.right = true;
        	// 			break;
        	// 		case KEY_P:
        	// 			paused = !paused;
        	// 			break;
        	// 		case KEY_F1:
        	// 			if (enableTextOverlay)
        	// 			{
        	// 				textOverlay->visible = !textOverlay->visible;
        	// 			}
        	// 			break;
        	// 	}
        	// }
        	// break;
        	// case XCB_KEY_RELEASE:
        	// {
        	// 	const xcb_key_release_event_t *keyEvent = (const xcb_key_release_event_t *)event;
        	// 	switch (keyEvent->detail)
        	// 	{
        	// 		case KEY_W:
        	// 			camera.keys.up = false;
        	// 			break;
        	// 		case KEY_S:
        	// 			camera.keys.down = false;
        	// 			break;
        	// 		case KEY_A:
        	// 			camera.keys.left = false;
        	// 			break;
        	// 		case KEY_D:
        	// 			camera.keys.right = false;
        	// 			break;
        	// 		case KEY_ESCAPE:
        	// 			quit = true;
        	// 			break;
        	// 	}
        	// 	keyPressed(keyEvent->detail);
        	// }
        	// break;
        	// case XCB_DESTROY_NOTIFY:
        	// 	quit = true;
        	// 	break;
        	// case XCB_CONFIGURE_NOTIFY:
        	// {
        	// 	const xcb_configure_notify_event_t *cfgEvent = (const xcb_configure_notify_event_t *)event;
        	// 	if ((prepared) && ((cfgEvent->width != width) || (cfgEvent->height != height)))
        	// 	{
        	// 			destWidth = cfgEvent->width;
        	// 			destHeight = cfgEvent->height;
        	// 			if ((destWidth > 0) && (destHeight > 0))
        	// 			{
        	// 				windowResize();
        	// 			}
        	// 	}
        	// }
            c @ _ => {
                logi!("Uncontrolled event: {:?}", c);
            },
        }
    }
}
