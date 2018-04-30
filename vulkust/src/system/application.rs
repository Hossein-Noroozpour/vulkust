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
    os_app: OsApp,
    mouse_info: MouseInfo,
    window_info: WindowInfo,
}

impl Application {
    #[cfg(not(target_os = "android"))]
    pub fn new(core_app: Arc<RwLock<CoreAppTrait>>) -> Self {
        let os_app = OsApp::new();
        return Application::set(core_app, os_app);
    }

    #[cfg(target_os = "android")]
    pub fn new(
        core_app: Arc<RwLock<CoreAppTrait>>,
        activity: *mut super::os::activity::ANativeActivity,
        saved_state: *mut libc::c_void,
        saved_state_size: libc::size_t,
    ) {
        let os_app = OsApp::new(activity, saved_state, saved_state_size);
        return Application::set(core_app, os_app);
    }

    fn set(core_app: Arc<RwLock<CoreAppTrait>>, os_app: OsApp) -> Self {
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

    #[cfg(not(target_os = "android"))]
    pub fn run(&self) {
        self.os_app.finalize();
        'main_loop: loop {
            let events = self.os_app.fetch_events();
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
