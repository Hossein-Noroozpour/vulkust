use vulkust::core::application::ApplicationTrait as CoreAppTrait;
use vulkust::core::event::Event;
use vulkust::render::camera::Orthographic;
use vulkust::render::engine::Engine as Renderer;
use vulkust::render::scene::{Scene, Ui as UiScene};
use vulkust::render::widget::Label;
use vulkust::system::os::application::Application as OsApp;

use std::sync::{Arc, RwLock};

pub struct MyGame {
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

    fn initialize(&mut self) {
        let renderer = vxunwrap!(self.renderer);
        let renderer = vxresult!(renderer.read());
        let ui_scene: Arc<RwLock<UiScene>> = renderer.create_scene();
        let camera: Arc<RwLock<Orthographic>> = renderer.create_camera();
        let label: Arc<RwLock<Label>> = renderer.create_mesh();
        {
            let mut label = vxresult!(label.write());
            // by default label has Ubuntu-B.ttf font.
            // If you want custom font,
            //     place your ttf it in data/fonts/ directory
            //     and call following function.
            // label.set_font_with_file_name("Ubuntu-R.ttf");
            label.set_text_size(50.0);
            label.set_text_color(1.0, 1.0, 1.0, 1.0);
            label.set_background_color(0.0, 0.0, 0.0, 0.0);
            label.set_text("Hello Vulkust!");
        }
        {
            let mut uiscn = vxresult!(ui_scene.write());
            uiscn.add_camera(camera);
            uiscn.add_mesh(label);
        }
        self.ui_scene = Some(ui_scene);
    }

    fn on_event(&self, _e: Event) {}

    fn update(&mut self) {}

    fn terminate(&mut self) {}
}