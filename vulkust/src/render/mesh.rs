use super::buffer::{DynamicBuffer, StaticBuffer};
use super::engine::GraphicApiEngine;
use super::object::Object;
use super::scene::Uniform as SceneUniform;
use std::mem::size_of;
use std::sync::{Arc, RwLock};
// use super::material::Material;

use gltf;
use math;

pub trait Mesh: Object {
    fn render(&self, scene_uniform: &SceneUniform) {
        unimplemented!();
    }
}

#[repr(C)]
pub struct Uniform {
    pub mvp: math::Matrix4<f32>,
}

pub struct Geometry {
    pub uniform_buffer: DynamicBuffer, // todo it must move to material
    pub vertex_buffer: StaticBuffer,
    pub index_buffer: StaticBuffer,
    pub indices_count: u32,
}

pub struct Basic {
    pub name: String,
    pub geometries: Vec<Geometry>,
    // pub material: Arc<RwLock<Material>>,
}

impl Basic {
    pub fn new_with_gltf(
        gapi_engine: &Arc<RwLock<GraphicApiEngine>>,
        mesh: gltf::Mesh,
        data: &Vec<u8>,
    ) -> Self {
        let name = vxunwrap_o!(mesh.name()).to_string();
        let primitives = mesh.primitives();
        let mut geometries = Vec::new();
        for primitive in primitives {
            let count = vxunwrap_o!(primitive.get(&gltf::Semantic::Positions)).count();
            // vxlogi!("count: {}", count);
            let mut vertex_buffer = vec![0u8; count * 44];
            for (sem, acc) in primitive.attributes() {
                let view = acc.view();
                match acc.data_type() {
                    gltf::accessor::DataType::F32 => {}
                    _ => vxlogf!("Only float data type is acceptable for vertex attributes"),
                }
                let source = view.buffer().source();
                let offset = view.offset();
                // let length = view.length();
                // vxlogi!("{:?} o-{} l-{} a-{} s-{:?}", sem, offset, length, acc.offset(), source);
                match source {
                    gltf::buffer::Source::Bin => (),
                    _ => vxlogf!("Buffer source must be binary."),
                }
                if view.stride() != None {
                    // Its meaning is not clear yet
                    vxlogf!("Stride is unexpectable.");
                }
                match sem {
                    gltf::Semantic::Positions => {
                        let mut buffer_index = 0;
                        let mut data_index = offset;
                        for _ in 0..count {
                            for _ in 0..12 {
                                vertex_buffer[buffer_index] = data[data_index];
                                buffer_index += 1;
                                data_index += 1;
                            }
                            buffer_index += 32; // 44 - 12
                        }
                    }
                    gltf::Semantic::Normals => {
                        let mut buffer_index = 12; // previous ending index
                        let mut data_index = offset;
                        for _ in 0..count {
                            for _ in 0..12 {
                                vertex_buffer[buffer_index] = data[data_index];
                                buffer_index += 1;
                                data_index += 1;
                            }
                            buffer_index += 32; // 44 - 12
                        }
                    }
                    gltf::Semantic::Tangents => {
                        let mut buffer_index = 24; // previous ending index
                        let mut data_index = offset;
                        for _ in 0..count {
                            for _ in 0..12 {
                                vertex_buffer[buffer_index] = data[data_index];
                                buffer_index += 1;
                                data_index += 1;
                            }
                            data_index += 4;
                            buffer_index += 32; // 44 - 12
                        }
                    }
                    gltf::Semantic::TexCoords(uv_count) => {
                        if uv_count > 0 {
                            vxlogf!("UV index must be zero.");
                        }
                        let mut buffer_index = 36; // previous ending index
                        let mut data_index = offset;
                        for _ in 0..count {
                            for _ in 0..8 {
                                vertex_buffer[buffer_index] = data[data_index];
                                buffer_index += 1;
                                data_index += 1;
                            }
                            buffer_index += 36; // 44 - 8
                        }
                    }
                    _ => {}
                }
            }
            let indices = vxunwrap_o!(primitive.indices());
            match indices.data_type() {
                gltf::accessor::DataType::U32 => {}
                _ => vxlogf!("Only u32 data type is acceptable for indices."),
            }
            let view = indices.view();
            let indices_count = indices.count();
            let offset = view.offset();
            let end = view.length() + offset;
            let index_buffer = data[offset..end].to_vec();
            // let v: Vec<f32> = unsafe {
            //     let len = 11 * count;
            //     Vec::from_raw_parts(transmute(vertex_buffer.as_mut_ptr()), len, len)
            // };
            // vxlogi!("{:?}", &v);
            let indices_count = indices_count as u32;
            let gapi_engine = vxresult!(gapi_engine.read());
            let vertex_buffer = vxresult!(gapi_engine.buffer_manager.write())
                .create_static_buffer_with_vec(&vertex_buffer);
            let index_buffer = vxresult!(gapi_engine.buffer_manager.write())
                .create_static_buffer_with_vec(&index_buffer);
            let uniform_buffer = vxresult!(gapi_engine.buffer_manager.write())
                .create_dynamic_buffer(size_of::<Uniform>() as isize);
            geometries.push(Geometry {
                uniform_buffer,
                vertex_buffer,
                index_buffer,
                indices_count,
            });
        }
        geometries.shrink_to_fit();
        Basic {
            name,
            geometries,
            // buffer: buffer,
            // buffer_manager: buffer_manager,
            // material: material,
        }
    }
}

impl Object for Basic {}

impl Mesh for Basic {
    fn render(&self, scene_uniform: &SceneUniform) {
        let mvp = scene_uniform.vp;
        // update uniform
        // bind des
        // bind pipe
        // bind buff
        // bind index
        // draw
    }
}
