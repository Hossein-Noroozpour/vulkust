use vulkust::core::application::Application as CoreAppTrait;
use vulkust::core::event::Event;
use vulkust::render::camera::Orthographic;
use vulkust::render::engine::Engine as Renderer;
use vulkust::render::object::Transferable;
use vulkust::render::scene::{Scene, Ui as UiScene};
use vulkust::render::widget::Label;
use vulkust::system::os::application::Application as OsApp;

use std::sync::{Arc, RwLock};

#[cfg_attr(debug_assertions, derive(Debug))]
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
        let renderer = vxunwrap!(&self.renderer);
        let renderer = vxresult!(renderer.read());
        let asset_manager = renderer.get_asset_manager();
        let ui_scene: Arc<RwLock<UiScene>> =
            vxresult!(asset_manager.get_scene_manager().write()).create();
        let camera: Arc<RwLock<Orthographic>> =
            vxresult!(asset_manager.get_camera_manager().write()).create();
        {
            let mut camera = vxresult!(camera.write());
            camera.move_local_z(-2.0);
        }
        let label: Arc<RwLock<Label>> =
            vxresult!(asset_manager.get_model_manager().write()).create();
        {
            let mut label = vxresult!(label.write());
            // by default label has Ubuntu-B.ttf font.
            // If you want custom font,
            //     place your ttf it in data/fonts/ directory
            //     and call following function.
            // label.set_font_with_file_name("your-font.ttf");
            label.set_size(0.05, &renderer);
            label.set_text_size(50.0, &renderer);
            label.set_text_color(1.0, 1.0, 1.0, 1.0, &renderer);
            label.set_background_color(1.0, 1.0, 1.0, 0.0, &renderer);
            label.set_text("Hello Vulkust!", &renderer);
        }
        {
            let mut uiscn = vxresult!(ui_scene.write());
            uiscn.add_camera(camera);
            uiscn.add_model(label);
        }
        self.ui_scene = Some(ui_scene);
    }

    fn on_event(&self, _e: Event) {}

    fn update(&mut self) {}

    fn terminate(&mut self) {}
}
