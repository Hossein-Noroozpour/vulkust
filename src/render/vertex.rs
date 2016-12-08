use super::super::math::vector::{
    Vec3,
    Vec2,
};

pub trait Vertex {

}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Pos {
    pub p: Vec3,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PosNrm {
    pub p: Vec3,
    pub n: Vec3,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PosNrmTxc2 {
    pub p: Vec3,
    pub n: Vec3,
    pub t: Vec2,
}