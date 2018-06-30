#[macro_use]
extern crate vulkust;

use vulkust::core::application::ApplicationTrait as CoreAppTrait;
use vulkust::core::event::Event;
use vulkust::render::engine::Engine as Renderer;
use vulkust::render::scene::Game as GameScene;
use vulkust::system::os::application::Application as OsApp;

use std::sync::{Arc, RwLock};

struct MyGame {
    pub os_app: Option<Arc<RwLock<OsApp>>>,
    pub renderer: Option<Arc<RwLock<Renderer>>>,
    pub scene: Option<Arc<RwLock<GameScene>>>,
}

impl MyGame {
    pub fn new() -> Self {
        MyGame {
            os_app: None,
            renderer: None,
            scene: None,
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
        self.scene =
            Some(vxresult!(vxunwrap!(self.renderer).write()).load_scene("data/1.glb", "scene-001"));
    }

    fn on_event(&self, _e: Event) {}

    fn update(&mut self) {}

    fn terminate(&mut self) {}
}

vulkust_start!(MyGame);
