// #[cfg(target_os = "android")]
// extern crate libc;
// use std::os::raw::c_void as std_void;
// use std::mem::transmute;
use std::sync::{Arc, RwLock};
use super::super::core::application::ApplicationTrait as CoreAppTrait;
use super::super::render::renderer::Renderer;
#[cfg(target_os = "android")]
use super::android::application::Application as OsApp;
#[cfg(target_os = "linux")]
use super::linux::application::Application as OsApp;
#[cfg(target_os = "macos")]
use super::mac::application::Application as OsApp;
#[cfg(target_os = "windows")]
use super::windows::application::Application as OsApp;
// use super::super::render::engine::{EngineTrait as RenderEngineTrait, RenderEngine};
// use super::os::{ApplicationTrait as OsApplicationTrait, OsApplication};

pub struct Application {
    // os_app: *mut OsApplication<CoreApp>,
    // render_engine: *mut RenderEngine<CoreApp>,
    pub core_app: Arc<RwLock<CoreAppTrait>>,
    pub os_app: OsApp,
}

impl Application {
    #[cfg(desktop_os)]
    pub fn new(core_app: Arc<RwLock<CoreAppTrait>>) -> Self {
        let os_app = OsApp::new(core_app.clone());
        Application {
            core_app,
            os_app,
        }
    }

    #[cfg(target_os = "android")]
    pub fn new(
        activity: *mut super::android::activity::ANativeActivity,
        saved_state: *mut libc::c_void,
        saved_state_size: libc::size_t,
    ) {
        use super::android::application::Args;
        use std::mem::transmute;
        let args = Args {
            activity: activity,
            saved_state: saved_state,
            saved_state_size: saved_state_size,
        };
        let _ = Self::set(unsafe { transmute(&args) });
    }

    // fn set(args: *const std_void) -> Self {
    //     let os_app = Box::into_raw(Box::new(OsApplication::<CoreApp>::new(args)));
    //     let render_engine = Box::into_raw(Box::new(RenderEngine::<CoreApp>::new()));
    //     let core_app = Box::into_raw(Box::new(CoreApp::new()));
    //     unsafe { (*os_app).set_core_app(transmute(core_app)) };
    //     unsafe { (*os_app).set_rnd_eng(transmute(render_engine)) };
    //     unsafe { (*render_engine).set_os_app(transmute(os_app)) };
    //     unsafe { (*render_engine).set_core_app(transmute(core_app)) };
    //     unsafe { (*os_app).initialize() };
    //     //logi!("{:?}     {:?}", os_app, render_engine);
    //     Application {
    //         os_app: os_app,
    //         render_engine: render_engine,
    //         core_app: core_app,
    //     }
    // }

    #[cfg(desktop_os)]
    pub fn run(&self) {
        self.os_app.execute();
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
