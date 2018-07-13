use super::super::core::application::ApplicationTrait as CoreAppTrait;
use super::super::core::event::Event;
use super::super::system::os::application::Application as OsApp;
use super::camera::DefaultCamera;
use super::config::Configurations as Config;
use super::mesh::DefaultMesh;
use super::scene::{DefaultScene, Loadable as LoadableScene, Manager as SceneManager};
use std::sync::{Arc, RwLock, Weak};
// use super::command::buffer::Buffer as CmdBuff;

pub use super::super::vulkan::engine::Engine as GraphicApiEngine;

pub struct Engine {
    pub config: Arc<RwLock<Config>>,
    pub gapi_engine: Arc<RwLock<GraphicApiEngine>>,
    pub os_app: Weak<RwLock<OsApp>>,
    pub core_app: Arc<RwLock<CoreAppTrait>>,
    pub scene_manager: Arc<RwLock<SceneManager>>,
}

impl Engine {
    pub fn new(core_app: Arc<RwLock<CoreAppTrait>>, os_app: &Arc<RwLock<OsApp>>) -> Self {
        let config = Arc::new(RwLock::new(Config {
            // todo It must be filled with a file
            number_cascaded_shadows: 6,
        }));
        let gapi_engine = Arc::new(RwLock::new(GraphicApiEngine::new(os_app)));
        let scene_manager = Arc::new(RwLock::new(SceneManager::new(&gapi_engine)));
        Engine {
            config,
            gapi_engine,
            os_app: Arc::downgrade(os_app),
            core_app,
            scene_manager,
        }
    }

    pub fn update(&self) {
        vxresult!(self.gapi_engine.write()).start_recording();
        vxresult!(self.scene_manager.read()).render();
        vxresult!(self.gapi_engine.write()).end_recording();
    }

    pub fn load_scene<S>(&self, file_name: &str, scene_name: &str) -> Arc<RwLock<S>>
    where
        S: 'static + LoadableScene,
    {
        vxresult!(self.scene_manager.write()).load::<S>(file_name, scene_name)
    }

    pub fn create_scene<S>(&self) -> Arc<RwLock<S>>
    where
        S: 'static + DefaultScene,
    {
        vxresult!(self.scene_manager.write()).create()
    }

    pub fn create_camera<C>(&self) -> Arc<RwLock<C>>
    where
        C: 'static + DefaultCamera,
    {
        vxresult!(self.scene_manager.read()).create_camera()
    }

    pub fn create_mesh<M>(&self) -> Arc<RwLock<M>>
    where
        M: 'static + DefaultMesh,
    {
        vxresult!(self.scene_manager.read()).create_mesh()
    }

    pub fn on_event(&self, _e: Event) {}
}
