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

    fn initialize(&self) {
        loginfo!(self.x);
    }

    fn update(&self) {
        loginfo!(self.x);
    }

    fn terminate(&self) {
        loginfo!(self.x);
    }
}

start!(Application);
