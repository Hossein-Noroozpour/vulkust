#[cfg(target_os = "android")]
use android;
#[cfg(target_os = "linux")]
use linux;
use super::super::core::application::Application as CoreApp;
use super::super::render::engine::Engine as RenderEngine;

use std::ptr::{
    null,
    null_mut,
};

pub struct Application <App, RenderEng> where App: CoreApp, RenderEng: RenderEngine {
	connection: *mut xcb::xcb_connection_t,
    screen: *mut xcb::xcb_screen_t,
    window: *mut xcb::xcb_window_t,
    atom_wm_delete_window: *mut xcb::xcb_intern_atom_reply_t,
   	core_app: App,
    render_engine: RenderEng,
}

impl<App, RenderEng> Application <App, RenderEng> where App: CoreApp, RenderEng: RenderEngine {
	fn new(a: App, r: RenderEng) -> Self {
		Application {
            connection: null_mut(),
            screen:  null_mut(),
            window: null_mut(),
            atom_wm_delete_window: null_mut(),
           	core_app: a,
            render_engine: r,
		}
	}
}
