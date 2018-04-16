use super::super::render::renderer::Renderer;
use super::super::system::application::Application as SysApp;
use super::event::Event;
use std::sync::{Arc, RwLock,};
pub trait ApplicationTrait {
    fn set_system_application(&mut self, app: Arc<RwLock<SysApp>>);
    fn set_renderer(&mut self, renderer: Arc<RwLock<Renderer>>);
    fn on_event(&self, e: Event);
    fn update(&mut self);
    fn terminate(&mut self);
}
