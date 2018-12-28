use super::super::super::core::types::Real;
use super::Light;

pub trait Point: Light {
    fn update_uniform(&self, &mut PointUniform);
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(debug_mode, derive(Debug))]
pub struct PointUniform {
    color: cgmath::Vector4<Real>,
    position_radius: cgmath::Vector4<Real>,
}

impl PointUniform {
    pub fn new() -> Self {
        PointUniform {
            color: cgmath::Vector4::new(0.0, 0.0, 0.0, 0.0),
            position_radius: cgmath::Vector4::new(0.0, 0.0, 0.0, 0.0),
        }
    }
}
