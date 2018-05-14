#[macro_use]
extern crate vulkust;

use vulkust::core::application::ApplicationTrait as CoreAppTrait;
use vulkust::core::event::Event;
// use vulkust::render::engine::Engine as RenderEngine;
// use vulkust::system::application::Application as SysApp;

use std::sync::{Arc, RwLock};

struct MyGame {}

impl MyGame {
    pub fn new() -> Self {
        MyGame {}
    }
}

impl CoreAppTrait for MyGame {
    // fn set_system_application(&mut self, _app: Arc<RwLock<SysApp>>) {}

    // fn set_renderer(&mut self, _renderer: Arc<RwLock<RenderEngine>>) {}

    fn on_event(&self, _e: Event) {}

    fn update(&mut self) {}

    fn terminate(&mut self) {}
}

vulkust_start!(MyGame);