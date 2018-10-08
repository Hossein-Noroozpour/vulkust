use vulkust::core::application::Application as CoreAppTrait;
use vulkust::core::event::{
    Button, ButtonAction, Event, Keyboard, Mouse, Move, Touch, TouchGesture, Type as EventType,
};
use vulkust::core::gesture;
use vulkust::math;
use vulkust::render::camera::{Camera, Orthographic, Perspective};
use vulkust::render::engine::Engine as Renderer;
use vulkust::render::material::Material;
use vulkust::render::model::{Base as ModelBase, Model};
use vulkust::render::object::Transferable;
use vulkust::render::scene::{Game as GameScene, Scene, Ui as UiScene};
use vulkust::render::widget::Label;
use vulkust::system::os::application::Application as OsApp;

use std::sync::{Arc, RwLock};

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct MyGame {
    pub os_app: Option<Arc<RwLock<OsApp>>>,
    pub renderer: Option<Arc<RwLock<Renderer>>>,
    pub scene: Option<Arc<RwLock<GameScene>>>,
    pub ui_scene: Option<Arc<RwLock<UiScene>>>,
    pub camera: Option<Arc<RwLock<Camera>>>,
    keys_state: Arc<RwLock<KeysState>>,
}

#[cfg_attr(debug_assertions, derive(Debug))]
struct KeysState {
    pub w: bool,
    pub s: bool,
    pub a: bool,
    pub d: bool,
    pub lm: bool,
}

