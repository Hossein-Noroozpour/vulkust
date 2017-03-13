extern crate num;

use ::math::vector::{
    Vec3,
    VectorElement,
};

macro_rules! ncu8 {
    ($var:expr) => (
        num::cast::<u8, E>($var).unwrap()
    )
}

#[derive(Debug, Clone, Copy)]
pub struct Ray3<E> where E: VectorElement {
    pub o:    Vec3<E>,
    pub d:    Vec3<E>,
    pub invd: Vec3<E>,
}

impl<E> Ray3<E> where E: VectorElement {
    pub fn new(o: Vec3<E>, d: Vec3<E>) -> Ray3<E> {
        Ray3 {
            o: o,
            d: d,
            invd: Vec3 {
                x: ncu8!(1) / d.x,
                y: ncu8!(1) / d.y,
                z: ncu8!(1) / d.z,
            }
        }
    }
}
