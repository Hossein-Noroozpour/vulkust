use std::sync::{Arc, RwLock};
use std::mem::transmute;
use std::f32::MAX as F32MAX;
use super::super::system::file::File;
use super::buffer::{
    Manager as BufferManager,
    StaticBuffer,
};
use super::engine::GraphicApiEngine;
// use super::material::Material;

use gltf;
use libc;

pub struct Mesh {
    // pub vertices: StaticBuffer,
    // pub indices: StaticBuffer,
    // pub material: Arc<RwLock<Material>>,
}

impl Mesh {
    pub fn new_with_gltf(
        gapi_engine: &Arc<RwLock<GraphicApiEngine>>,
        mesh: gltf::Mesh,
        data: &Vec<u8>,
    ) -> Self {
        let name = vxunwrap_o!(mesh.name()).to_string();
        let primitives = mesh.primitives();
        // let mut vertex_buffers = Vec::new();
        for primitive in primitives {
            let accessor = vxunwrap_o!(primitive.get(&gltf::Semantic::Positions));
            let count = accessor.count();
            let mut vertex_buffer = vec![0u8; (count / 3) * 44];
            let fun = |sem : &gltf::Semantic| {
                let accessor = primitive.get(&gltf::Semantic::Positions);
                let accessor = vxunwrap_o!(accessor);
                let view = accessor.view();
                let source = view.buffer().source();
                match source {
                    gltf::buffer::Source::Bin => (),
                    _ => vxlogf!("Buffer source must be binary."),
                }
                let offset = view.offset();
                let length = view.length();
                if view.stride() != None { // Its meaning is not clear
                    vxlogf!("Stride is unexpectable.");
                }
                data[offset..(length + offset)]
            };
            let pos = fun(&gltf::Semantic::Positions);
            let pos = fun(&gltf::Semantic::Normals);
            let pos = fun(&gltf::Semantic::Positions);
            let pos = fun(&gltf::Semantic::Positions);
            let mut srci = offset;
            let mut dsti = 0;
            for c in 0..count {
                let si = i + offset;
                let i = (i / 12) * 44 + (i % 12);

            }
        }
        Mesh {
            // buffer: buffer,
            // buffer_manager: buffer_manager,
            // material: material,
        }
    }
}
