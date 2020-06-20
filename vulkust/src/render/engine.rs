use super::super::core::application::Application as CoreAppTrait;
use super::super::core::asset::Manager as AssetManager;
use super::super::core::event::Event;
use super::super::core::timing::Timing;
use super::super::system::os::application::Application as OsApp;
use super::config::Configurations;
use super::gapi::GraphicApiEngine;
use super::multithreaded::Engine as MultithreadedEngine;
use std::sync::{Arc, RwLock, Weak};

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Engine {
    myself: Option<Weak<RwLock<Engine>>>,
    gapi_engine: Arc<RwLock<GraphicApiEngine>>,
    os_app: Weak<RwLock<OsApp>>,
    core_app: Arc<RwLock<dyn CoreAppTrait>>,
    asset_manager: AssetManager,
    timing: Arc<RwLock<Timing>>,
    config: Configurations,
    multithreaded_engine: MultithreadedEngine,
}

impl Engine {
    pub fn new(core_app: Arc<RwLock<dyn CoreAppTrait>>, os_app: &Arc<RwLock<OsApp>>) -> Self {
        let core_config = &vx_result!(core_app.read()).get_config();
        let asset_manager = AssetManager::new(&core_config);
        let config = core_config.get_render().clone();
        let gapi_engine = Arc::new(RwLock::new(GraphicApiEngine::new(os_app, core_config)));
        let myself = None;
        let multithreaded_engine =
            MultithreadedEngine::new(gapi_engine.clone(), &asset_manager, &config);
        Self {
            myself,
            gapi_engine,
            os_app: Arc::downgrade(os_app),
            core_app,
            asset_manager,
            timing: Arc::new(RwLock::new(Timing::new())),
            config,
            multithreaded_engine,
        }
    }

    pub fn get_timing(&self) -> &Arc<RwLock<Timing>> {
        return &self.timing;
    }

    pub(crate) fn get_os_app(&self) -> &Weak<RwLock<OsApp>> {
        return &self.os_app;
    }

    pub(crate) fn get_gapi_engine(&self) -> &Arc<RwLock<GraphicApiEngine>> {
        return &self.gapi_engine;
    }

    pub fn set_myself(&mut self, myself: Weak<RwLock<Engine>>) {
        self.asset_manager.set_engine(&myself);
        self.myself = Some(myself);
    }

    pub fn get_config(&self) -> &Configurations {
        return &self.config;
    }

    pub fn update(&self) {
        vx_result!(self.timing.write()).update();
        self.multithreaded_engine.render();
    }

    pub fn get_asset_manager(&self) -> &AssetManager {
        return &self.asset_manager;
    }

    pub fn on_event(&self, _e: Event) {}
}

unsafe impl Send for Engine {}

unsafe impl Sync for Engine {}
