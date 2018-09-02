#![feature(duration_as_u128)]
#[macro_use]
extern crate vulkust;

use vulkust::core::application::Application as CoreAppTrait;
use vulkust::core::event::{
    Event, 
    Move, 
    Type as EventType, 
    ButtonAction, 
    Button, 
    Keyboard, 
    Mouse, 
    Touch, 
    TouchGesture
};
use vulkust::core::gesture;
use vulkust::core::types::Id;
use vulkust::render::camera::Camera;
use vulkust::render::engine::Engine as Renderer;
use vulkust::render::scene::Scene;
use vulkust::system::os::application::Application as OsApp;

use std::sync::{Arc, RwLock};

pub mod data_gx3d;

/// In this example you have to place your data.gx3d file in data directory of your project (in
/// android assets/data/ and in ios Resources/data/). Then if data.gx3d was presented render engine
/// is gonna import its references and then you can load your scene by id (in here we load the first
/// scene). Keep in mind that, you can not have several gx3d file and its name must be data.gx3d

#[cfg_attr(debug_assertions, derive(Debug))]
struct KeysState {
    pub w: bool,
    pub s: bool,
    pub a: bool,
    pub d: bool,
    pub lm: bool,
}

#[cfg_attr(debug_assertions, derive(Debug))]
struct MyGame {
    pub os_app: Option<Arc<RwLock<OsApp>>>,
    pub renderer: Option<Arc<RwLock<Renderer>>>,
    pub scene: Option<Arc<RwLock<Scene>>>,
    pub camera: Option<Arc<RwLock<Camera>>>,
    keys_state: Arc<RwLock<KeysState>>,
}

impl MyGame {
    pub fn new() -> Self {
        MyGame {
            os_app: None,
            renderer: None,
            scene: None,
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
        let renderer = vxresult!(vxunwrap!(&self.renderer).read());
        let scene_manager = vxresult!(renderer.scene_manager.read());
        let scene = scene_manager.load_gx3d(data_gx3d::Scene::SceneGameSplash as Id);
        self.camera = vxunwrap!(vxresult!(scene.read()).get_active_camera()).upgrade();
        self.scene = Some(scene);
    }

    fn on_event(&self, e: Event) {
        match e.event_type {
            EventType::Move(m) => match m {
                Move::Mouse { previous: _, current: _, delta } => {
                    if vxresult!(self.keys_state.read()).lm {
                        let mut camera = vxresult!(vxunwrap!(&self.camera).write());
                        camera.rotate_local_x(delta.1 * 1.5);
                        camera.rotate_global_z(delta.0 * 1.5);
                    }
                },
                _ => (),
            },
            EventType::Button {button, action } => match action {
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
                Touch::Gesture { start_time: _, duration: _, state, gest} => match state {
                    gesture::State::InMiddle => match gest {
                        TouchGesture::Drag { index: _, start: _, previous: _, current: _, delta } => {
                            let mut camera = vxresult!(vxunwrap!(&self.camera).write());
                            camera.rotate_local_x(delta.1 * 1.5);
                            camera.rotate_global_z(delta.0 * 1.5);
                        },
                        _ => (),
                    },
                    _ => (),
                },
                _ => (),
            }
            _ => (),
        }
    }

    fn update(&mut self) {
        let keys_state = vxresult!(self.keys_state.read());
        if keys_state.w || keys_state.a || keys_state.s || keys_state.d {
            let mut camera = vxresult!(vxunwrap!(&self.camera).write());
            let delta = {
                let renderer = vxresult!(vxunwrap!(&self.renderer).read());
                let n = vxresult!(renderer.timing.read()).length_of_previous_frame.as_nanos();
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

    fn terminate(&mut self) {
        self.camera = None;
        self.scene = None;
        self.renderer = None;
        self.os_app = None;
    }
}

vulkust_start!(MyGame);
