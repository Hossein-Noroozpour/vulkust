use super::super::super::core::application::ApplicationTrait as CoreAppTrait;
use super::super::super::core::event::{Button, Event, Keyboard, Mouse, Type as EventType, Window};
use super::super::super::libc::c_int;
use super::super::super::render::engine::Engine as RenderEngine;
use super::file::AASSET_MANAGER;
use super::glue::{AndroidApp, AndroidPollSource, AppCmd};
use super::looper::ALooper_pollAll;
use std::mem::transmute;
use std::ptr::null_mut;
use std::sync::{Arc, RwLock};

pub struct Application {
    pub core_app: Option<Arc<RwLock<CoreAppTrait>>>,
    pub renderer: Option<Arc<RwLock<RenderEngine>>>,
    pub and_app: *mut AndroidApp,
    pub events: Arc<RwLock<Vec<Event>>>,
}

impl Application {
    pub fn new(core_app: Arc<RwLock<CoreAppTrait>>, and_app: *mut AndroidApp) -> Self {
        unsafe {
            (*and_app).on_app_cmd = handle_cmd;
            AASSET_MANAGER = transmute((*(*and_app).activity).assetManager);
        }
        Application {
            core_app: Some(core_app),
            renderer: None,
            and_app,
            events: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn initialize(&self) {
        vxlogi!("I'm in");
        let mut events = 0 as c_int;
        let mut source = 0 as *mut AndroidPollSource;
        while unsafe { (*self.and_app).destroy_requested == 0 } {
            if unsafe { ALooper_pollAll(-1, null_mut(), &mut events, transmute(&mut source)) } >= 0
            {
                if source != null_mut() {
                    unsafe {
                        ((*source).process)(self.and_app, source);
                    }
                }
                if unsafe { (*self.and_app).window != null_mut() } {
                    return;
                }
            }
        }
        vxloge!("Unexpected flow.");
    }

    pub fn set_renderer(&mut self, renderer: Arc<RwLock<RenderEngine>>) {
        self.renderer = Some(renderer);
    }

    pub fn run(&self) {
        loop {
            let _ = self.fetch_events();
            vxresult!(vxunwrap!(self.renderer).write()).update();
        }
    }

    fn handle_cmd(&self, cmd: i32) {
        match unsafe { transmute::<i8, AppCmd>(cmd as i8) } {
            AppCmd::InitWindow => {
                vxlogi!("Window has been shown!");
            }
            AppCmd::TermWindow => {
                vxlogi!("Window has been terminated!");
            }
            c @ _ => {
                let _ = c;
                vxlogi!("event {:?} not handled.", c);
            }
        }
    }

    pub fn fetch_events(&self) -> Vec<Event> {
        let mut events = 0 as c_int;
        let mut source = 0 as *mut AndroidPollSource;
        while unsafe {
            (*self.and_app).destroy_requested == 0
                && ALooper_pollAll(0, null_mut(), &mut events, transmute(&mut source)) >= 0
        } && source != null_mut()
        {
            unsafe {
                ((*source).process)(self.and_app, source);
            }
        }
        let events = vxresult!(self.events.read()).clone();
        vxresult!(self.events.write()).clear();
        return events;
    }

    pub fn get_window_aspect_ratio(&self) -> f32 {
        1.7
    }
}

extern "C" fn handle_cmd(android_app: *mut AndroidApp, cmd: i32) {
    unsafe {
        vxresult!(vxunwrap!((*android_app).os_app).read()).handle_cmd(cmd);
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        vxloge!(
            "Error unexpected deletion of Os Application this is a \
             TODO I will decide later how to do finall termination."
        );
    }
}
