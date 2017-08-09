use super::super::super::math::number::Float;
use super::super::super::math::matrix::Mat4x4;
use super::super::super::math::vector::Vec3;
use super::super::super::system::file::File;
use super::{Basic, Camera};

pub struct Perspective<E>
where
    E: Float,
{
    b: Basic<E>,
    fov: E,
    near: E,
    far: E,
    p: Mat4x4<E>,
    vp: Mat4x4<E>,
}

impl<E> Perspective<E>
where
    E: Float,
{
    pub fn new(f: &mut File, ratio: E) -> Self {
        let fov = f.read_type();
        let near = f.read_type();
        let far = f.read_type();
        let b = Basic::new(f, ratio);
        let p = Mat4x4::pers(fov, b.a, near, far);
        let vp = &p * b.get_view();
        Perspective {
            b: b,
            fov: fov,
            near: near,
            far: far,
            p: p,
            vp: vp,
        }
    }
}

impl<E> Camera<E> for Perspective<E>
where
    E: Float,
{
    fn travel(&mut self, d: &Vec3<E>) {
        self.b.travel(d);
        self.vp = &self.p * self.b.get_view();
    }
    fn place(&mut self, l: &Vec3<E>) {
        self.b.place(l);
        self.vp = &self.p * self.b.get_view();
    }
    fn rotate_local_x(&mut self) {
        self.b.rotate_local_x();
        self.vp = &self.p * self.b.get_view();
    }
    fn rotate_local_y(&mut self) {
        self.b.rotate_local_y();
        self.vp = &self.p * self.b.get_view();
    }
    fn rotate_local_z(&mut self) {
        self.b.rotate_local_z();
        self.vp = &self.p * self.b.get_view();
    }
    fn rotate(&mut self, axis: &Vec3<E>) {
        self.b.rotate(axis);
        self.vp = &self.p * self.b.get_view();
    }
    fn side(&mut self) {
        self.b.side();
        self.vp = &self.p * self.b.get_view();
    }
    fn forward(&mut self) {
        self.b.forward();
        self.vp = &self.p * self.b.get_view();
    }
    fn set_viewport(&mut self, w: E, h: E) {
        self.b.a = w / h;
        self.p = Mat4x4::pers(self.fov, self.b.a, self.near, self.far);
        self.vp = &self.p * self.b.get_view();
    }
    fn set_fild_of_view(&mut self, f: E) {
        self.fov = f;
        self.p = Mat4x4::pers(self.fov, self.b.a, self.near, self.far);
        self.vp = &self.p * self.b.get_view();
    }
    fn set_range(&mut self, s: E, e: E) {
        self.far = e;
        self.near = s;
        self.p = Mat4x4::pers(self.fov, self.b.a, self.near, self.far);
        self.vp = &self.p * self.b.get_view();
    }
    fn get_view(&self) -> &Mat4x4<E> {
        self.b.get_view()
    }
    fn get_view_projection(&self) -> &Mat4x4<E> {
        &self.vp
    }
    fn look_at(&mut self, eye: &Vec3<E>, at: &Vec3<E>, up: &Vec3<E>) {
        self.b.look_at(eye, at, up);
        self.vp = &self.p * self.b.get_view();
    }
    fn set_rotation_speed(&mut self, speed: E) {
        self.b.set_rotation_speed(speed);
    }
    fn set_speed(&mut self, speed: E) {
        self.b.set_speed(speed);
    }
}
