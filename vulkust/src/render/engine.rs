// use super::super::vulkan::engine::Engine as AbstractEngine;

use super::super::core::application::ApplicationTrait as CoreAppTrait;
use super::super::core::event::Event;
use super::super::system::application::OsApp;

// use super::command::buffer::Buffer as CmdBuff;
// use super::scene::Scene;
use std::sync::{Arc, RwLock,};

pub struct Engine {
    core_app: Arc<RwLock<CoreAppTrait>>
}

impl Engine {
    pub fn new(core_app: Arc<RwLock<CoreAppTrait>>) -> Self {
        Engine {
            core_app,
        }
    }

    pub fn update(&mut self) {}

    pub fn on_event(&self, _e: Event) {}
}