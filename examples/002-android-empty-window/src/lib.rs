#[macro_use]
extern crate vulkust;

use vulkust::core::application::Application as CoreAppTrait;
use vulkust::core::event::Event;
use vulkust::render::engine::Engine as RenderEngine;
use vulkust::system::os::application::Application as OsApp;

use std::sync::{Arc, RwLock};

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct MyGame {}

impl MyGame {
    pub fn new() -> Self {
        MyGame {}
    }
}

impl CoreAppTrait for MyGame {
    fn set_os_app(&mut self, _app: Arc<RwLock<OsApp>>) {}

    fn set_renderer(&mut self, _renderer: Arc<RwLock<RenderEngine>>) {}

    fn on_event(&self, _e: Event) {}

    fn update(&mut self) {}

    fn terminate(&mut self) {}
}

vulkust_start!(MyGame);
