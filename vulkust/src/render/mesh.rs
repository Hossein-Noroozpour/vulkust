use std::sync::{Arc, RwLock};
use std::mem::transmute;
use super::super::system::file::File;
use super::buffer::{
    Manager as BufferManager,
    StaticBuffer,
};
use super::engine::GraphicApiEngine;
// use super::material::Material;

use gltf;

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
            // let mut vertex_buffer = Vec::new();
            let accessor = vxunwrap_o!(primitive.get(&gltf::Semantic::Positions));
            let view = accessor.view();
            let source = view.buffer().source();
            match source {
                gltf::buffer::Source::Bin => (),
                _ => vxlogf!("Buffer source must be binary."),
            }
            let offset = view.offset();
            let length = view.length() / 4;
            if view.stride() != None {
                vxlogf!("Stride is unexpectable.");
            }
            let v: Vec<f32> = unsafe { Vec::from_raw_parts(transmute(data.as_ptr().offset(offset as isize)), length, length) };
            vxlogi!("Vec: {:?}", &v);
            // for i in 0..length {
            //     let modi = i % 12;
            //     let i = (i / 12) * 
            // }
        }
        Mesh {
            // buffer: buffer,
            // buffer_manager: buffer_manager,
            // material: material,
        }
    }
}
