use super::super::core::application::ApplicationTrait as CoreAppTrait;
use super::super::core::event::Event;
use super::super::system::os::application::Application as OsApp;
pub use super::super::vulkan::engine::Engine as GraphicApiEngine;

// use super::command::buffer::Buffer as CmdBuff;
// use super::scene::Scene;
use std::sync::{Arc, RwLock};

pub struct Engine {
    gapi_engine: GraphicApiEngine,
    core_app: Arc<RwLock<CoreAppTrait>>,
}

impl Engine {
    pub fn new(core_app: Arc<RwLock<CoreAppTrait>>, os_app: &OsApp) -> Self {
        let gapi_engine = GraphicApiEngine::new(os_app);
        Engine {
            gapi_engine,
            core_app,
        }
    }

    pub fn update(&mut self) {}

    pub fn on_event(&self, _e: Event) {}
}
