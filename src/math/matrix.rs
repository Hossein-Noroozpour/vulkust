extern crate num;

use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign
};

use ::math::vector::{
    Vec2,
    Vec3,
    MathVector,
    VectorElement,
};
use ::io::file::Stream;

pub trait Mat {
    fn read(&mut self, s: &mut Stream);
}

pub trait Mat4<E>: Mat + Mul<Vec3<E>> + Mul where E: VectorElement, Self: Sized {
}

#[derive(Debug, Clone, Copy)]
pub struct Mat4x4<E> where E: VectorElement {
    pub data: [[E; 4]; 4],
}

impl<E> Mat4x4<E> where E: VectorElement {
    pub fn new() -> Mat4x4<E> {
        Mat4x4 {
            data: [
                [num::cast(1).unwrap(), num::cast(0).unwrap(), num::cast(0).unwrap(), num::cast(0).unwrap()],
                [num::cast(0).unwrap(), num::cast(1).unwrap(), num::cast(0).unwrap(), num::cast(0).unwrap()],
                [num::cast(0).unwrap(), num::cast(0).unwrap(), num::cast(1).unwrap(), num::cast(0).unwrap()],
                [num::cast(0).unwrap(), num::cast(0).unwrap(), num::cast(0).unwrap(), num::cast(1).unwrap()],
            ],
        }
    }

    pub fn rotation_transform(d: &E, v: &Vec3<E>) -> Mat4x4<E> {
        let sinus: E = num::cast(num::cast::<E, f64>(*d).unwrap().sin()).unwrap();
		let cosinus: E = num::cast(num::cast::<E, f64>(*d).unwrap().cos()).unwrap();
		let oneminuscos = num::cast::<u8, E>(1).unwrap() - cosinus;
		let w = v;
		let wx2 = w.x * w.x;
		let wxy = w.x * w.y;
		let wxz = w.x * w.z;
		let wy2 = w.y * w.y;
		let wyz = w.y * w.z;
		let wz2 = w.z * w.z;
		let wxyonemincos = wxy * oneminuscos;
		let wxzonemincos = wxz * oneminuscos;
		let wyzonemincos = wyz * oneminuscos;
		let wxsin = w.x * sinus;
		let wysin = w.y * sinus;
		let wzsin = w.z * sinus;
		Mat4x4 {
            data: [
    		    [
                    cosinus + (wx2 * oneminuscos),
                    wxyonemincos - wzsin,
                    wysin + wxzonemincos,
                    num::cast(0).unwrap(),
                ],
    		    [
                    wzsin + wxyonemincos,
                    cosinus + (wy2 * oneminuscos),
                    wyzonemincos - wxsin,
                    num::cast(0).unwrap(),
                ],
    		    [
                    wxzonemincos - wysin,
                    wxsin + wyzonemincos,
                    cosinus + (wz2 * oneminuscos),
                    num::cast(0).unwrap(),
                ],
    		    [
                    num::cast(0).unwrap(),
                    num::cast(0).unwrap(),
                    num::cast(0).unwrap(),
                    num::cast(1).unwrap(),
                ],
            ],
        }
    }
}

impl<E> Mat for Mat4x4<E> where E: VectorElement {
    fn read(&mut self, s: &mut Stream) {
        for i in 0..4 {
            for j in 0..4 {
                self.data[j][i] = num::cast(s.read(&0f32)).unwrap();
            }
        }
    }
}

impl<E> Mul<Vec3<E>> for Mat4x4<E> where E: VectorElement {
    type Output = Vec3<E>;
    fn mul(self, o: Vec3<E>) -> Vec3<E> {
        Vec3 {
            x: self.data[0][0] * o.x + self.data[0][1] * o.y + self.data[0][2] * o.z + self.data[0][3],
            y: self.data[1][0] * o.x + self.data[1][1] * o.y + self.data[1][2] * o.z + self.data[1][3],
            z: self.data[2][0] * o.x + self.data[2][1] * o.y + self.data[2][2] * o.z + self.data[2][3],
        }
    }
}

impl<E> Mul<Mat4x4<E>> for Mat4x4<E> where E: VectorElement {
    type Output = Mat4x4<E>;
    fn mul(self, o: Mat4x4<E>) -> Mat4x4<E> {
        let mut m = Mat4x4::new();
        for i in 0..4 {
            for j in 0..4 {
                m.data[i][j] = num::cast(0).unwrap();
                for k in 0..4 {
                    m.data[i][j] += self.data[i][k] * o.data[k][j];
                }
            }
        }
        m
    }
}

impl<E> Mat4<E> for Mat4x4<E> where E: VectorElement {}
