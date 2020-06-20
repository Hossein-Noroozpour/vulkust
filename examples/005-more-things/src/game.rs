use vulkust::cgmath;
use vulkust::core::application::Application as CoreAppTrait;
use vulkust::core::event::{
    Button, ButtonAction, Event, Keyboard, Mouse, Move, Touch, TouchGesture, Type as EventType,
};
use vulkust::core::gesture;
use vulkust::core::types::Real;
use vulkust::render::camera::{Camera, Orthographic, Perspective};
use vulkust::render::engine::Engine as Renderer;
use vulkust::render::light::Sun;
use vulkust::render::material::Material;
use vulkust::render::model::{Base as ModelBase, Model};
use vulkust::render::object::Transferable;
use vulkust::render::scene::{Game as GameScene, Scene, Ui as UiScene};
use vulkust::render::widget::Label;
use vulkust::system::os::application::Application as OsApp;

use std::sync::{Arc, RwLock};

use rand::{thread_rng, Rng};

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct MyGame {
    os_app: Option<Arc<RwLock<OsApp>>>,
    renderer: Option<Arc<RwLock<Renderer>>>,
    scene: Option<Arc<RwLock<GameScene>>>,
    ui_scene: Option<Arc<RwLock<UiScene>>>,
    camera: Option<Arc<RwLock<dyn Camera>>>,
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
        let renderer = vx_unwrap!(&self.renderer);
        let renderer = vx_result!(renderer.read());
        let asset_manager = renderer.get_asset_manager();
        let scene: Arc<RwLock<GameScene>> =
            vx_result!(asset_manager.get_scene_manager().write()).create();
        let camera: Arc<RwLock<Perspective>> =
            vx_result!(asset_manager.get_camera_manager().write()).create();
        {
            let mut camera = vx_result!(camera.write());
            camera.set_location(&cgmath::Vector3::new(0.0, 0.0, 4.0));
        }
        self.camera = Some(camera.clone());

        let sun = vx_result!(asset_manager.get_light_manager().write()).create::<Sun>();
        {
            let mut scn = vx_result!(scene.write());
            scn.add_camera(camera);
            place_cubes(&mut *scn, &*renderer);
            scn.add_light(sun);
        }
        self.scene = Some(scene);
        let ui_scene: Arc<RwLock<UiScene>> =
            vx_result!(asset_manager.get_scene_manager().write()).create();
        let camera: Arc<RwLock<Orthographic>> =
            vx_result!(asset_manager.get_camera_manager().write()).create();
        {
            let mut camera = vx_result!(camera.write());
            camera.move_local_z(1.999);
        }
        let label: Arc<RwLock<Label>> =
            vx_result!(asset_manager.get_model_manager().write()).create();
        {
            let mut label = vx_result!(label.write());
            label.set_size(0.05, &renderer);
            label.set_text_size(50.0, &renderer);
            label.set_text_color(1.0, 0.0, 0.0, 1.0, &renderer);
            label.set_background_color(1.0, 0.0, 0.0, 0.0, &renderer);
            label.set_text("More things from Vulkust!", &renderer);
        }
        {
            let mut uiscn = vx_result!(ui_scene.write());
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
                    if vx_result!(self.keys_state.read()).lm {
                        let mut camera = vx_result!(vx_unwrap!(&self.camera).write());
                        camera.rotate_local_x(delta.1 * 1.5);
                        camera.rotate_global_z(delta.0 * 1.5);
                    }
                }
                _ => (),
            },
            EventType::Button { button, action } => match action {
                ButtonAction::Press => match button {
                    Button::Keyboard(k) => match k {
                        Keyboard::W => vx_result!(self.keys_state.write()).w = true,
                        Keyboard::A => vx_result!(self.keys_state.write()).a = true,
                        Keyboard::S => vx_result!(self.keys_state.write()).s = true,
                        Keyboard::D => vx_result!(self.keys_state.write()).d = true,
                        _ => (),
                    },
                    Button::Mouse(m) => match m {
                        Mouse::Left => vx_result!(self.keys_state.write()).lm = true,
                        _ => (),
                    },
                },
                ButtonAction::Release => match button {
                    Button::Keyboard(k) => match k {
                        Keyboard::W => vx_result!(self.keys_state.write()).w = false,
                        Keyboard::A => vx_result!(self.keys_state.write()).a = false,
                        Keyboard::S => vx_result!(self.keys_state.write()).s = false,
                        Keyboard::D => vx_result!(self.keys_state.write()).d = false,
                        _ => (),
                    },
                    Button::Mouse(m) => match m {
                        Mouse::Left => vx_result!(self.keys_state.write()).lm = false,
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
                            let mut camera = vx_result!(vx_unwrap!(&self.camera).write());
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
        let keys_state = vx_result!(self.keys_state.read());
        if keys_state.w || keys_state.a || keys_state.s || keys_state.d {
            let mut camera = vx_result!(vx_unwrap!(&self.camera).write());
            let delta = {
                let renderer = vx_result!(vx_unwrap!(&self.renderer).read());
                let n = vx_result!(renderer.get_timing().read())
                    .length_of_previous_frame
                    .as_nanos();
                (n as f64 / 1_000_000_000.0) as f32
            };
            if keys_state.w {
                camera.move_local_z(delta * -1.7);
            }
            if keys_state.s {
                camera.move_local_z(delta * 1.7);
            }
            if keys_state.a {
                camera.move_local_x(delta * -1.7);
            }
            if keys_state.d {
                camera.move_local_x(delta * 1.7);
            }
        }
    }

    fn terminate(&mut self) {}
}

fn place_cubes(scn: &mut dyn Scene, eng: &Renderer) {
    let astmgr = eng.get_asset_manager();
    const GROUND_CUBE_ASPECT: Real = 2.0;
    const GROUND_CUBE_SPACING: Real = GROUND_CUBE_ASPECT * 0.1;
    const GROUND_CUBE_ROW_COUNT: usize = 10;
    const ROW_INC: Real = GROUND_CUBE_SPACING + GROUND_CUBE_ASPECT * 2.0;
    const ROW_START: Real = ((GROUND_CUBE_ROW_COUNT - 1) as Real * GROUND_CUBE_SPACING
        + (GROUND_CUBE_ASPECT * 2.0) * GROUND_CUBE_ROW_COUNT as Real)
        * -0.5;
    let mut y = ROW_START;
    let ground_mesh = vx_result!(astmgr.get_mesh_manager().write()).create_cube(2.0);
    let cs = [
        [50, 50, 50, 255],
        [210, 210, 210, 255],
        [153, 255, 150, 255],
        [232, 220, 137, 255],
        [255, 200, 163, 255],
        [232, 121, 195, 255],
        [149, 141, 255, 255],
    ];
    let mut ground_meshes = Vec::with_capacity(cs.len());
    for c in &cs {
        let mut m = Material::default(eng);
        m.set_base_color(eng, c[0], c[1], c[2], c[3]);
        m.set_metallic_factor(0.1);
        m.set_roughness_factor(0.2);
        m.finalize_textures_change(eng);
        ground_meshes.push((ground_mesh.clone(), m));
    }
    let mut ground_mesh_index = 0;
    let mut mdlmgr = vx_result!(astmgr.get_model_manager().write());
    for _ in 0..GROUND_CUBE_ROW_COUNT {
        let mut x = ROW_START;
        for _ in 0..GROUND_CUBE_ROW_COUNT {
            let m: Arc<RwLock<dyn Model>> = mdlmgr.create::<ModelBase>();
            {
                let mut m = vx_result!(m.write());
                let (mesh, mat) = &ground_meshes[ground_mesh_index];
                m.add_mesh(mesh.clone(), mat.clone());
                m.translate(&cgmath::Vector3::new(x, y, -5.0));
            }
            scn.add_model(m);
            x += ROW_INC;
            ground_mesh_index += 1;
            ground_mesh_index &= 1;
        }
        y += ROW_INC;
        ground_mesh_index += 1;
        ground_mesh_index &= 1;
    }
    const RANGE: Real = ROW_START * 0.7;
    let mut rng = thread_rng();
    for _ in 0..50 {
        let y = rng.gen_range(RANGE, -RANGE);
        let x = rng.gen_range(RANGE, -RANGE);
        let z = rng.gen_range(0.0, 1.0);
        let s = rng.gen_range(0.25, 0.5);
        let m: Arc<RwLock<dyn Model>> = mdlmgr.create::<ModelBase>();
        {
            let mut m = vx_result!(m.write());
            let (mesh, mat) = &ground_meshes[ground_mesh_index];
            m.add_mesh(mesh.clone(), mat.clone());
            m.translate(&cgmath::Vector3::new(x, y, z));
            m.scale(s);
        }
        scn.add_model(m);
        ground_mesh_index += 1;
        ground_mesh_index %= cs.len();
    }
}
