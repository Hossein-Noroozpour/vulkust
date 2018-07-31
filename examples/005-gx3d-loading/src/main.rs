#[macro_use]
extern crate vulkust;

use vulkust::core::application::Application as CoreAppTrait;
use vulkust::core::event::Event;
use vulkust::core::types::Id;
use vulkust::render::camera::{DefaultCamera, Orthographic};
use vulkust::render::engine::Engine as Renderer;
use vulkust::render::scene::{Scene, Ui as UiScene};
use vulkust::render::widget::Label;
use vulkust::system::os::application::Application as OsApp;

use std::sync::{Arc, RwLock};

mod data_gx3d;

///     In this example you have to place your data.gx3d file in data directory of your project (in
/// android assets/data/ and in ios Resources/data/). Then if data.gx3d was presented render engine
/// is gonna import its references and then you can load your scene by id (in here we load the first
/// scene). Keep in mind that, you can not have several gx3d file and its name must be data.gx3d

#[cfg_attr(debug_assertions, derive(Debug))]
struct MyGame {
    pub os_app: Option<Arc<RwLock<OsApp>>>,
    pub renderer: Option<Arc<RwLock<Renderer>>>,
    pub scene: Option<Arc<RwLock<Scene>>>,
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
        let renderer = vxresult!(vxunwrap!(&self.renderer).read());
        let scene_manager = vxresult!(renderer.scene_manager.read());
        self.scene = Some(scene_manager.load_gx3d(data_gx3d::Scene::SCENE_GAME_SPLASH as Id));
    }

    fn on_event(&self, _e: Event) {}

    fn update(&mut self) {}

    fn terminate(&mut self) {}
}

vulkust_start!(MyGame);
