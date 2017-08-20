use std::ops::{Mul, MulAssign};
use super::number::{Float, Number};
use super::vector::Vec3;
use super::super::system::file::File;

#[repr(simd)]
pub struct SMat4x4D(
    pub f64,
    pub f64,
    pub f64,
    pub f64,
    pub f64,
    pub f64,
    pub f64,
    pub f64,
    pub f64,
    pub f64,
    pub f64,
    pub f64,
    pub f64,
    pub f64,
    pub f64,
    pub f64,
);

#[repr(simd)]
pub struct SMat4x4F(
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
);

// column major matrix
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Mat4x4<E>
where
    E: Number,
{
    pub data: [[E; 4]; 4],
}

impl<E> Mat4x4<E>
where
    E: Float,
{
    pub fn ident() -> Mat4x4<E> {
        Mat4x4 {
            data: [
                [E::new(1.0), E::new(0.0), E::new(0.0), E::new(0.0)],
                [E::new(0.0), E::new(1.0), E::new(0.0), E::new(0.0)],
                [E::new(0.0), E::new(0.0), E::new(1.0), E::new(0.0)],
                [E::new(0.0), E::new(0.0), E::new(0.0), E::new(1.0)],
            ],
        }
    }

    pub fn zero() -> Mat4x4<E> {
        Mat4x4 {
            data: [[E::new(0.0); 4]; 4],
        }
    }

    pub fn rotation(d: E, v: &Vec3<E>) -> Mat4x4<E> {
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
                    wxyonemincos + wzsin,
                    wxzonemincos - wysin,
                    E::new(0.0),
                ],
                [
                    wxyonemincos - wzsin,
                    cosinus + (wy2 * oneminuscos),
                    wxsin + wyzonemincos,
                    E::new(0.0),
                ],
                [
                    wysin + wxzonemincos,
                    wyzonemincos - wxsin,
                    cosinus + (wz2 * oneminuscos),
                    E::new(0.0),
                ],
                [E::new(0.0), E::new(0.0), E::new(0.0), E::new(1.0)],
            ],
        }
    }

    pub fn translate(&mut self, v: &Vec3<E>) {
        self.data[3][0] += v.x;
        self.data[3][1] += v.y;
        self.data[3][2] += v.z;
    }

    pub fn translator(v: &Vec3<E>) -> Self {
        let mut m = Mat4x4::ident();
        m.data[3][0] = v.x;
        m.data[3][1] = v.y;
        m.data[3][2] = v.z;
        return m;
    }

    pub fn set_translation(&mut self, v: &Vec3<E>) {
        self.data[3][0] = v.x;
        self.data[3][1] = v.y;
        self.data[3][2] = v.z;
    }

    pub fn update_rotation(&mut self, m: &Mat3x3<E>) {
        for i in 0..3 {
            for j in 0..3 {
                self.data[i][j] = m.data[i][j];
            }
        }
    }

    pub fn get_smat4x4f(&self) -> SMat4x4F {
        SMat4x4F(
            self.data[0][0].to_f32(),
            self.data[0][1].to_f32(),
            self.data[0][2].to_f32(),
            self.data[0][3].to_f32(),
            self.data[1][0].to_f32(),
            self.data[1][1].to_f32(),
            self.data[1][2].to_f32(),
            self.data[1][3].to_f32(),
            self.data[2][0].to_f32(),
            self.data[2][1].to_f32(),
            self.data[2][2].to_f32(),
            self.data[2][3].to_f32(),
            self.data[3][0].to_f32(),
            self.data[3][1].to_f32(),
            self.data[3][2].to_f32(),
            self.data[3][3].to_f32(),
        )
    }

    pub fn pers(fovy: E, aspect: E, near: E, far: E) -> Self {
        let fovy = (fovy * E::new(0.5)).tan();
        let ys = E::new(1.0) / fovy;
        let xs = ys / aspect;
        let near_far = near - far;
        let zs = (far + near) / near_far;
        let ws = (E::new(2.0) * far * near) / near_far;
        Mat4x4 {
            data: [
                [xs, E::new(0.0), E::new(0.0), E::new(0.0)],
                [E::new(0.0), ys, E::new(0.0), E::new(0.0)],
                [E::new(0.0), E::new(0.0), zs, E::new(-1.0)],
                [E::new(0.0), E::new(0.0), ws, E::new(0.0)],
                // [E::new(0.0), E::new(0.0), zs,           ws],
                // [E::new(0.0), E::new(0.0), E::new(-1.0), E::new(0.0)],
            ],
        }
    }

    pub fn ortho(aspect: E, near: E, far: E) -> Self {
        Mat4x4 {
            data: [
                [E::new(2.0) / aspect, E::new(0.0), E::new(0.0), E::new(0.0)],
                [E::new(0.0), E::new(2.0) * aspect, E::new(0.0), E::new(0.0)],
                [
                    E::new(0.0),
                    E::new(0.0),
                    E::new(2.0) / (near - far),
                    E::new(0.0),
                ],
                [
                    E::new(0.0),
                    E::new(0.0),
                    (far + near) / (near - far),
                    E::new(1.0),
                ],
            ],
        }
    }

    pub fn get_mat3x3(&self) -> Mat3x3<E> {
        let mut mat = Mat3x3::zero();
        for i in 0..3 {
            for j in 0..3 {
                mat.data[i][j] = self.data[i][j];
            }
        }
        return mat;
    }

    pub fn det(&self) -> E {
        (self.data[0][0] *
            ((self.data[1][1] *
                ((self.data[2][2] * self.data[3][3]) - (self.data[2][3] * self.data[3][2]))) -
                (self.data[1][2] *
                    ((self.data[2][1] * self.data[3][3]) - (self.data[2][3] * self.data[3][1]))) +
                (self.data[1][3] *
                    ((self.data[2][1] * self.data[3][2]) - (self.data[2][2] * self.data[3][1]))))) -
            (self.data[0][1] *
                ((self.data[1][0] *
                    ((self.data[2][2] * self.data[3][3]) - (self.data[2][3] * self.data[3][2]))) -
                    (self.data[1][2] *
                        ((self.data[2][0] * self.data[3][3]) -
                            (self.data[2][3] * self.data[3][0]))) +
                    (self.data[1][3] *
                        ((self.data[2][0] * self.data[3][2]) -
                            (self.data[2][2] * self.data[3][0]))))) +
            (self.data[0][2] *
                ((self.data[1][0] *
                    ((self.data[2][1] * self.data[3][3]) - (self.data[2][3] * self.data[3][1]))) -
                    (self.data[1][1] *
                        ((self.data[2][0] * self.data[3][3]) -
                            (self.data[2][3] * self.data[3][0]))) +
                    (self.data[1][3] *
                        ((self.data[2][0] * self.data[3][1]) -
                            (self.data[2][1] * self.data[3][0]))))) -
            (self.data[0][3] *
                ((self.data[1][0] *
                    ((self.data[2][1] * self.data[3][2]) - (self.data[2][2] * self.data[3][1]))) -
                    (self.data[1][1] *
                        ((self.data[2][0] * self.data[3][2]) -
                            (self.data[2][2] * self.data[3][0]))) +
                    (self.data[1][2] *
                        ((self.data[2][0] * self.data[3][1]) -
                            (self.data[2][1] * self.data[3][0])))))
    }

    pub fn inv(&self) -> Self {
        let d = self.det();
        Mat4x4 {
            data: [
                [
                    ((self.data[1][1] *
                        ((self.data[2][2] * self.data[3][3]) -
                            (self.data[2][3] * self.data[3][2]))) -
                        (self.data[1][2] *
                            ((self.data[2][1] * self.data[3][3]) -
                                (self.data[2][3] * self.data[3][1]))) +
                        (self.data[1][3] *
                            ((self.data[2][1] * self.data[3][2]) -
                                (self.data[2][2] * self.data[3][1])))) / d,
                    ((self.data[1][0] *
                        ((self.data[2][2] * self.data[3][3]) -
                            (self.data[2][3] * self.data[3][2]))) -
                        (self.data[1][2] *
                            ((self.data[2][0] * self.data[3][3]) -
                                (self.data[2][3] * self.data[3][0]))) +
                        (self.data[1][3] *
                            ((self.data[2][0] * self.data[3][2]) -
                                (self.data[2][2] * self.data[3][0])))) / d,
                    ((self.data[1][0] *
                        ((self.data[2][1] * self.data[3][3]) -
                            (self.data[2][3] * self.data[3][1]))) -
                        (self.data[1][1] *
                            ((self.data[2][0] * self.data[3][3]) -
                                (self.data[2][3] * self.data[3][0]))) +
                        (self.data[1][3] *
                            ((self.data[2][0] * self.data[3][1]) -
                                (self.data[2][1] * self.data[3][0])))) / d,
                    ((self.data[1][0] *
                        ((self.data[2][1] * self.data[3][2]) -
                            (self.data[2][2] * self.data[3][1]))) -
                        (self.data[1][1] *
                            ((self.data[2][0] * self.data[3][2]) -
                                (self.data[2][2] * self.data[3][0]))) +
                        (self.data[1][2] *
                            ((self.data[2][0] * self.data[3][1]) -
                                (self.data[2][1] * self.data[3][0])))) / d,
                ],
                [
                    ((self.data[0][1] *
                        ((self.data[2][2] * self.data[3][3]) -
                            (self.data[2][3] * self.data[3][2]))) -
                        (self.data[0][2] *
                            ((self.data[2][1] * self.data[3][3]) -
                                (self.data[2][3] * self.data[3][1]))) +
                        (self.data[0][3] *
                            ((self.data[2][1] * self.data[3][2]) -
                                (self.data[2][2] * self.data[3][1])))) / d,
                    ((self.data[0][0] *
                        ((self.data[2][2] * self.data[3][3]) -
                            (self.data[2][3] * self.data[3][2]))) -
                        (self.data[0][2] *
                            ((self.data[2][0] * self.data[3][3]) -
                                (self.data[2][3] * self.data[3][0]))) +
                        (self.data[0][3] *
                            ((self.data[2][0] * self.data[3][2]) -
                                (self.data[2][2] * self.data[3][0])))) / d,
                    ((self.data[0][0] *
                        ((self.data[2][1] * self.data[3][3]) -
                            (self.data[2][3] * self.data[3][1]))) -
                        (self.data[0][1] *
                            ((self.data[2][0] * self.data[3][3]) -
                                (self.data[2][3] * self.data[3][0]))) +
                        (self.data[0][3] *
                            ((self.data[2][0] * self.data[3][1]) -
                                (self.data[2][1] * self.data[3][0])))) / d,
                    ((self.data[0][0] *
                        ((self.data[2][1] * self.data[3][2]) -
                            (self.data[2][2] * self.data[3][1]))) -
                        (self.data[0][1] *
                            ((self.data[2][0] * self.data[3][2]) -
                                (self.data[2][2] * self.data[3][0]))) +
                        (self.data[0][2] *
                            ((self.data[2][0] * self.data[3][1]) -
                                (self.data[2][1] * self.data[3][0])))) / d,
                ],
                [
                    ((self.data[0][1] *
                        ((self.data[1][2] * self.data[3][3]) -
                            (self.data[1][3] * self.data[3][2]))) -
                        (self.data[0][2] *
                            ((self.data[1][1] * self.data[3][3]) -
                                (self.data[1][3] * self.data[3][1]))) +
                        (self.data[0][3] *
                            ((self.data[1][1] * self.data[3][2]) -
                                (self.data[1][2] * self.data[3][1])))) / d,
                    ((self.data[0][0] *
                        ((self.data[1][2] * self.data[3][3]) -
                            (self.data[1][3] * self.data[3][2]))) -
                        (self.data[0][2] *
                            ((self.data[1][0] * self.data[3][3]) -
                                (self.data[1][3] * self.data[3][0]))) +
                        (self.data[0][3] *
                            ((self.data[1][0] * self.data[3][2]) -
                                (self.data[1][2] * self.data[3][0])))) / d,
                    ((self.data[0][0] *
                        ((self.data[1][1] * self.data[3][3]) -
                            (self.data[1][3] * self.data[3][1]))) -
                        (self.data[0][1] *
                            ((self.data[1][0] * self.data[3][3]) -
                                (self.data[1][3] * self.data[3][0]))) +
                        (self.data[0][3] *
                            ((self.data[1][0] * self.data[3][1]) -
                                (self.data[1][1] * self.data[3][0])))) / d,
                    ((self.data[0][0] *
                        ((self.data[1][1] * self.data[3][2]) -
                            (self.data[1][2] * self.data[3][1]))) -
                        (self.data[0][1] *
                            ((self.data[1][0] * self.data[3][2]) -
                                (self.data[1][2] * self.data[3][0]))) +
                        (self.data[0][2] *
                            ((self.data[1][0] * self.data[3][1]) -
                                (self.data[1][1] * self.data[3][0])))) / d,
                ],
                [
                    ((self.data[0][1] *
                        ((self.data[1][2] * self.data[2][3]) -
                            (self.data[1][3] * self.data[2][2]))) -
                        (self.data[0][2] *
                            ((self.data[1][1] * self.data[2][3]) -
                                (self.data[1][3] * self.data[2][1]))) +
                        (self.data[0][3] *
                            ((self.data[1][1] * self.data[2][2]) -
                                (self.data[1][2] * self.data[2][1])))) / d,
                    ((self.data[0][0] *
                        ((self.data[1][2] * self.data[2][3]) -
                            (self.data[1][3] * self.data[2][2]))) -
                        (self.data[0][2] *
                            ((self.data[1][0] * self.data[2][3]) -
                                (self.data[1][3] * self.data[2][0]))) +
                        (self.data[0][3] *
                            ((self.data[1][0] * self.data[2][2]) -
                                (self.data[1][2] * self.data[2][0])))) / d,
                    ((self.data[0][0] *
                        ((self.data[1][1] * self.data[2][3]) -
                            (self.data[1][3] * self.data[2][1]))) -
                        (self.data[0][1] *
                            ((self.data[1][0] * self.data[2][3]) -
                                (self.data[1][3] * self.data[2][0]))) +
                        (self.data[0][3] *
                            ((self.data[1][0] * self.data[2][1]) -
                                (self.data[1][1] * self.data[2][0])))) / d,
                    ((self.data[0][0] *
                        ((self.data[1][1] * self.data[2][2]) -
                            (self.data[1][2] * self.data[2][1]))) -
                        (self.data[0][1] *
                            ((self.data[1][0] * self.data[2][2]) -
                                (self.data[1][2] * self.data[2][0]))) +
                        (self.data[0][2] *
                            ((self.data[1][0] * self.data[2][1]) -
                                (self.data[1][1] * self.data[2][0])))) / d,
                ],
            ],
        }
    }

    pub fn new_from_file(f: &mut File) -> Self {
        let mut data = [[E::new(0.0); 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                data[i][j] = f.read_type();
            }
        }
        Mat4x4 { data: data }
    }
}

