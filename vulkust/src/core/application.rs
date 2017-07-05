use super::super::render::engine::RenderEngine;
use super::super::system::os::OsApplication;
use super::event::Event;

pub trait ApplicationTrait: Sized {
    fn new() -> Self;
    fn initialize(
        &mut self,
        _o: &'static mut OsApplication<Self>,
        _r: &'static mut RenderEngine<Self>) -> bool {
        return true;
    }
    fn on_event(&mut self, _e: Event) {}
    fn update(&mut self) -> bool;
    fn terminate(&mut self);
}
