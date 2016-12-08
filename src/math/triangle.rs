extern crate num;

use ::math::vector::{
    Vec3,
    Vec2,
    MathVector,
    VectorElement,
};
use ::math::num::{
    min,
    max,
};
use ::math::aabbox::AABBox3;
use ::math::ray::Ray3;
use ::render::vertex::{
    HasPosition,
    HasNormal,
    HasUV,
};

pub trait Triangle<T>: Sized + Clone + Copy where T: VectorElement {
    fn get_aabb<V>(&self, vertices: &Vec<V>) -> AABBox3<T> where V: HasPosition<T>;
    fn get_midpoint<V>(&self, vertices: &Vec<V>) -> Vec3<T> where V: HasPosition<T>;
    fn intersect<V>(&self, r: &Ray3<T>, tmin: &T, vertices: &Vec<V>) -> Option<(T, T, T)> where V: HasPosition<T>;
    fn barycentric<V>(&self, p: &Vec3<T>, vertices: &Vec<V>) -> Vec3<T> where V: HasPosition<T>;
}

macro_rules! tri {
    ($stru:ident) => (
        impl<T> Triangle<T> for $stru<T> where T: VectorElement {
            fn get_aabb<V>(&self, vertices: &Vec<V>) -> AABBox3<T> where V: HasPosition<T> {
                AABBox3 {
                    blf: Vec3 {
                        x: min((*vertices[self.ind[0]].get_pos()).x, min((*vertices[self.ind[1]].get_pos()).x, (*vertices[self.ind[2]].get_pos()).x)),
                        y: min((*vertices[self.ind[0]].get_pos()).y, min((*vertices[self.ind[1]].get_pos()).y, (*vertices[self.ind[2]].get_pos()).y)),
                        z: min((*vertices[self.ind[0]].get_pos()).z, min((*vertices[self.ind[1]].get_pos()).z, (*vertices[self.ind[2]].get_pos()).z)),
                    },
                    trr: Vec3 {
                        x: max((*vertices[self.ind[0]].get_pos()).x, max((*vertices[self.ind[1]].get_pos()).x, (*vertices[self.ind[2]].get_pos()).x)),
                        y: max((*vertices[self.ind[0]].get_pos()).y, max((*vertices[self.ind[1]].get_pos()).y, (*vertices[self.ind[2]].get_pos()).y)),
                        z: max((*vertices[self.ind[0]].get_pos()).z, max((*vertices[self.ind[1]].get_pos()).z, (*vertices[self.ind[2]].get_pos()).z)),
                    },
                }
            }

            fn get_midpoint<V>(&self, vertices: &Vec<V>) -> Vec3<T> where V: HasPosition<T> {
                ((*vertices[self.ind[0]].get_pos()) + (*vertices[self.ind[1]].get_pos()) + (*vertices[self.ind[2]].get_pos())) / num::cast::<u8, T>(3).unwrap()
            }

            /// return: It will t, u, v,
            fn intersect<V>(&self, r: &Ray3<T>, tmin: &T, vertices: &Vec<V>) -> Option<(T, T, T)> where V: HasPosition<T> {

                let pvec = r.d.cross(&self.edg[1]);
                let det = self.edg[0].dot(&pvec);
                if det == num::cast(0).unwrap() {
                    return None;
                }
                let inv_det: T;
                inv_det = num::cast::<i8, T>(1).unwrap() / det;
                let tvec = r.o - *(vertices[self.ind[0]].get_pos());
                let u = tvec.dot(&pvec) * inv_det;
                if u < num::cast(0).unwrap() || u > num::cast(1).unwrap() {
                    return None;
                }
                let qvec = tvec.cross(&self.edg[0]);
                let v = r.d.dot(&qvec) * inv_det;
                if v < num::cast(0).unwrap() || u + v > num::cast(1).unwrap() {
                    return None;
                }
                let t = self.edg[1].dot(&qvec) * inv_det; // Set distance along ray to intersection
                if t < *tmin {
                    if t > num::cast(1e-9).unwrap() {
                        return Some((t, u, v));
                    }
                }
                None
            }

            // Returns barycentric coordinates of point p on the triangle
            fn barycentric<V>(&self, p: &Vec3<T>, vertices: &Vec<V>) -> Vec3<T> where V: HasPosition<T> {
                let v2_ = *p - (*vertices[self.ind[0]].get_pos());
                let d00 = self.edg[0].dot(&self.edg[0]);
                let d01 = self.edg[0].dot(&self.edg[1]);
                let d11 = self.edg[1].dot(&self.edg[1]);
                let d20 = v2_.dot(&self.edg[0]);
                let d21 = v2_.dot(&self.edg[1]);
                let d = d00*d11 - d01*d01;
                let v = (d11*d20 - d01*d21) / d;
                let w = (d00*d21 - d01*d20) / d;
                let u = num::cast::<i8, T>(1).unwrap() - v - w;
                return Vec3 {
                    x: u,
                    y: v,
                    z: w,
                };
            }
        }
    )
}

#[derive(Debug, Clone, Copy)]
pub struct TexturedTriangle<T> where T: VectorElement {
    edg: [Vec3<T>; 2],
    ind: [usize; 3],
    tedg: [Vec2<T>; 2],
}

impl<T> TexturedTriangle<T> where T: VectorElement, Vec3<T>: MathVector<T> {
    pub fn new<V>(inds: &[usize; 3], vertices: &Vec<V>) -> TexturedTriangle<T> where V: HasPosition<T> + HasNormal<T> + HasUV<T> {
        TexturedTriangle {
            edg : [
                *vertices[inds[1]].get_pos() - *vertices[inds[0]].get_pos(),
                *vertices[inds[2]].get_pos() - *vertices[inds[0]].get_pos(),
            ],
            ind: [
                inds[0],
                inds[1],
                inds[2],
            ],
            tedg: [
                *vertices[inds[1]].get_uv() - *vertices[inds[0]].get_uv(),
                *vertices[inds[2]].get_uv() - *vertices[inds[0]].get_uv(),
            ]
        }
    }
    pub fn get_texture_coord(&self, u: T, v: T) -> Vec2<T> {
        self.tedg[0] * u + self.tedg[1] * v
    }
}

tri!(TexturedTriangle);

#[derive(Debug, Clone, Copy)]
pub struct SolidTriangle<T> where T: VectorElement {
    edg: [Vec3<T>; 2],
    ind: [usize; 3],
}

impl<T> SolidTriangle<T> where T: VectorElement, Vec3<T>: MathVector<T> {
    pub fn new<V>(inds: &[usize; 3], vertices: &Vec<V>) -> SolidTriangle<T> where V: HasPosition<T> + HasNormal<T> {
        SolidTriangle {
            edg: [
                *vertices[inds[1]].get_pos() - *vertices[inds[0]].get_pos(),
                *vertices[inds[2]].get_pos() - *vertices[inds[0]].get_pos(),
            ],
            ind: [
                inds[0],
                inds[1],
                inds[2],
            ],
        }
    }
}

tri!(SolidTriangle);
