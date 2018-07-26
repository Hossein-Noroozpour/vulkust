use super::engine::Engine;
use super::gx3d::Gx3DReader;
use super::texture::Texture;
use std::sync::{Arc, RwLock};

use math;

#[repr(u8)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum Field {
    Float = 1,
    Texture = 2,
    Vector = 3,
}

#[repr(C)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Uniform {
    pub base_color: math::Vector3<f32>,
    pub base_color_factor: math::Vector3<f32>,
    pub metallic_roughness: math::Vector3<f32>,
    pub metallic_factor: f32,
    pub roughness_factor: f32,
    pub normal: math::Vector3<f32>,
    pub normal_scale: f32,
    pub occlusion: math::Vector3<f32>,
    pub occlusion_strength: f32,
    pub emissive: math::Vector3<f32>,
    pub emissive_factor: math::Vector3<f32>,
    pub alpha: f32,
    pub alpha_cutoff: f32,
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub enum TranslucencyMode {
    Cutoff,
    Tansparent,
    Opaque,
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Material {
    pub base_color_texture: Option<Arc<RwLock<Texture>>>,
    pub base_color_factor_texture: Option<Arc<RwLock<Texture>>>,
    pub metallic_roughness_texture: Option<Arc<RwLock<Texture>>>,
    pub normal_texture: Option<Arc<RwLock<Texture>>>,
    pub occlusion_texture: Option<Arc<RwLock<Texture>>>,
    pub emissive_texture: Option<Arc<RwLock<Texture>>>,
    pub emissive_factor_texture: Option<Arc<RwLock<Texture>>>,
}

impl Material {
    pub fn new_with_gx3d(engine: &Arc<RwLock<Engine>>, reader: &mut Gx3DReader) -> Self {
        unimplemented!();
    }
}
