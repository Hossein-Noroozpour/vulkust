#[macro_use]
extern crate vulkust;

use vulkust::core::application::ApplicationTrait as CoreApp;

struct Application {
    x: i32,
}

impl CoreApp for Application {
    fn new() -> Self {
        Application {
           x: 32,
        }
    }

    fn update(&mut self) -> bool {
        self.x += 3;
        loginfo!(self.x);
        return false;
    }

    fn terminate(&mut self) {
        self.x = 0;
        loginfo!(self.x);
    }
}

start!(Application);
