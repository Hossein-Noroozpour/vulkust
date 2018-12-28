use super::super::core::types::Real;
use cgmath;
use cgmath::InnerSpace;

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct Plane {
    n: cgmath::Vector3<Real>,
    p: cgmath::Vector3<Real>,
    d: Real,
}

#[repr(u8)]
#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) enum PlaneIntersectStatue {
    Above,
    Intersecting,
    Under,
}

impl Plane {
    pub(crate) fn new(
        p: cgmath::Vector3<Real>,
        f: cgmath::Vector3<Real>,
        s: cgmath::Vector3<Real>,
    ) -> Self {
        let pf = f - p;
        let ps = s - p;
        let n = pf.cross(ps).normalize();
        let d = -(n.dot(p));
        Self { n, p, d }
    }

    // pub(crate) fn new_with_point_normal(
    //     p: cgmath::Vector3<Real>,
    //     n: cgmath::Vector3<Real>,
    // ) -> Self {
    //     let n = n.normalize();
    //     let d = -(n.dot(p));
    //     Self { n, p, d }
    // }

    pub(crate) fn intersect_sphere(
        &self,
        radius: Real,
        center: &cgmath::Vector3<Real>,
    ) -> PlaneIntersectStatue {
        let dis = self.n.dot(*center) + self.d;
        if radius <= dis {
            return PlaneIntersectStatue::Above;
        }
        if radius <= -dis {
            return PlaneIntersectStatue::Under;
        }
        return PlaneIntersectStatue::Intersecting;
    }

    // pub(crate) fn translate(&mut self, l: &cgmath::Vector3<Real>) {
    //     self.p += *l;
    //     self.d = -(self.n.dot(self.p));
    // }

    // pub(crate) fn rotate_around(&mut self, l: &cgmath::Vector3<Real>, m: &cgmath::Matrix4<Real>) {
    //     let mut lp = self.p - l;
    //     lp = (m * lp.extend(1.0)).truncate();
    //     self.n = (m * self.n.extend(0.0)).truncate().normalize();
    //     self.p = lp + l;
    //     self.d = -(self.n.dot(self.p));
    // }

    // pub(crate) fn transform(&mut self, m: &cgmath::Matrix4<Real>) {
    //     self.p = (m * self.p.extend(1.0)).truncate();
    //     self.n = (m * self.n.extend(0.0)).truncate().normalize();
    //     self.d = -(self.n.dot(self.p));
    // }
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            n: cgmath::Vector3::new(0.0, 0.0, 1.0),
            p: cgmath::Vector3::new(0.0, 0.0, 0.0),
            d: 0.0,
        }
    }
}
