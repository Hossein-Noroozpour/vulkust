use super::super::core::gx3d::Gx3DReader;
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

use gltf;

#[repr(u8)]
#[cfg_attr(debug_mode, derive(Debug))]
pub enum TypeId {
    GAME = 1,
    UI = 2,
}

pub trait Scene: Object {
    fn add_camera(&mut self, Arc<RwLock<Camera>>);
    fn add_model(&mut self, Arc<RwLock<Model>>);
    fn add_light(&mut self, Arc<RwLock<Light>>);
    fn get_active_camera(&self) -> &Option<Weak<RwLock<Camera>>>;
    fn get_models(&self) -> &BTreeMap<Id, Arc<RwLock<Model>>>;
    fn get_all_models(&self) -> &BTreeMap<Id, Weak<RwLock<Model>>>;
    fn update(&mut self, usize);
    fn render_gbuffer_shadow_maps(
        &self,
        &GraphicApiEngine,
        &Arc<CmdPool>,
        &GBufferFiller,
        &Shadower,
        usize,
    );
    fn update_shadow_makers(&self);
    fn render_shadow_maps(&self, &Shadower, usize, usize);
    fn clean(&mut self);
    fn submit(
        &mut self,
        &GraphicApiEngine,
        &Arc<Semaphore>,
        &Arc<CmdPool>,
        &GBufferFiller,
        &mut Shadower,
        &Deferred,
        Option<&SSAO>,
    ) -> Arc<Semaphore>;
}

pub trait Loadable: Scene + Sized {
    fn new_with_gltf(&Engine, &gltf::Scene, &[u8]) -> Self;
    fn new_with_gx3d(&Engine, &mut Gx3DReader, Id) -> Self;
}

pub trait DefaultScene: Scene + Sized {
    fn default(&Engine) -> Self;
}
