use std::os::raw::c_void;


pub type OsApplication<CoreApp> = Application<CoreApp>;

use super::super::core::application::ApplicationTrait as CoreAppTrait;
use super::super::render::engine::RenderEngine;

pub trait ApplicationTrait<CoreApp>
where
    CoreApp: 'static + CoreAppTrait,
{
    fn new(args: *const c_void) -> Self;
    fn set_core_app(&mut self, c: &'static mut CoreApp);
    fn set_rnd_eng(&mut self, r: &'static mut RenderEngine<CoreApp>);
    fn initialize(&mut self) -> bool;
    fn execute(&mut self) -> bool;
    fn get_mouse_position(&self) -> (f64, f64);
    fn get_window_ratio(&self) -> f64;
}
