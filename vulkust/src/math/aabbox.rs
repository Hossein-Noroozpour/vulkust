extern crate num;

use ::math::num::{
    min,
    max,
};

use ::math::vector::{
    Vec3,
    Axis,
    MathVector,
    VectorElement,
};

use ::math::ray::Ray3;

pub trait ExpandableToOther {
    fn expand(&mut self, o: &Self);
}

pub trait ExpandableToPoint3<T> where T: VectorElement, Vec3<T>: MathVector<T> {
    fn expand(&mut self, o: &Vec3<T>);
}

#[derive(Debug, Clone, Copy)]
pub struct AABBox3<T> where T: VectorElement, Vec3<T>: MathVector<T> {
    pub blf: Vec3<T>,
    pub trr: Vec3<T>
}

impl<T> ExpandableToOther for AABBox3<T> where T: VectorElement, Vec3<T>: MathVector<T> {

    fn expand(&mut self, o : &Self) {
        if o.blf.x < self.blf.x { self.blf.x = o.blf.x; }
        if o.blf.y < self.blf.y { self.blf.y = o.blf.y; }
        if o.blf.z < self.blf.z { self.blf.z = o.blf.z; }

        if o.trr.x > self.trr.x { self.trr.x = o.trr.x; }
        if o.trr.y > self.trr.y { self.trr.y = o.trr.y; }
        if o.trr.z > self.trr.z { self.trr.z = o.trr.z; }
    }
}

impl<T> ExpandableToPoint3<T> for AABBox3<T> where T: VectorElement, Vec3<T>: MathVector<T> {

    fn expand(&mut self, p : &Vec3<T>) {
        if p.x < self.blf.x { self.blf.x = p.x; }
        if p.y < self.blf.y { self.blf.y = p.y; }
        if p.z < self.blf.z { self.blf.z = p.z; }
    }
}

impl<T> AABBox3<T> where T: VectorElement, Vec3<T>: MathVector<T> {

    pub fn new() -> AABBox3<T> {
        AABBox3 {
            blf: Vec3::new(num::cast(0).unwrap()),
            trr: Vec3::new(num::cast(0).unwrap()),
        }
    }

    pub fn get_longest_axis(&self) -> Axis {
        let diff = self.trr - self.blf; // TODO check for occurance, if it is too much store it in box
        if diff.x > diff.y && diff.x > diff.z { return Axis::X; }
        if diff.y > diff.x && diff.y > diff.z { return Axis::Y; }
        return Axis::Z;
    }

    // Check if ray intersects with box. Returns true/false and stores distance in t
    pub fn intersection(&self, r: &Ray3<T>) -> (bool, T) {

        let tx1 = (self.blf.x - r.o.x) * r.invd.x;
        let tx2 = (self.trr.x - r.o.x) * r.invd.x;

        let mut tmin = min(tx1, tx2);
        let mut tmax = max(tx1, tx2);

        let ty1 = (self.blf.y - r.o.y) * r.invd.y;
        let ty2 = (self.trr.y - r.o.y) * r.invd.y;

        tmin = max(tmin, min(ty1, ty2));
        tmax = min(tmax, max(ty1, ty2));

        let tz1 = (self.blf.z - r.o.z) * r.invd.z;
        let tz2 = (self.trr.z - r.o.z) * r.invd.z;

        tmin = max(tmin, min(tz1, tz2));
        tmax = min(tmax, max(tz1, tz2));

        let t = tmin;

        return (tmax >= tmin, t);
    }
}
