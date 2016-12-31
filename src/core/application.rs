//use super::super::system::application::Application as SysApp;

pub struct BasicApplication {
}

pub trait Application {
    fn main(&mut self);
}

impl BasicApplication {
    pub fn new() -> Self {
        BasicApplication {
        }
    }
}

impl Application for BasicApplication {
    fn main(&mut self) {
        loop {
            logdbg!("main loop");
        }
    }
}