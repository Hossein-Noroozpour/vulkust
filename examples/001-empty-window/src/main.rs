#[macro_use]
extern crate vulkust;

use vulkust::core::application::ApplicationTrait as CoreAppTrait;
use vulkust::system::application::Application as SysApp;
use vulkust::render::renderer::Renderer;
use vulkust::core::event::Event;

use std::sync::{Arc, RwLock,};

struct MyGame {
    x: u64,
}

impl MyGame {
    pub fn new() -> Self {
        MyGame {
            x: 0,
        }
    }
}

impl CoreAppTrait for MyGame {

    fn set_system_application(&mut self, _app: Arc<RwLock<SysApp>>) {}

    fn set_renderer(&mut self, _renderer: Arc<RwLock<Renderer>>) {}

    fn on_event(&self, _e: Event) {}

    fn update(&mut self) {}

    fn terminate(&mut self) {}
}

vulkust_start!(MyGame);
