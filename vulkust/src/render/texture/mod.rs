use super::super::core::object::Object as CoreObject;
use super::image::View as ImageView;
use super::sampler::Sampler;
use std::sync::Arc;

pub mod cube;
pub mod manager;
pub mod t2d;

pub use self::cube::Cube;
pub use self::manager::Manager;
pub use self::t2d::Texture2D;
use super::engine::Engine;
use crate::core::{gx3d::Gx3DReader, types::Id};

#[repr(u8)]
#[cfg_attr(debug_mode, derive(Debug))]
pub enum TextureType {
    T2D = 1,
    T3D = 2,
    Cube = 3,
}

pub trait Texture: CoreObject {
    fn get_image_view(&self) -> &Arc<ImageView>;
    fn get_sampler(&self) -> &Arc<Sampler>;
}

pub trait Loadable: Sized {
    fn new_with_gltf(gltf: &gltf::Texture, engine: &Engine, data: &[u8]) -> Self;
    fn new_with_gx3d(engine: &Engine, reader: &mut Gx3DReader, id: Id) -> Self;
}
