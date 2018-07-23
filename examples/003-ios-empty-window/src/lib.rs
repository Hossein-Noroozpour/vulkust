#[macro_use]
extern crate vulkust;

use vulkust::core::application::Application as CoreAppTrait;
use vulkust::core::event::Event;

use std::sync::{Arc, RwLock};

pub struct MyGame {}

impl MyGame {
    pub fn new() -> Self {
        MyGame {}
    }
}

impl CoreAppTrait for MyGame {
    fn on_event(&self, _e: Event) {}

    fn update(&mut self) {}

    fn terminate(&mut self) {}
}

vulkust_start!(MyGame);
