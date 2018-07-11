use super::super::core::object::Object as CoreObject;
use super::super::core::types::Id;
use super::buffer::{DynamicBuffer, StaticBuffer};
use super::descriptor::Set as DescriptorSet;
use super::engine::GraphicApiEngine;
use super::object::{Base as ObjectBase, Object};
use super::scene::Uniform as SceneUniform;
use super::texture::{Manager as TextureManager, Texture, Texture2D};
use std::mem::size_of;
use std::mem::transmute;
use std::sync::{Arc, RwLock, Weak};
use std::collections::BTreeMap;
// use super::material::Material;

use gltf;
use math;
use math::Matrix;

pub trait Mesh: Object {
    fn render(&mut self, _: &SceneUniform) {
        unimplemented!();
    }
}

pub trait DefaultMesh: Mesh {
    fn default(Arc<RwLock<GraphicApiEngine>>) -> Self;
}

pub struct Manager {
    pub gapi_engine: Arc<RwLock<GraphicApiEngine>>,
    pub meshes: BTreeMap<Id, Weak<RwLock<Mesh>>>,
}

impl Manager {
    pub fn new(gapi_engine: &Arc<RwLock<GraphicApiEngine>>) -> Self {
        Manager {
            gapi_engine: gapi_engine.clone(),
            meshes: BTreeMap::new(),
        }
    }

    pub fn create<M>(&mut self) -> Arc<RwLock<M>> where M: 'static + DefaultMesh {
        let mesh = M::default(self.gapi_engine.clone());
        let id = mesh.get_id();
        let mesh = Arc::new(RwLock::new(mesh));
        let m: Arc<RwLock<Mesh>> = mesh.clone();
        let m: Weak<RwLock<Mesh>> = Arc::downgrade(&m);
        self.meshes.insert(id, m);
        mesh
    }
}

#[repr(C)]
pub struct Uniform {
    pub mvp: math::Matrix4<f32>,
}

pub struct Geometry {
    pub texture: Arc<RwLock<Texture>>,
    pub descriptor_set: Arc<DescriptorSet>,
    pub uniform_buffer: DynamicBuffer, // todo it must move to material
    pub vertex_buffer: StaticBuffer,
    pub index_buffer: StaticBuffer,
    pub indices_count: u32,
}

pub struct Base {
    pub obj_base: ObjectBase,
    pub gapi_engine: Arc<RwLock<GraphicApiEngine>>,
    pub geometries: Vec<Geometry>,
    // pub material: Arc<RwLock<Material>>,
}

impl Base {
    pub fn new_with_gltf(
        gapi_engine: &Arc<RwLock<GraphicApiEngine>>,
        mesh: gltf::Mesh,
        texture_manager: &Arc<RwLock<TextureManager>>,
        data: &Vec<u8>,
    ) -> Self {
        let gapi_engine_clone = gapi_engine.clone();
        let obj_base = ObjectBase::new();
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
            let texture = vxunwrap_o!(
                primitive
                    .material()
                    .pbr_metallic_roughness()
                    .base_color_texture()
            ).texture();
            let texture =
                vxresult!(texture_manager.write()).get_with_gltf::<Texture2D>(&texture, data);
            let descriptor_set = Arc::new(
                gapi_engine.create_descriptor_set(&vxresult!(texture.read()).get_image_view()),
            ); // todo
            geometries.push(Geometry {
                texture,
                descriptor_set,
                uniform_buffer,
                vertex_buffer,
                index_buffer,
                indices_count,
            });
        }
        geometries.shrink_to_fit();
        Base {
            obj_base,
            geometries,
            gapi_engine: gapi_engine_clone,
            // buffer: buffer,
            // buffer_manager: buffer_manager,
            // material: material,
        }
    }
}

impl CoreObject for Base {
    fn get_id(&self) -> Id {
        self.obj_base.get_id()
    }
}

impl Object for Base {
    fn render(&self) {
        vxlogf!("Mesh does not implement this function instead it does the Mesh trait render.");
    }

    fn update(&mut self) {}

    fn disable_rendering(&mut self) {
        self.obj_base.disable_rendering()
    }

    fn enable_rendering(&mut self) {
        self.obj_base.enable_rendering()
    }
}

impl Mesh for Base {
    fn render(&mut self, scene_uniform: &SceneUniform) {
        let mvp = scene_uniform.vp;
        for geo in &mut self.geometries {
            geo.uniform_buffer
                .update(unsafe { transmute(mvp.as_ptr()) });
            vxresult!(self.gapi_engine.read()).render_main_pipeline(
                &geo.descriptor_set,
                &geo.uniform_buffer,
                &geo.vertex_buffer,
                &geo.index_buffer,
                geo.indices_count,
            );
        }
    }
}
