use super::super::math::vector::{
    Vec3,
    Vec2,
};

pub trait Vertex {

}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Pos {
    pub p: [f32; 3],
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PosNrm {
    pub p: [f32; 3],
    pub n: [f32; 3],
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PosNrmTxc2 {
    pub p: [f32; 3],
    pub n: [f32; 3],
    pub t: [f32; 2],
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PosTxc2 {
    pub p: [f32; 3],
    pub t: [f32; 2],
}
