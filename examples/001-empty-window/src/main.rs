#[macro_use]
extern crate vulkust;

struct Application {
    x: i32,
}

impl Application {
    fn new() -> Self {
        Application {
           x: 32,
        }
    }

    fn initialize(&mut self) {
        self.x += 1;
        loginfo!(self.x);
    }

    fn update(&mut self) {
        self.x += 3;
        loginfo!(self.x);
    }

    fn terminate(&mut self) {
        self.x = 0;
        loginfo!(self.x);
    }
}

start!(Application);
