#[macro_use]
extern crate vulkust;

use std::mem::transmute;
use self::vulkust::core::application::ApplicationTrait as MyAppTrait;
use self::vulkust::core::event::Event;
use self::vulkust::system::os::OsApplication;
use self::vulkust::render::engine::{RenderEngine, EngineTrait};

struct MyGame {
    os_app: &'static mut OsApplication<MyGame>,
    rnd_eng: &'static mut RenderEngine<MyGame>,
}

impl MyAppTrait for MyGame {
    fn new() -> Self {
        MyGame {
            os_app: unsafe { transmute(0usize) },
            rnd_eng: unsafe { transmute(0usize) },
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
                camera.rotate_local_y();
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
