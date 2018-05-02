use super::super::core::application::ApplicationTrait as CoreAppTrait;
use super::super::core::constants::{DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH};
use super::super::core::event::{Event, Type as EventType};
use super::super::core::types::Real;
use super::super::libc;
use super::super::render::engine::Engine as RenderEngine;
use super::os::application::Application as OsApp;
use std::sync::{Arc, RwLock};

pub struct MouseInfo {
    pub x: Real,
    pub y: Real,
}

pub struct WindowInfo {
    pub width: Real,
    pub height: Real,
    pub ratio: Real,
}

pub struct Application {
    core_app: Arc<RwLock<CoreAppTrait>>,
    renderer: Arc<RwLock<RenderEngine>>,
    os_app: Arc<RwLock<OsApp>>,
    mouse_info: MouseInfo,
    window_info: WindowInfo,
}

impl Application {
    pub fn new(core_app: Arc<RwLock<CoreAppTrait>>, os_app: Arc<RwLock<OsApp>>) -> Self {
        let renderer = Arc::new(RwLock::new(RenderEngine::new(core_app.clone(), &os_app)));
        let mouse_info = MouseInfo { x: 0.0, y: 0.0 };
        let window_info = WindowInfo {
            width: DEFAULT_WINDOW_WIDTH,
            height: DEFAULT_WINDOW_HEIGHT,
            ratio: DEFAULT_WINDOW_WIDTH / DEFAULT_WINDOW_HEIGHT,
        };
        Application {
            core_app,
            renderer,
            os_app,
            mouse_info,
            window_info,
        }
    }

    pub fn initialize(&mut self, itself: Arc<RwLock<Application>>) {
        // self.os_app.initialize(itself);
        vxunimplemented!();
    }

    pub fn run(&self) {
        'main_loop: loop {
            let events = vxresult!(self.os_app.read()).fetch_events();
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
            }
        }
    }
}

// impl<CoreApp> Drop for Application<CoreApp>
// where
//     CoreApp: ApplicationTrait,
// {
//     #[cfg(not(target_os = "android"))]
//     fn drop(&mut self) {
//         logi!("Main system application got deleted.");
//         unsafe {
//             Box::from_raw(self.core_app);
//         }
//         unsafe {
//             Box::from_raw(self.render_engine);
//         }
//         unsafe {
//             Box::from_raw(self.os_app);
//         }
//     }
//     #[cfg(target_os = "android")]
//     fn drop(&mut self) {
//         logi!("Main system application got deleted.");
//         let _ = self.core_app;
//         let _ = self.render_engine;
//         let _ = self.os_app;
//     }
// }
