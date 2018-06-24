#[macro_use]
extern crate vulkust;

use vulkust::core::application::ApplicationTrait as CoreAppTrait;
use vulkust::core::event::Event;
use vulkust::render::engine::Engine as Renderer;
use vulkust::system::os::application::Application as OsApp;

use std::sync::{Arc, RwLock};

struct MyGame {
    pub os_app: Option<Arc<RwLock<OsApp>>>,
    pub renderer: Option<Arc<RwLock<Renderer>>>,
}

impl MyGame {
    pub fn new() -> Self {
        MyGame {
            os_app: None,
            renderer: None,
        }
    }
}

impl CoreAppTrait for MyGame {
    fn set_os_app(&mut self, os_app: Arc<RwLock<OsApp>>) {
        self.os_app = Some(os_app);
    }

    fn set_renderer(&mut self, renderer: Arc<RwLock<Renderer>>) {
        self.renderer = Some(renderer);
    }

    fn initialize(&mut self) {
        vxresult!(vxunwrap!(self.renderer).write()).load_scene("data.gltf", "scene-01");
    }

    fn on_event(&self, _e: Event) {}

    fn update(&mut self) {}

    fn terminate(&mut self) {}
}

vulkust_start!(MyGame);
