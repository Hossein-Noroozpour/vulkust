use super::super::core::application::ApplicationTrait as CoreAppTrait;
use super::super::core::event::Event;
use super::super::system::os::application::Application as OsApp;
pub use super::super::vulkan::engine::Engine as GraphicApiEngine;
// use super::command::buffer::Buffer as CmdBuff;
// use super::scene::Scene;
use std::sync::{Arc, RwLock, Weak};

pub struct Engine {
    pub gapi_engine: GraphicApiEngine,
    pub os_app: Weak<RwLock<OsApp>>,
    pub core_app: Arc<RwLock<CoreAppTrait>>,
}

impl Engine {
    pub fn new(core_app: Arc<RwLock<CoreAppTrait>>, os_app: &Arc<RwLock<OsApp>>) -> Self {
        let gapi_engine = GraphicApiEngine::new(os_app);
        Engine {
            gapi_engine,
            os_app: Arc::downgrade(os_app),
            core_app,
        }
    }

    pub fn update(&mut self) {
        self.gapi_engine.update();
    }

    pub fn on_event(&self, _e: Event) {}
}
