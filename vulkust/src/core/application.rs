use super::super::render::engine::RenderEngine;
use super::super::system::os::OsApplication;

pub trait ApplicationTrait: Sized {
    fn new() -> Self;
    fn initialize(&mut self, o: *mut OsApplication<Self>, r: *mut RenderEngine<Self>) -> bool {
        loginfo!("Application automatically initialized.");
        return true;
    }
    fn update(&mut self) -> bool;
    fn terminate(&mut self);
}
