use super::super::core::application::ApplicationTrait as CoreAppTrait;
use super::super::core::event::Event;
use super::super::system::os::application::Application as OsApp;
pub use super::super::vulkan::engine::Engine as GraphicApiEngine;
// use super::command::buffer::Buffer as CmdBuff;
use super::scene::{Loadable as LoadableScene, Manager as SceneManager};
use std::sync::{Arc, RwLock, Weak};

pub struct Engine {
    pub gapi_engine: Arc<RwLock<GraphicApiEngine>>,
    pub os_app: Weak<RwLock<OsApp>>,
    pub core_app: Arc<RwLock<CoreAppTrait>>,
    pub scene_manager: Arc<RwLock<SceneManager>>,
}

impl Engine {
    pub fn new(core_app: Arc<RwLock<CoreAppTrait>>, os_app: &Arc<RwLock<OsApp>>) -> Self {
        let gapi_engine = Arc::new(RwLock::new(GraphicApiEngine::new(os_app)));
        let scene_manager = Arc::new(RwLock::new(SceneManager::new(&gapi_engine)));
        Engine {
            gapi_engine,
            os_app: Arc::downgrade(os_app),
            core_app,
            scene_manager,
        }
    }

    pub fn update(&self) {
        vxresult!(self.gapi_engine.write()).update();
    }

    pub fn load_scene<S: 'static + LoadableScene>(
        &self,
        file_name: &str,
        scene_name: &str,
    ) -> Arc<RwLock<S>> {
        vxlogi!("Reached");
        vxresult!(self.scene_manager.write()).load::<S>(file_name, scene_name)
    }

    pub fn on_event(&self, _e: Event) {}
}
