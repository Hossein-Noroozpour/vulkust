use super::super::core::types::Real;
use super::plane::{Plane, PlaneIntersectStatue};
use cgmath;

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct Frustum {
    planes: [Plane; 6],
}

impl Frustum {
    pub(crate) fn new(planes: [Plane; 6]) -> Self {
        Self { planes }
    }

    pub(crate) fn intersects_center_radius(
        &self,
        center: &cgmath::Vector3<Real>,
        radius: Real,
    ) -> bool {
        for f in &self.planes {
            let s = f.intersect_sphere(radius, center);
            match s {
                PlaneIntersectStatue::Above => return false,
                _ => (),
            }
        }
        return true;
    }
}

impl Default for Frustum {
    fn default() -> Self {
        Self {
            planes: [
                Plane::default(),
                Plane::default(),
                Plane::default(),
                Plane::default(),
                Plane::default(),
                Plane::default(),
            ],
        }
    }
}
