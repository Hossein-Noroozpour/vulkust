pub mod manager;

use super::super::math::matrix::{Mat4x4, Mat3x3};
use super::super::math::vector::Vec3;
use super::super::system::file::File;

pub trait Light {}

pub struct Sun {
    pub loc: Vec3<f32>,
    pub far: f32,
    pub near: f32,
    pub size: f32,
    pub dir: Vec3<f32>,
    pub vp: Mat4x4<f32>,
}

impl Sun {
    pub fn new(file: &mut File) -> Self {
        let mut dir = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
        let far = file.read_type();
        let near = file.read_type();
        let size = file.read_type();
        let mut loc = Vec3::new_from_file(file);
        let mut r = Mat3x3::rotation(-file.read_type::<f32>(), &Vec3 { x: 1.0, y: 0.0, z: 0.0 });
        r *= &Mat3x3::rotation(-file.read_type::<f32>(), &Vec3 { x: 0.0, y: 1.0, z: 0.0 });
        r *= &Mat3x3::rotation(-file.read_type::<f32>(), &Vec3 { x: 0.0, y: 0.0, z: 1.0 });
        dir = (&r * &dir).normalized();
        r.scale(size);
        let r = r.to_mat4x4();
        let v = &r * &Mat4x4::translator(&-&loc);
        let vp = &Mat4x4::ortho(1.0, far, near) * &v;
        Sun {
            loc: loc,
            far: far,
            near: near,
            size: size,
            dir: dir,
            vp: vp,
        }
    }
}

impl Light for Sun {}
