use super::super::render::engine::Engine as RenderEngine;
use super::super::system::os::application::Application as OsApp;
use super::event::Event;
use std::sync::{Arc, RwLock};

pub trait ApplicationTrait {
    fn set_os_app(&mut self, _app: Arc<RwLock<OsApp>>) {}
    fn set_renderer(&mut self, _renderer: Arc<RwLock<RenderEngine>>) {}
    fn on_event(&self, _e: Event) {}
    fn update(&mut self) {}
    fn terminate(&mut self) {}
}
