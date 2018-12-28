use super::super::super::core::types::Real;
use super::super::config::MAX_DIRECTIONAL_CASCADES_MATRIX_COUNT;
use super::{Light, Sun};

use cgmath;

pub trait Directional: Light {
    fn to_sun(&self) -> Option<&Sun>;
    fn to_mut_sun(&mut self) -> Option<&mut Sun>;
    fn to_base(&self) -> Option<&Base>;
    fn to_mut_base(&mut self) -> Option<&mut Base>;
    fn update_uniform(&self, &mut DirectionalUniform);
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(debug_mode, derive(Debug))]
pub struct DirectionalUniform {
    pub(super) color: cgmath::Vector4<Real>,
    pub(super) direction: cgmath::Vector4<Real>,
}

impl DirectionalUniform {
    pub(crate) fn new() -> Self {
        Self {
            color: cgmath::Vector4::new(1.0, 1.0, 1.0, 1.0),
            direction: cgmath::Vector4::new(0.0, 0.0, -1.0, 1.0),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(debug_mode, derive(Debug))]
pub struct ShadowAccumulatorDirectionalUniform {
    pub(super) view_projection_biases:
        [cgmath::Matrix4<Real>; MAX_DIRECTIONAL_CASCADES_MATRIX_COUNT as usize],
    pub(super) direction_strength: cgmath::Vector4<Real>,
    pub(super) cascades_count: u32,
    pub(super) light_index: u32,
}

impl ShadowAccumulatorDirectionalUniform {
    pub(super) fn new() -> Self {
        Self {
            view_projection_biases: [cgmath::Matrix4::new(
                1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.5, 1.0,
            ); MAX_DIRECTIONAL_CASCADES_MATRIX_COUNT as usize],
            direction_strength: cgmath::Vector4::new(0.0, 0.0, -1.0, 1.0),
            cascades_count: 0,
            light_index: 0,
        }
    }
}

pub struct Base {}
