#[macro_use]
extern crate vulkust;

use std::mem::transmute;
use self::vulkust::core::application::ApplicationTrait as MyAppTrait;
use self::vulkust::core::event::{Event, Mouse, Button};
use self::vulkust::math::vector::Vec3;
use self::vulkust::system::os::OsApplication;
use self::vulkust::render::engine::{RenderEngine, EngineTrait};

struct MyGame {
    os_app: &'static mut OsApplication<MyGame>,
    rnd_eng: &'static mut RenderEngine<MyGame>,
    middle_mouse_down: bool,
}

impl MyAppTrait for MyGame {
    fn new() -> Self {
        MyGame {
            os_app: unsafe { transmute(0usize) },
            rnd_eng: unsafe { transmute(0usize) },
            middle_mouse_down: false,
        }
    }

    fn initialize(
        &mut self,
        o: &'static mut OsApplication<MyGame>,
        r: &'static mut RenderEngine<MyGame>) -> bool {
        self.os_app = o;
        self.rnd_eng = r;
        return true;
    }

    fn on_event(&mut self, e: Event) {
        let mut camera =
            self.rnd_eng.get_mut_basic().get_mut_current_scene().get_mut_current_camera();
        match e {
            Event::MouseMove {delta_x, delta_y} => {
                if self.middle_mouse_down {
                    camera.set_rotation_speed(delta_x as f32);
                    camera.rotate(&Vec3 {x: 0.0, y: 1.0, z: 0.0});
                    camera.set_rotation_speed(delta_y as f32);
                    camera.rotate_local_x();
                }
            },
            Event::Press {button} => {
                match button {
                    Button::Mouse(m) => {
                        match m {
                            Mouse::Middle => {
                                self.middle_mouse_down = true;
                            },
                            _ => {},
                        }
                    },
                    _ => {},
                }
            },
            Event::Release {button} => {
                match button {
                    Button::Mouse(m) => {
                        match m {
                            Mouse::Middle => {
                                self.middle_mouse_down = false;
                            },
                            _ => {},
                        }
                    },
                    _ => {},
                }
            },
            _ => {},
        }
    }

    fn update(&mut self) -> bool {
        return false;
    }

    fn terminate(&mut self) {
    }
}

start!(MyGame);