impl MyGame {
    pub fn new() -> Self {
        MyGame {
            os_app: None,
            renderer: None,
            scene: None,
            ui_scene: None,
            camera: None,
            keys_state: Arc::new(RwLock::new(KeysState {
                w: false,
                a: false,
                s: false,
                d: false,
                lm: false,
            })),
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
        let scene: Arc<RwLock<GameScene>> = renderer.create_scene();
        let camera: Arc<RwLock<Perspective>> = renderer.create_camera();
        {
            let mut camera = vxresult!(camera.write());
            camera.set_location(&math::Vector3::new(0.0, 0.0, 4.0));
        }
        self.camera = Some(camera.clone());
        let model: Arc<RwLock<Model>> = renderer.create_model::<ModelBase>();
        {
            let vertices = [
                -1.0, -1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, -1.0, 1.0, 0.0,
                0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, -1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0,
                0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0,
                //----------------------------------------------------------------------------------
                -1.0, -1.0, -1.0, 0.0, 0.0, -1.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, -1.0, -1.0,
                0.0, 0.0, -1.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, -1.0, 1.0, -1.0, 0.0, 0.0, -1.0,
                -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, -1.0, 0.0, 0.0, -1.0, -1.0, 0.0, 0.0, 1.0,
                0.0, 0.0,
                //----------------------------------------------------------------------------------
                1.0, -1.0, -1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, -1.0, 1.0,
                0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, -1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0,
                1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0,
                //----------------------------------------------------------------------------------
                -1.0, -1.0, -1.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, -1.0, 1.0, -1.0,
                -1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, -1.0, -1.0, 1.0, -1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 1.0, 0.0, 0.0, -1.0, 1.0, 1.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0,
                0.0,
                //----------------------------------------------------------------------------------
                -1.0, 1.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, -1.0, 0.0,
                1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, -1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0,
                //----------------------------------------------------------------------------------
                -1.0, -1.0, -1.0, 0.0, -1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, -1.0, -1.0,
                0.0, -1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, -1.0, -1.0, 1.0, 0.0, -1.0, 0.0, 0.0,
                0.0, 1.0, 1.0, 0.0, 0.0, 1.0, -1.0, 1.0, 0.0, -1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0,
                0.0,
                //----------------------------------------------------------------------------------
            ];
            let indices = [
                0, 1, 2, 1, 3, 2, 4, 6, 5, 5, 6, 7, 8, 9, 10, 9, 11, 10, 12, 14, 13, 13, 14, 15,
                16, 18, 17, 17, 18, 19, 20, 21, 22, 21, 23, 22,
            ];
            let material = Material::default(&*renderer);
            let scnmgr = vxresult!(renderer.scene_manager.read());
            let mesh = vxresult!(scnmgr.mesh_manager.write())
                .create_with_material(material, &vertices, &indices, 1.8, &*renderer);
            vxresult!(model.write()).add_mesh(mesh);
        }
        {
            let mut scn = vxresult!(scene.write());
            scn.add_camera(camera);
            scn.add_model(model);
        }
        self.scene = Some(scene);
        let ui_scene: Arc<RwLock<UiScene>> = renderer.create_scene();
        let camera: Arc<RwLock<Orthographic>> = renderer.create_camera();
        {
            let mut camera = vxresult!(camera.write());
            camera.move_local_z(-1.999);
        }
        let label: Arc<RwLock<Label>> = renderer.create_model();
        {
            let mut label = vxresult!(label.write());
            label.set_size(0.05, &renderer);
            label.set_text_size(50.0, &renderer);
            label.set_text_color(1.0, 0.0, 0.0, 1.0, &renderer);
            label.set_background_color(1.0, 0.0, 0.0, 0.0, &renderer);
            label.set_text("More things from Vulkust!", &renderer);
        }
        {
            let mut uiscn = vxresult!(ui_scene.write());
            uiscn.add_camera(camera);
            uiscn.add_model(label);
        }
        self.ui_scene = Some(ui_scene);
    }

    fn on_event(&self, e: Event) {
        match e.event_type {
            EventType::Move(m) => match m {
                Move::Mouse {
                    previous: _,
                    current: _,
                    delta,
                } => {
                    if vxresult!(self.keys_state.read()).lm {
                        let mut camera = vxresult!(vxunwrap!(&self.camera).write());
                        camera.rotate_local_x(delta.1 * 1.5);
                        camera.rotate_global_z(delta.0 * 1.5);
                    }
                }
                _ => (),
            },
            EventType::Button { button, action } => match action {
                ButtonAction::Press => match button {
                    Button::Keyboard(k) => match k {
                        Keyboard::W => vxresult!(self.keys_state.write()).w = true,
                        Keyboard::A => vxresult!(self.keys_state.write()).a = true,
                        Keyboard::S => vxresult!(self.keys_state.write()).s = true,
                        Keyboard::D => vxresult!(self.keys_state.write()).d = true,
                        _ => (),
                    },
                    Button::Mouse(m) => match m {
                        Mouse::Left => vxresult!(self.keys_state.write()).lm = true,
                        _ => (),
                    },
                },
                ButtonAction::Release => match button {
                    Button::Keyboard(k) => match k {
                        Keyboard::W => vxresult!(self.keys_state.write()).w = false,
                        Keyboard::A => vxresult!(self.keys_state.write()).a = false,
                        Keyboard::S => vxresult!(self.keys_state.write()).s = false,
                        Keyboard::D => vxresult!(self.keys_state.write()).d = false,
                        _ => (),
                    },
                    Button::Mouse(m) => match m {
                        Mouse::Left => vxresult!(self.keys_state.write()).lm = false,
                        _ => (),
                    },
                },
            },
            EventType::Touch(t) => match t {
                Touch::Gesture {
                    start_time: _,
                    duration: _,
                    state,
                    gest,
                } => match state {
                    gesture::State::InMiddle => match gest {
                        TouchGesture::Drag {
                            index: _,
                            start: _,
                            previous: _,
                            current: _,
                            delta,
                        } => {
                            let mut camera = vxresult!(vxunwrap!(&self.camera).write());
                            camera.rotate_local_x(delta.1 * 1.5);
                            camera.rotate_global_z(delta.0 * 1.5);
                        }
                        _ => (),
                    },
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        }
    }

    fn update(&mut self) {
        let keys_state = vxresult!(self.keys_state.read());
        if keys_state.w || keys_state.a || keys_state.s || keys_state.d {
            let mut camera = vxresult!(vxunwrap!(&self.camera).write());
            let delta = {
                let renderer = vxresult!(vxunwrap!(&self.renderer).read());
                let n = vxresult!(renderer.timing.read())
                    .length_of_previous_frame
                    .as_nanos();
                (n as f64 / 1_000_000_000.0) as f32
            };
            if keys_state.w {
                camera.move_local_z(delta * 0.7);
            }
            if keys_state.s {
                camera.move_local_z(delta * -0.7);
            }
            if keys_state.a {
                camera.move_local_x(delta * -0.7);
            }
            if keys_state.d {
                camera.move_local_x(delta * 0.7);
            }
        }
    }

    fn terminate(&mut self) {}
}
