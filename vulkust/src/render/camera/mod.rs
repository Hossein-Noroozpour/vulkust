pub mod manager;
pub mod perspective;

use super::super::math::number::Float;
use super::super::math::matrix::{Mat4x4, Mat3x3};
use super::super::math::vector::Vec3;

pub trait Camera<E>
where
    E: Float,
{
    fn travel(&mut self, _d: &Vec3<E>) {
        logf!("Unimplemented");
    }
    fn place(&mut self, _l: &Vec3<E>) {
        logf!("Unimplemented");
    }
    fn rotate_local_x(&mut self) {
        logf!("Unimplemented");
    }
    fn rotate_local_y(&mut self) {
        logf!("Unimplemented");
    }
    fn rotate_local_z(&mut self) {
        logf!("Unimplemented");
    }
    fn rotate(&mut self, _axis: &Vec3<E>) {
        logf!("Unimplemented");
    }
    fn side(&mut self) {
        logf!("Unimplemented");
    }
    fn forward(&mut self) {
        logf!("Unimplemented");
    }
    fn set_viewport(&mut self, _w: E, _h: E) {
        logf!("Unimplemented");
    }
    fn set_fild_of_view(&mut self, _f: E) {
        logf!("Unimplemented");
    }
    fn set_range(&mut self, _s: E, _e: E) {
        logf!("Unimplemented");
    }
    fn get_view(&self) -> &Mat4x4<E> {
        logf!("Unimplemented");
    }
    fn get_view_projection(&self) -> &Mat4x4<E> {
        logf!("Unimplemented");
    }
    fn look_at(&mut self, _eye: &Vec3<E>, _at: &Vec3<E>, _up: &Vec3<E>) {
        logf!("Unimplemented");
    }
    fn set_rotation_speed(&mut self, _speed: E) {
        logf!("Unimplemented");
    }
    fn set_speed(&mut self, _speed: E) {
        logf!("Unimplemented");
    }
}

struct Basic<E>
where
    E: Float,
{
    s: E,
    rs: E,
    p: Vec3<E>,
    x: Vec3<E>,
    y: Vec3<E>,
    z: Vec3<E>,
    r: Mat4x4<E>,
    v: Mat4x4<E>,
}


impl<E> Basic<E>
where
    E: Float,
{
    pub fn new() -> Self {
        Basic {
            s: E::new(0.01),
            rs: E::new(0.1),
            p: Vec3 {
                x: E::new(0.0),
                y: E::new(0.0),
                z: E::new(0.0),
            },
            x: Vec3 {
                x: E::new(1.0),
                y: E::new(0.0),
                z: E::new(0.0),
            },
            y: Vec3 {
                x: E::new(0.0),
                y: E::new(1.0),
                z: E::new(0.0),
            },
            z: Vec3 {
                x: E::new(0.0),
                y: E::new(0.0),
                z: E::new(1.0),
            },
            r: Mat4x4::ident(),
            v: Mat4x4::ident(),
        }
    }
}

impl<E> Camera<E> for Basic<E>
where
    E: Float,
{
    fn travel(&mut self, d: &Vec3<E>) {
        self.p += d;
        let t = Mat4x4::translator(&-d);
        self.v *= &t;
    }

    fn place(&mut self, l: &Vec3<E>) {
        let t = Mat4x4::translator(&-&(l - &self.p));
        self.p = *l;
        self.v *= &t;
    }

    fn rotate_local_x(&mut self) {
        let r = Mat4x4::rotation(-self.rs, &self.x);
        let rr = Mat3x3::rotation(self.rs, &self.x);
        self.y = &rr * &self.y;
        self.z = &rr * &self.z;
        self.r *= &r;
        self.v = &self.r * &Mat4x4::translator(&-&self.p);
    }

    fn rotate_local_y(&mut self) {
        let r = Mat4x4::rotation(-self.rs, &self.y);
        let rr = Mat3x3::rotation(self.rs, &self.y);
        self.x = &rr * &self.x;
        self.z = &rr * &self.z;
        self.r *= &r;
        self.v = &self.r * &Mat4x4::translator(&-&self.p);
    }

    fn rotate_local_z(&mut self) {
        let r = Mat4x4::rotation(-self.rs, &self.z);
        let rr = Mat3x3::rotation(self.rs, &self.z);
        self.x = &rr * &self.x;
        self.y = &rr * &self.y;
        self.r *= &r;
        self.v = &self.r * &Mat4x4::translator(&-&self.p);
    }

    fn rotate(&mut self, axis: &Vec3<E>) {
        let r = Mat4x4::rotation(-self.rs, axis);
        let rr = Mat3x3::rotation(self.rs, axis);
        self.x = &rr * &self.x;
        self.y = &rr * &self.y;
        self.z = &rr * &self.z;
        self.r *= &r;
        self.v = &self.r * &Mat4x4::translator(&-&self.p);
    }

    fn side(&mut self) {
        let d = &self.x * self.s;
        self.travel(&d);
    }

    fn forward(&mut self) {
        let d = &self.z * self.s;
        self.travel(&d);
    }

    fn get_view(&self) -> &Mat4x4<E> {
        &self.v
    }

    fn look_at(&mut self, eye: &Vec3<E>, at: &Vec3<E>, up: &Vec3<E>) {
        self.z = (at - eye).normalized();
        self.x = self.z.cross(up).normalized();
        self.y = self.x.cross(&self.z);
        self.p = *eye;
        self.v = Mat4x4 {
            data: [
                [self.x.x,          self.y.x,       -self.z.x,        E::new(0.0)],
                [self.x.y,          self.y.y,       -self.z.y,        E::new(0.0)],
                [self.x.z,          self.y.z,       -self.z.z,        E::new(0.0)],
                [-self.x.dot(eye), -self.y.dot(eye), self.z.dot(eye), E::new(1.0)],
            ],
        };
        self.r = &self.v * &Mat4x4::translator(&self.p);
    }

    fn set_rotation_speed(&mut self, speed: E) {
        self.rs = speed;
    }

    fn set_speed(&mut self, speed: E) {
        self.s = speed;
    }
}
