use super::super::core::application::Application as CoreAppTrait;
use super::super::core::event::Event;
use super::super::core::timing::Timing;
use super::super::system::os::application::Application as OsApp;
use super::camera::DefaultCamera;
use super::config::Configurations;
use super::gapi::GraphicApiEngine;
use super::gx3d::import as gx3d_import;
use super::model::DefaultModel;
use super::multithreaded::Engine as MultithreadedEngine;
use super::scene::{DefaultScene, Loadable as LoadableScene, Manager as SceneManager};
use std::sync::{Arc, RwLock, Weak};

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Engine {
    pub myself: Option<Weak<RwLock<Engine>>>,
    pub(crate) gapi_engine: Arc<RwLock<GraphicApiEngine>>,
    pub os_app: Weak<RwLock<OsApp>>,
    pub core_app: Arc<RwLock<CoreAppTrait>>,
    pub scene_manager: Arc<RwLock<SceneManager>>,
    pub timing: Arc<RwLock<Timing>>,
    config: Configurations,
    multithreaded_engine: MultithreadedEngine,
}

impl Engine {
    pub fn new(core_app: Arc<RwLock<CoreAppTrait>>, os_app: &Arc<RwLock<OsApp>>) -> Self {
        let config = &vxresult!(core_app.read()).get_config();
        let config = config.get_render().clone();
        let gapi_engine = GraphicApiEngine::new(os_app, &config);
        let scene_manager = Arc::new(RwLock::new(SceneManager::new()));
        gx3d_import(&scene_manager);
        let gapi_engine = Arc::new(RwLock::new(gapi_engine));
        let myself = None;
        let multithreaded_engine =
            MultithreadedEngine::new(gapi_engine.clone(), scene_manager.clone(), &config);
        Engine {
            myself,
            gapi_engine,
            os_app: Arc::downgrade(os_app),
            core_app,
            scene_manager,
            timing: Arc::new(RwLock::new(Timing::new())),
            config,
            multithreaded_engine,
        }
    }

    pub(crate) fn get_gapi_engine(&self) -> &Arc<RwLock<GraphicApiEngine>> {
        return &self.gapi_engine;
    }

    pub fn set_myself(&mut self, myself: Weak<RwLock<Engine>>) {
        vxresult!(self.scene_manager.write()).set_engine(myself.clone());
        self.myself = Some(myself);
    }

    pub fn get_config(&self) -> &Configurations {
        return &self.config;
    }

    pub fn update(&self) {
        vxresult!(self.timing.write()).update();
        self.multithreaded_engine.render();
    }

    pub fn load_gltf_scene<S>(&self, file_name: &str, scene_name: &str) -> Arc<RwLock<S>>
    where
        S: 'static + LoadableScene,
    {
        vxresult!(self.scene_manager.write()).load_gltf::<S>(file_name, scene_name)
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

    pub fn create_model<M>(&self) -> Arc<RwLock<M>>
    where
        M: 'static + DefaultModel,
    {
        let sm = vxresult!(self.scene_manager.read());
        let mut mm = vxresult!(sm.model_manager.write());
        let m = mm.create(self);
        return m;
    }

    pub fn on_event(&self, _e: Event) {}
}

unsafe impl Send for Engine {}

unsafe impl Sync for Engine {}
