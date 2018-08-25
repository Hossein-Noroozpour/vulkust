use vulkust::core::application::Application as CoreAppTrait;
use vulkust::core::event::Event;
use vulkust::render::camera::Perspective;
use vulkust::render::engine::Engine as Renderer;
use vulkust::render::scene::{Scene, Ui as UiScene};
use vulkust::render::model::{Base as ModelBase, Model};
use vulkust::system::os::application::Application as OsApp;
use vulkust::render::object::Transferable;
use vulkust::render::material::Material;
use vulkust::math;

use std::sync::{Arc, RwLock};

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct MyGame {
    pub os_app: Option<Arc<RwLock<OsApp>>>,
    pub renderer: Option<Arc<RwLock<Renderer>>>,
    pub scene: Option<Arc<RwLock<UiScene>>>,
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
        let renderer = vxunwrap!(&self.renderer);
        let renderer = vxresult!(renderer.read());
        let scene: Arc<RwLock<UiScene>> = renderer.create_scene();
        let camera: Arc<RwLock<Perspective>> = renderer.create_camera();
        {
            let mut camera = vxresult!(camera.write());
            camera.set_location(&math::Vector3::new(0.0, 0.0, 3.0));
        }
        let model: Arc<RwLock<Model>> = renderer.create_model::<ModelBase>();
        // {
        //     let mut label = vxresult!(label.write());
        //     // by default label has Ubuntu-B.ttf font.
        //     // If you want custom font,
        //     //     place your ttf it in data/fonts/ directory
        //     //     and call following function.
        //     // label.set_font_with_file_name("your-font.ttf");
        //     label.set_size(0.15, &renderer);
        //     label.set_text_size(50.0, &renderer);
        //     label.set_text_color(1.0, 1.0, 1.0, 1.0, &renderer);
        //     label.set_background_color(0.0, 0.0, 0.0, 0.0, &renderer);
        //     label.set_text("Hello Vulkust!", &renderer);
        // }
        {
            let vertices = [
                -1.0, -1.0, 1.0,    0.0, 0.0, 1.0,    1.0, 0.0, 0.0, 1.0,     0.0, 0.0,
                1.0, -1.0, 1.0,     0.0, 0.0, 1.0,    1.0, 0.0, 0.0, 1.0,     0.0, 0.0,
                -1.0, 1.0, 1.0,     0.0, 0.0, 1.0,    1.0, 0.0, 0.0, 1.0,     0.0, 0.0, 
                1.0, 1.0, 1.0,      0.0, 0.0, 1.0,    1.0, 0.0, 0.0, 1.0,     0.0, 0.0,
                //----------------------------------------------------------------------------------
                -1.0, -1.0, -1.0,   0.0, 0.0, -1.0,  -1.0, 0.0, 0.0, 1.0,     0.0, 0.0,
                1.0, -1.0, -1.0,    0.0, 0.0, -1.0,  -1.0, 0.0, 0.0, 1.0,     0.0, 0.0,
                -1.0, 1.0, -1.0,    0.0, 0.0, -1.0,  -1.0, 0.0, 0.0, 1.0,     0.0, 0.0, 
                1.0, 1.0, -1.0,     0.0, 0.0, -1.0,  -1.0, 0.0, 0.0, 1.0,     0.0, 0.0,
                //----------------------------------------------------------------------------------
                1.0, -1.0, -1.0,    1.0, 0.0, 0.0,    0.0, 0.0, 1.0, 1.0,     0.0, 0.0,
                1.0, 1.0, -1.0,     1.0, 0.0, 0.0,    0.0, 0.0, 1.0, 1.0,     0.0, 0.0,
                1.0, -1.0, 1.0,     1.0, 0.0, 0.0,    0.0, 0.0, 1.0, 1.0,     0.0, 0.0, 
                1.0, 1.0, 1.0,      1.0, 0.0, 0.0,    0.0, 0.0, 1.0, 1.0,     0.0, 0.0,
                //----------------------------------------------------------------------------------
                -1.0, -1.0, -1.0,  -1.0, 0.0, 0.0,    0.0, 0.0, 1.0, 1.0,     0.0, 0.0,
                -1.0, 1.0, -1.0,   -1.0, 0.0, 0.0,    0.0, 0.0, 1.0, 1.0,     0.0, 0.0,
                -1.0, -1.0, 1.0,   -1.0, 0.0, 0.0,    0.0, 0.0, 1.0, 1.0,     0.0, 0.0, 
                -1.0, 1.0, 1.0,    -1.0, 0.0, 0.0,    0.0, 0.0, 1.0, 1.0,     0.0, 0.0,
                //----------------------------------------------------------------------------------
                -1.0, 1.0, -1.0,    0.0, 1.0, 0.0,    0.0, 0.0, 1.0, 1.0,     0.0, 0.0,
                1.0, 1.0, -1.0,     0.0, 1.0, 0.0,    0.0, 0.0, 1.0, 1.0,     0.0, 0.0,
                -1.0, 1.0, 1.0,     0.0, 1.0, 0.0,    0.0, 0.0, 1.0, 1.0,     0.0, 0.0, 
                1.0, 1.0, 1.0,      0.0, 1.0, 0.0,    0.0, 0.0, 1.0, 1.0,     0.0, 0.0,
                //----------------------------------------------------------------------------------
                -1.0, -1.0, -1.0,   0.0, -1.0, 0.0,    0.0, 0.0, 1.0, 1.0,     0.0, 0.0,
                1.0, -1.0, -1.0,    0.0, -1.0, 0.0,    0.0, 0.0, 1.0, 1.0,     0.0, 0.0,
                -1.0, -1.0, 1.0,    0.0, -1.0, 0.0,    0.0, 0.0, 1.0, 1.0,     0.0, 0.0, 
                1.0, -1.0, 1.0,     0.0, -1.0, 0.0,    0.0, 0.0, 1.0, 1.0,     0.0, 0.0,
                //----------------------------------------------------------------------------------
            ];
            let indices = [
                0, 1, 2,      1, 3, 2,
                4, 6, 5,      5, 6, 7,
                8, 9, 10,     9, 11, 10,
                12, 14, 13,   13, 14, 15,
                16, 18, 17,   17, 18, 19,
                20, 21, 22,   21, 23, 22,
            ];
            let material = Material::default(&*renderer);
            let scnmgr = vxresult!(renderer.scene_manager.read());
            let mesh = vxresult!(scnmgr.mesh_manager.write())
                .create_with_material(material, &vertices, &indices, &*renderer);
            vxresult!(model.write()).add_mesh(mesh);
        }
        {
            let mut uiscn = vxresult!(scene.write());
            uiscn.add_camera(camera);
            uiscn.add_model(model);
        }
        self.scene = Some(scene);
    }

    fn on_event(&self, _e: Event) {}

    fn update(&mut self) {}

    fn terminate(&mut self) {}
}