impl<'a, 'b, E> Mul<&'b Vec3<E>> for &'a Mat4x4<E>
where
    E: Number,
{
    type Output = Vec3<E>;
    fn mul(self, o: &'b Vec3<E>) -> Vec3<E> {
        Vec3 {
            x: self.data[0][0] * o.x + self.data[1][0] * o.y + self.data[2][0] * o.z +
                self.data[3][0],
            y: self.data[0][1] * o.x + self.data[1][1] * o.y + self.data[2][1] * o.z +
                self.data[3][1],
            z: self.data[0][2] * o.x + self.data[1][2] * o.y + self.data[2][2] * o.z +
                self.data[3][2],
        }
    }
}

impl<'a, 'b, E> Mul<&'b Mat4x4<E>> for &'a Mat4x4<E>
where
    E: Float,
{
    type Output = Mat4x4<E>;
    fn mul(self, o: &'b Mat4x4<E>) -> Mat4x4<E> {
        let mut m = Mat4x4::zero();
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    m.data[j][i] += o.data[j][k] * self.data[k][i];
                }
            }
        }
        m
    }
}

impl<'a, E> MulAssign<&'a Mat4x4<E>> for Mat4x4<E>
where
    E: Float,
{
    fn mul_assign(&mut self, o: &'a Mat4x4<E>) {
        let mut data = [[E::new(0.0); 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    data[j][i] += o.data[j][k] * self.data[k][i];
                }
            }
        }
        self.data = data;
    }
}

