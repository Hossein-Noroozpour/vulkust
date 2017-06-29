use std::ops::Mul;
use super::number::Number;
use super::vector::Vec3;

#[repr(simd)]
pub struct SMat4x4D(f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64);

#[repr(simd)]
pub struct SMat4x4F(f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32);

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
    E: Number,
{
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

    pub fn translate(&mut self, x: E, y: E, z: E) {
        self.data[3][0] += x;
        self.data[3][1] += y;
        self.data[3][2] += z;
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

    pub fn projection(fovy: E, aspect: E, near: E, far: E) -> Self {
        let ys = E::new(1.0) / (fovy * E::new(0.5)).tan();
        let xs = ys / aspect;
        let zs = far / (near - far);
        Mat4x4 {
            data: [
                [xs, E::new(0.0), E::new(0.0), E::new(0.0)],
                [E::new(0.0), ys, E::new(0.0), E::new(0.0)],
                [E::new(0.0), E::new(0.0), zs, E::new(-1.0)],
                [E::new(0.0), E::new(0.0), near * zs, E::new(0.0)],
            ],
        }
    }

    pub fn get_mat3x3(&self) -> Mat3x3<E> {
        let mut mat = Mat3x3::new();
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
    E: Number,
{
    type Output = Mat4x4<E>;
    fn mul(self, o: &'b Mat4x4<E>) -> Mat4x4<E> {
        let mut m = Mat4x4::new();
        for i in 0..4 {
            for j in 0..4 {
                m.data[i][j] = E::new(0.0);
                for k in 0..4 {
                    m.data[i][j] += o.data[j][k] * self.data[k][i];
                }
            }
        }
        m
    }
}

#[repr(simd)]
pub struct SMat3x3F(f32, f32, f32, f32, f32, f32, f32, f32, f32);

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
    E: Number,
{
    pub fn new() -> Self {
        Mat3x3 { data: [[E::new(0.0); 3]; 3] }
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
                    ((self.data[1][0] * self.data[2][2]) - (self.data[1][2] * self.data[2][0])) / d,
                    ((self.data[1][0] * self.data[2][1]) - (self.data[1][1] * self.data[2][0])) / d,
                ],
                [
                    ((self.data[0][1] * self.data[2][2]) - (self.data[0][2] * self.data[2][1])) / d,
                    ((self.data[0][0] * self.data[2][2]) - (self.data[0][2] * self.data[2][0])) / d,
                    ((self.data[0][0] * self.data[2][1]) - (self.data[0][1] * self.data[2][0])) / d,
                ],
                [
                    ((self.data[0][1] * self.data[1][2]) - (self.data[0][2] * self.data[1][1])) / d,
                    ((self.data[0][0] * self.data[1][2]) - (self.data[0][2] * self.data[1][0])) / d,
                    ((self.data[0][0] * self.data[1][1]) - (self.data[0][1] * self.data[1][0])) / d,
                ],
            ],
        }
    }

    pub fn t(&self) -> Self {
        Mat3x3 {
            data: [
                [self.data[0][0], self.data[1][0], self.data[2][0], ],
                [self.data[0][1], self.data[1][1], self.data[2][1], ],
                [self.data[0][2], self.data[1][2], self.data[2][2], ],
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
}
