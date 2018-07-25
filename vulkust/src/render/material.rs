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
    pub base_color: math::Vector4<f32>,
    pub base_color_factor: math::Vector4<f32>,
    pub metallic_roughness: math::Vector4<f32>,
    pub metallic_factor: f32,
    pub roughness_factor: f32,
    pub normal: math::Vector4<f32>,
    pub normal_scale: f32,
    pub occlusion: math::Vector4<f32>,
    pub occlusion_strength: f32,
    pub emissive: math::Vector4<f32>,
    pub emissive_factor: math::Vector4<f32>,
    pub alpha_cutoff: f32,
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Material {

}