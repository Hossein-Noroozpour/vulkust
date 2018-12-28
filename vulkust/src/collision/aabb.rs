use super::super::core::types::Real;

use std::f32::MAX as F32MAX;
use std::f32::MIN as F32MIN;

type Vec3 = cgmath::Vector3<Real>;

#[derive(Clone, Copy)]
#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct Aabb3 {
    max: Vec3,
    min: Vec3,
}

// It has good tolerance for unaccurate actions
// performance got a little decrease for better functionality

impl Aabb3 {
    pub(crate) fn new() -> Self {
        Self {
            max: Vec3::new(F32MIN, F32MIN, F32MIN),
            min: Vec3::new(F32MAX, F32MAX, F32MAX),
        }
    }

    // pub(crate) fn new_with_points(pmin: &Vec3, pmax: &Vec3) -> Self {
    //     Self {
    //         max: *pmax,
    //         min: *pmin,
    //     }
    // }

    pub(crate) fn new_with_center_radius(c: &Vec3, r: Real) -> Self {
        let r = Vec3::new(r, r, r);
        Self {
            max: c + r,
            min: c - r,
        }
    }

    pub(crate) fn insert(&mut self, p: &Vec3) {
        if p.x < self.min.x {
            self.min.x = p.x;
        }
        if p.x > self.max.x {
            self.max.x = p.x;
        }
        if p.y < self.min.y {
            self.min.y = p.y;
        }
        if p.y > self.max.y {
            self.max.y = p.y;
        }
        if p.z < self.min.z {
            self.min.z = p.z;
        }
        if p.z > self.max.z {
            self.max.z = p.z;
        }
    }

    pub(crate) fn insert_aabb(&mut self, o: &Self) {
        if o.min.x < self.min.x {
            self.min.x = o.min.x;
        }
        if o.min.y < self.min.y {
            self.min.y = o.min.y;
        }
        if o.min.z < self.min.z {
            self.min.z = o.min.z;
        }
        if o.max.x > self.max.x {
            self.max.x = o.max.x;
        }
        if o.max.y > self.max.y {
            self.max.y = o.max.y;
        }
        if o.max.z > self.max.z {
            self.max.z = o.max.z;
        }
    }

    // pub(crate) fn intersects_aabb(&self, o: &Self) -> bool {
    //     let mr = (self.max - self.min) * 0.5;
    //     let mc = (self.max + self.min) * 0.5;
    //     let or = (o.max - o.min) * 0.5;
    //     let oc = (o.max + o.min) * 0.5;
    //     let r = or + mr;
    //     let d = mc - oc;
    //     let d = Vec3::new(d.x.abs(), d.y.abs(), d.z.abs());
    //     return d.x < r.x || d.y < r.y || d.z < r.z;
    // }

    pub(crate) fn get_intersection_with_aabb(&self, o: &Self) -> Self {
        return Self {
            min: Vec3::new(
                o.min.x.max(self.min.x),
                o.min.y.max(self.min.y),
                o.min.z.max(self.min.z),
            ),
            max: Vec3::new(
                o.max.x.min(self.max.x),
                o.max.y.min(self.max.y),
                o.max.z.min(self.max.z),
            ),
        };
    }

    pub(crate) fn intersects_center_radius(&self, c: &Vec3, r: Real) -> bool {
        let mr = (self.max - self.min) * 0.5;
        let mc = (self.max + self.min) * 0.5;
        let r = mr + Vec3::new(r, r, r);
        let d = mc - c;
        let d = Vec3::new(d.x.abs(), d.y.abs(), d.z.abs());
        return d.x < r.x || d.y < r.y || d.z < r.z;
    }

    pub(crate) fn get_min_max_diff(&self) -> Vec3 {
        return self.max - self.min;
    }

    pub(crate) fn get_center(&self) -> Vec3 {
        return (self.min + self.max) * 0.5;
    }

    pub(crate) fn get_max(&self) -> Vec3 {
        return self.max;
    }

    // pub(crate) fn get_min(&self) -> Vec3 {
    //     return self.min;
    // }
}
