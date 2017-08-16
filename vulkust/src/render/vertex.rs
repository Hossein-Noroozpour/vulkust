pub trait Vertex {}

#[derive(Clone)]
pub enum Attribute {
    Vec3F32,
    Vec2F32,
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
