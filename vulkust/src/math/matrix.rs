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
use super::number::Float;
use super::vector::{
    Vec2,
    Vec3,
};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Mat4x4<E> where E: Float {
    pub data: [[E; 4]; 4],
}

impl<E> Mat4x4<E> where E: Float {
    pub fn new() -> Mat4x4<E> {
        Mat4x4 {
            data: [
                [E::new(1.0), E::new(0.0), E::new(0.0), E::new(0.0)],
                [E::new(0.0), E::new(1.0), E::new(0.0), E::new(0.0)],
                [E::new(0.0), E::new(0.0), E::new(1.0), E::new(0.0)],
                [E::new(0.0), E::new(0.0), E::new(0.0), E::new(1.0)],
            ],
        }
    }

    pub fn rotation_transform(d: &E, v: &Vec3<E>) -> Mat4x4<E> {
        let sinus: E = E::new(d.to().sin());
		let cosinus: E = E::new(d.to().cos());
		let oneminuscos = E::new(1.0) - cosinus;
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
                    E::new(0.0),
                ],
    		    [
                    wzsin + wxyonemincos,
                    cosinus + (wy2 * oneminuscos),
                    wyzonemincos - wxsin,
                    E::new(0.0),
                ],
    		    [
                    wxzonemincos - wysin,
                    wxsin + wyzonemincos,
                    cosinus + (wz2 * oneminuscos),
                    E::new(0.0),
                ],
    		    [
                    E::new(0.0),
                    E::new(0.0),
                    E::new(0.0),
                    E::new(1.0),
                ],
            ],
        }
    }
}

impl<E> Mul<Vec3<E>> for Mat4x4<E> where E: Float {
    type Output = Vec3<E>;
    fn mul(self, o: Vec3<E>) -> Vec3<E> {
        Vec3 {
            x: self.data[0][0] * o.x + self.data[0][1] * o.y + self.data[0][2] * o.z + self.data[0][3],
            y: self.data[1][0] * o.x + self.data[1][1] * o.y + self.data[1][2] * o.z + self.data[1][3],
            z: self.data[2][0] * o.x + self.data[2][1] * o.y + self.data[2][2] * o.z + self.data[2][3],
        }
    }
}

impl<E> Mul<Mat4x4<E>> for Mat4x4<E> where E: Float {
    type Output = Mat4x4<E>;
    fn mul(self, o: Mat4x4<E>) -> Mat4x4<E> {
        let mut m = Mat4x4::new();
        for i in 0..4 {
            for j in 0..4 {
                m.data[i][j] = E::new(0.0);
                for k in 0..4 {
                    m.data[i][j] += self.data[i][k] * o.data[k][j];
                }
            }
        }
        m
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Mat3x3<E> where E: Float {
    pub data: [[E; 3]; 3],
}
