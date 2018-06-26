use std::sync::{Arc, RwLock};
use std::mem::transmute;
use super::super::system::file::File;
use super::buffer::{
    Manager as BufferManager,
    StaticBuffer,
};
use super::engine::GraphicApiEngine;
use super::material::Material;

use gltf;

pub struct Mesh {
    pub vertices: StaticBuffer,
    pub indices: StaticBuffer,
    pub material: Arc<RwLock<Material>>,
}

impl Mesh {
    pub fn new(
        file: &Arc<RwLock<File>>,
        gapi_engine: &Arc<RwLock<GraphicApiEngine>>,
        gltf_mesh: gltf::Mesh,
    ) -> Self {
        let name = vxunwrap_o!(gltf_mesh.name()).to_string();
        let primitives = gltf_mesh.primitives();
        for primitive in primitives {
            fsfdgdfgsdfgdfgdgasdgsdfgsdg
        }
        Mesh {
            buffer: buffer,
            buffer_manager: buffer_manager,
            material: material,
        }
    }
}
