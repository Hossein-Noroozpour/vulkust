use super::engine::Engine;
use super::object::{Object, Transferable};
use super::sync::Semaphore;
use std::sync::{Arc};

pub mod directional;
pub mod lamp;
pub mod manager;
pub mod point;
pub mod sun;

pub use self::directional::{
    Base as DirectionalBase, Directional, DirectionalUniform, ShadowAccumulatorDirectionalUniform,
};
pub use self::manager::Manager;
pub use self::point::{Base as PointBase, Point, PointUniform};
pub use self::sun::Sun;

#[cfg_attr(debug_mode, derive(Debug))]
#[repr(u8)]
enum TypeId {
    Sun = 1,
    Lamp = 2,
}

pub trait Light: Object + Transferable {
    fn to_directional(&self) -> Option<&dyn Directional>;
    fn to_mut_directional(&mut self) -> Option<&mut dyn Directional>;
    fn to_point(&self) -> Option<&dyn Point>;
    fn to_mut_point(&mut self) -> Option<&mut dyn Point>;
    fn to_shadow_maker(&self) -> Option<&dyn ShadowMaker>;
    fn to_mut_shadow_maker(&mut self) -> Option<&mut dyn ShadowMaker>;
    fn update(&mut self);
}

pub trait ShadowMaker: Light {
    fn shadow(&self, &mut Model, &Arc<RwLock<Model>>, usize);
    fn begin_secondary_commands(&self, g_engine: &GraphicApiEngine, cmd_pool: &Arc<CmdPool>, shadower: &Shadower, usize, usize);
    fn render_shadow_mapper(&self, shadower: &Shadower, frame_number: usize, kernel_index: usize);
    fn submit_shadow_mapper(
        &mut self,
        semaphore: &Semaphore,
        g_engine: &GraphicApiEngine,
        sadower: &Shadower,
        frame_number: usize,
    ) -> Arc<Semaphore>;
}

pub trait DefaultLighting: Light {
    fn default(eng: &Engine) -> Self;
}
