#[macro_use]
extern crate vulkust;

use vulkust::core::application::Application as CoreAppTrait;
use vulkust::core::event::Event;
use vulkust::render::camera::{DefaultCamera, Orthographic};
use vulkust::render::engine::Engine as Renderer;
use vulkust::render::scene::{Scene, Ui as UiScene};
use vulkust::render::widget::Label;
use vulkust::system::os::application::Application as OsApp;

use std::sync::{Arc, RwLock};

///     In this example you have to place your data.gx3d file in data directory of your project (in
/// android assets/data/ and in ios Resources/data/). Then if data.gx3d was presented render engine
/// is gonna import its references and then you can load your scene by id (in here we load the first
/// scene). Keep in mind that, you can not have several gx3d file and its name must be data.gx3d

struct MyGame {
    pub os_app: Option<Arc<RwLock<OsApp>>>,
    pub renderer: Option<Arc<RwLock<Renderer>>>,
    pub ui_scene: Option<Arc<RwLock<UiScene>>>,
}

impl MyGame {
    pub fn new() -> Self {
        MyGame {
            os_app: None,
            renderer: None,
            ui_scene: None,
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

    fn initialize(&mut self) {}

    fn on_event(&self, _e: Event) {}

    fn update(&mut self) {}

    fn terminate(&mut self) {}
}

vulkust_start!(MyGame);
