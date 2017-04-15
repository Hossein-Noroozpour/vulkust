use std::os::raw::c_void;
#[cfg(target_os = "android")]
use super::android::application::Application;
#[cfg(target_os = "linux")]
use super::linux::application::Application;
#[cfg(target_os = "windows")]
use super::windows::application::Application;

pub type OsApplication<CoreApp> = Application<CoreApp>;

use super::super::core::application::ApplicationTrait as CoreAppTrait;
use super::super::render::engine::RenderEngine;

pub trait ApplicationTrait<CoreApp>
        where CoreApp: CoreAppTrait {
    fn new(args: *const c_void) -> Self;
    fn start(&mut self) -> bool;
    fn set_core_app(&mut self, c: *mut CoreApp);
    fn set_rnd_eng(&mut self, r: *mut RenderEngine<CoreApp>);
    fn execute(&mut self) -> bool;
}
