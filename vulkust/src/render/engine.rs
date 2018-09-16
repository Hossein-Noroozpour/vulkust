use super::super::core::application::Application as CoreAppTrait;
use super::super::core::event::Event;
use super::super::system::os::application::Application as OsApp;
use super::camera::DefaultCamera;
use super::gx3d::import as gx3d_import;
use super::model::DefaultModel;
use super::scene::{DefaultScene, Loadable as LoadableScene, Manager as SceneManager};
use std::sync::{Arc, RwLock, Weak};
use std::time::{Duration, Instant};
// use super::command::buffer::Buffer as CmdBuff;

pub use super::super::vulkan::engine::Engine as GraphicApiEngine;

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Timing {
    pub start_of_previous_frame: Instant,
    pub start_of_current_frame: Instant,
    pub length_of_previous_frame: Duration,
}

impl Timing {
    fn new() -> Self {
        let start_of_previous_frame = Instant::now();
        let start_of_current_frame = Instant::now();
        let length_of_previous_frame =
            start_of_current_frame.duration_since(start_of_previous_frame);
        Timing {
            start_of_previous_frame,
            start_of_current_frame,
            length_of_previous_frame,
        }
    }

    pub fn update(&mut self) {
        self.start_of_previous_frame = self.start_of_current_frame;
        self.start_of_current_frame = Instant::now();
        self.length_of_previous_frame = self
            .start_of_current_frame
            .duration_since(self.start_of_previous_frame);
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Engine {
    pub myself: Option<Weak<RwLock<Engine>>>,
    pub gapi_engine: Arc<RwLock<GraphicApiEngine>>,
    pub os_app: Weak<RwLock<OsApp>>,
    pub core_app: Arc<RwLock<CoreAppTrait>>,
    pub scene_manager: Arc<RwLock<SceneManager>>,
    pub timing: Arc<RwLock<Timing>>,
}

impl Engine {
    pub fn new(core_app: Arc<RwLock<CoreAppTrait>>, os_app: &Arc<RwLock<OsApp>>) -> Self {
        let config = &vxresult!(core_app.read()).get_config();
        let gapi_engine = Arc::new(RwLock::new(GraphicApiEngine::new(os_app, &config.render)));
        let scene_manager = Arc::new(RwLock::new(SceneManager::new()));
        gx3d_import(&scene_manager);
        let myself = None;
        Engine {
            myself,
            gapi_engine,
            os_app: Arc::downgrade(os_app),
            core_app,
            scene_manager,
            timing: Arc::new(RwLock::new(Timing::new())),
        }
    }

    pub fn set_myself(&mut self, myself: Weak<RwLock<Engine>>) {
        self.myself = Some(myself.clone());
        vxresult!(self.scene_manager.write()).set_engine(myself);
    }

    pub fn update(&self) {
        vxresult!(self.gapi_engine.write()).start_recording();
        vxresult!(self.scene_manager.read()).render();
        vxresult!(self.gapi_engine.write()).end_recording();
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
