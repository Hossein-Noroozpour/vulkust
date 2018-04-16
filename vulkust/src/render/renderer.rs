use super::super::core::event::Event;
use super::super::core::application::ApplicationTrait as CoreAppTrait;
use std::sync::{Arc, RwLock,};
pub struct Renderer {
    core_app: Arc<RwLock<CoreAppTrait>>
}

impl Renderer {
    pub fn new(core_app: Arc<RwLock<CoreAppTrait>>) -> Self {
        Renderer {
            core_app,
        }
    }

    pub fn update(&mut self) {}

    pub fn on_event(&self, _e: Event) {}
}