impl<E> Default for Mat4x4<E> where
    E: Float,
{
    fn default() -> Self {
        Mat4x4::ident()
    }
}

#[repr(simd)]
pub struct SMat3x3F(
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Mat3x3<E>
where
    E: Number,
{
    pub data: [[E; 3]; 3],
}

impl<E> Mat3x3<E>
where
    E: Float,
{
    pub fn zero() -> Self {
        Mat3x3 {
            data: [[E::new(0.0); 3]; 3],
        }
    }

    pub fn ident() -> Self {
        let mut data = [[E::new(0.0); 3]; 3];
        for i in 0..3 {
            data[i][i] = E::new(0.0);
        }
        Mat3x3 { data: data }
    }

    pub fn det(&self) -> E {
        (self.data[0][0] *
            ((self.data[1][1] * self.data[2][2]) - (self.data[1][2] * self.data[2][1]))) -
            (self.data[0][1] *
                ((self.data[1][0] * self.data[2][2]) - (self.data[1][2] * self.data[2][0]))) +
            (self.data[0][2] *
                ((self.data[1][0] * self.data[2][1]) - (self.data[1][1] * self.data[2][0])))
    }

    pub fn inv(&self) -> Self {
        let d = self.det();
        Mat3x3 {
            data: [
                [
                    ((self.data[1][1] * self.data[2][2]) - (self.data[1][2] * self.data[2][1])) / d,
                    ((self.data[1][0] * self.data[2][2]) - (self.data[1][2] * self.data[2][0])) /
                        -d,
                    ((self.data[1][0] * self.data[2][1]) - (self.data[1][1] * self.data[2][0])) / d,
                ],
                [
                    ((self.data[0][1] * self.data[2][2]) - (self.data[0][2] * self.data[2][1])) /
                        -d,
                    ((self.data[0][0] * self.data[2][2]) - (self.data[0][2] * self.data[2][0])) / d,
                    ((self.data[0][0] * self.data[2][1]) - (self.data[0][1] * self.data[2][0])) /
                        -d,
                ],
                [
                    ((self.data[0][1] * self.data[1][2]) - (self.data[0][2] * self.data[1][1])) / d,
                    ((self.data[0][0] * self.data[1][2]) - (self.data[0][2] * self.data[1][0])) /
                        -d,
                    ((self.data[0][0] * self.data[1][1]) - (self.data[0][1] * self.data[1][0])) / d,
                ],
            ],
        }
    }

    pub fn t(&self) -> Self {
        Mat3x3 {
            data: [
                [self.data[0][0], self.data[1][0], self.data[2][0]],
                [self.data[0][1], self.data[1][1], self.data[2][1]],
                [self.data[0][2], self.data[1][2], self.data[2][2]],
            ],
        }
    }

    pub fn get_smat3x3f(&self) -> SMat3x3F {
        SMat3x3F(
            self.data[0][0].to_f32(),
            self.data[0][1].to_f32(),
            self.data[0][2].to_f32(),
            self.data[1][0].to_f32(),
            self.data[1][1].to_f32(),
            self.data[1][2].to_f32(),
            self.data[2][0].to_f32(),
            self.data[2][1].to_f32(),
            self.data[2][2].to_f32(),
        )
    }

    // v: must be normalized
    pub fn rotation(d: E, v: &Vec3<E>) -> Self {
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
        Mat3x3 {
            data: [
                [
                    cosinus + (wx2 * oneminuscos),
                    wxyonemincos + wzsin,
                    wxzonemincos - wysin,
                ],
                [
                    wxyonemincos - wzsin,
                    cosinus + (wy2 * oneminuscos),
                    wxsin + wyzonemincos,
                ],
                [
                    wysin + wxzonemincos,
                    wyzonemincos - wxsin,
                    cosinus + (wz2 * oneminuscos),
                ],
            ],
        }
    }

    pub fn to_mat4x4(&self) -> Mat4x4<E> {
        let mut m = Mat4x4::zero();
        for i in 0..3 {
            for j in 0..3 {
                m.data[i][j] = self.data[i][j];
            }
        }
        return m;
    }

    pub fn scale(&mut self, e: E) {
        for i in 0..3 {
            self.data[i][i] *= e;
        }
    }
}

impl<'a, 'b, E> Mul<&'b Vec3<E>> for &'a Mat3x3<E>
where
    E: Number,
{
    type Output = Vec3<E>;
    fn mul(self, o: &'b Vec3<E>) -> Vec3<E> {
        Vec3 {
            x: self.data[0][0] * o.x + self.data[1][0] * o.y + self.data[2][0] * o.z,
            y: self.data[0][1] * o.x + self.data[1][1] * o.y + self.data[2][1] * o.z,
            z: self.data[0][2] * o.x + self.data[1][2] * o.y + self.data[2][2] * o.z,
        }
    }
}

impl<'a, 'b, E> Mul<&'b Mat3x3<E>> for &'a Mat3x3<E>
where
    E: Float,
{
    type Output = Mat3x3<E>;
    fn mul(self, o: &'b Mat3x3<E>) -> Mat3x3<E> {
        let mut m = Mat3x3::zero();
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    m.data[j][i] += o.data[j][k] * self.data[k][i];
                }
            }
        }
        m
    }
}

impl<'a, E> MulAssign<&'a Mat3x3<E>> for Mat3x3<E>
where
    E: Float,
{
    fn mul_assign(&mut self, o: &'a Mat3x3<E>) {
        let mut data = [[E::new(0.0); 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    data[j][i] += o.data[j][k] * self.data[k][i];
                }
            }
        }
        self.data = data;
    }
}
