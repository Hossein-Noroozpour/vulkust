use super::super::core::debug::Debug;
use super::super::core::gx3d::Gx3DReader;
use std::sync::{Arc, RwLock};

use math;

pub trait Collider: Debug {}

#[cfg_attr(debug_mode, derive(Debug))]
#[repr(u8)]
pub enum TypeId {
    Mesh = 2,
    Ghost = 1,
}

pub fn read(reader: &mut Gx3DReader) -> Arc<RwLock<Collider>> {
    let t = reader.read_type_id();
    if t == TypeId::Ghost as u8 {
        return Arc::new(RwLock::new(Ghost::new()));
    } else if t == TypeId::Mesh as u8 {
        return Arc::new(RwLock::new(Mesh::new_with_gx3d(reader)));
    } else {
        vxunexpected!();
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Mesh {
    pub vertices: Vec<math::Vector3<f32>>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn new_with_gx3d(reader: &mut Gx3DReader) -> Self {
        let count: u64 = reader.read();
        let mut vertices = vec![math::Vector3::new(0.0, 0.0, 0.0); count as usize];
        for i in 0..count as usize {
            vertices[i] = math::Vector3::new(reader.read(), reader.read(), reader.read());
        }
        let indices = reader.read_array();
        Mesh { vertices, indices }
    }
}

impl Collider for Mesh {}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Ghost {}

impl Ghost {
    pub fn new() -> Self {
        Ghost {}
    }
}

impl Collider for Ghost {}
