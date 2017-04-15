#[macro_use]
extern crate vulkust;

use vulkust::core::application::ApplicationTrait as MyAppTrait;

struct MyGame {
    x: u64,
}

impl MyAppTrait for MyGame {
    fn new() -> Self {
        MyGame { x: 0 }
    }

    fn update(&mut self) -> bool {
        self.x += 1;
        return false;
    }

    fn terminate(&mut self) {
        logi!("{}", self.x);
        self.x = 0;
    }
}

start!(MyGame);
