use super::xcb;
use super::super::super::core::application::ApplicationTrait;
use super::super::super::render::engine::RenderEngine;
use super::super::os::{OsApplication, ApplicationTrait as OsApplicationTrait};

use std::ptr::{
    null,
    null_mut,
};

pub struct Application <CoreApp>
        where CoreApp: ApplicationTrait {
	connection: *mut xcb::xcb_connection_t,
    screen: *mut xcb::xcb_screen_t,
    window: *mut xcb::xcb_window_t,
    atom_wm_delete_window: *mut xcb::xcb_intern_atom_reply_t,
   	core_app: *mut CoreApp,
    render_engine: *mut RenderEngine<CoreApp>,
}

impl<CoreApp> OsApplicationTrait <CoreApp> for Application<CoreApp>
        where CoreApp: ApplicationTrait {
	fn new() -> Self {
		Application {
            connection: null_mut(),
            screen:  null_mut(),
            window: null_mut(),
            atom_wm_delete_window: null_mut(),
           	core_app: null_mut(),
            render_engine: null_mut(),
		}
	}
    fn start(&mut self) -> bool {
        return true;
    }
    fn set_core_app(&mut self, c: *mut CoreApp) {
        self.core_app = c;
    }
    fn set_rnd_eng(&mut self, r: *mut RenderEngine<CoreApp>) {
        self.render_engine = r;
    }
    fn execute() -> bool {
        return true;
    }
}
