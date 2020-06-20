use super::super::core::types::Id;
use super::camera::Camera;
use super::command::Pool as CmdPool;
use super::deferred::Deferred;
use super::engine::Engine;
use super::g_buffer_filler::GBufferFiller;
use super::gapi::GraphicApiEngine;
use super::light::Light;
use super::model::Model;
use super::object::Object;
use super::shadower::Shadower;
use super::ssao::SSAO;
use super::sync::Semaphore;
use crate::core::gx3d::Gx3DReader;
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock, Weak};

mod base_deferred_shadow;
mod base_unlit;
pub mod game;
pub mod manager;
pub mod ui;

pub use self::game::Game;
pub use self::manager::Manager;
pub use self::ui::Ui;

#[repr(u8)]
#[cfg_attr(debug_mode, derive(Debug))]
pub enum TypeId {
    GAME = 1,
    UI = 2,
}

pub trait Scene: Object {
    fn add_camera(&mut self, camera: Arc<RwLock<dyn Camera>>);
    fn add_model(&mut self, model: Arc<RwLock<dyn Model>>);
    fn add_light(&mut self, light: Arc<RwLock<dyn Light>>);
    fn get_active_camera(&self) -> &Option<Weak<RwLock<dyn Camera>>>;
    fn get_models(&self) -> &BTreeMap<Id, Arc<RwLock<dyn Model>>>;
    fn get_all_models(&self) -> &BTreeMap<Id, Weak<RwLock<dyn Model>>>;
    fn update(&mut self, frame_number: usize);
    fn render_gbuffer_shadow_maps(
        &self,
        gapi_engine: &GraphicApiEngine,
        cmd_pool: &Arc<CmdPool>,
        filler: &GBufferFiller,
        shadower: &Shadower,
        kernel_index: usize,
    );
    fn update_shadow_makers(&self);
    fn render_shadow_maps(&self, shadower: &Shadower, kernel_index: usize, frame_number: usize);
    fn clean(&mut self);
    fn submit(
        &mut self,
        gapi_engine: &GraphicApiEngine,
        semaphore: &Arc<Semaphore>,
        cmd_pool: &Arc<CmdPool>,
        filler: &GBufferFiller,
        shadower: &mut Shadower,
        deferred: &Deferred,
        ssao: Option<&SSAO>,
    ) -> Arc<Semaphore>;
}

pub trait Loadable: Scene + Sized {
    fn new_with_gltf(engine: &Engine, gltf_obj: &gltf::Scene, data: &[u8]) -> Self;
    fn new_with_gx3d(engine: &Engine, reader: &mut Gx3DReader, id: Id) -> Self;
}

pub trait DefaultScene: Scene + Sized {
    fn default(engine: &Engine) -> Self;
}
