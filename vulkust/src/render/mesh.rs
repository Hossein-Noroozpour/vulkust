use super::super::core::object::Object as CoreObject;
use super::super::core::types::Id;
use super::buffer::{DynamicBuffer, StaticBuffer};
use super::descriptor::Set as DescriptorSet;
use super::engine::Engine;
use super::gx3d::Table as Gx3dTable;
use super::object::{Base as ObjectBase, Object};
use super::scene::Uniform as SceneUniform;
use super::texture::{Manager as TextureManager, Texture, Texture2D};
use std::collections::BTreeMap;
use super::super::core::debug::Debug;
use std::mem::size_of;
use std::mem::transmute;
use std::sync::{Arc, RwLock, Weak};
// use super::material::Material;

use gltf;
use math;
use math::Matrix;

pub trait Mesh: Object {
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Manager {
    pub meshes: BTreeMap<Id, Weak<RwLock<Mesh>>>,
    pub name_to_id: BTreeMap<String, Id>,
    pub gx3d_table: Option<Gx3dTable>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            meshes: BTreeMap::new(),
            name_to_id: BTreeMap::new(),
            gx3d_table: None,
        }
    }

    pub fn load_gltf(&mut self, primitive: gltf::Primitive, engine: &Engine, data: &[u8]) -> Arc<RwLock<Mesh>> {
        let mesh = Base::new_with_gltf_primitive(primitive, engine, data);
        let id = mesh.get_id();
        let name = mesh.get_name();
        let mesh: Arc<RwLock<Mesh>> = Arc::new(RwLock::new(mesh));
        self.meshes.insert(id, Arc::downgrade(&mesh));
        if let Some(name) = name {
            self.name_to_id.insert(name, id);
        }
        return mesh;
    }

    pub fn create<M>(&mut self, engine: &Arc<RwLock<Engine>>) -> Arc<RwLock<M>>
    where
        M: 'static + DefaultMesh,
    {
        let mesh = M::default(engine);
        let id = mesh.get_id();
        if let Some(name) = mesh.get_name() {
            self.name_to_id.insert(name, id);
        }
        let mesh = Arc::new(RwLock::new(mesh));
        let m: Arc<RwLock<Mesh>> = mesh.clone();
        let m: Weak<RwLock<Mesh>> = Arc::downgrade(&m);
        self.meshes.insert(id, m);
        mesh
    }
}

#[repr(C)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Uniform {
    pub mvp: math::Matrix4<f32>,
}

// impl Geometry {
//     
// }

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Base {
    pub obj_base: ObjectBase,
    
    pub texture: Arc<RwLock<Texture>>,
    pub descriptor_set: Arc<DescriptorSet>,
    pub uniform_buffer: DynamicBuffer, // todo it must move to material
    // pub material: Arc<RwLock<Material>>,

    pub vertex_buffer: StaticBuffer,
    pub index_buffer: StaticBuffer,
    pub indices_count: u32,
}

impl Base {
    pub fn new_with_gltf_primitive(
        primitive: gltf::Primitive,
        engine: &Engine,
        data: &[u8],
    ) -> Self {
        let count = vxunwrap!(primitive.get(&gltf::Semantic::Positions)).count();
            let mut vertex_buffer = vec![0u8; count * 44];
            for (sem, acc) in primitive.attributes() {
                let view = acc.view();
                match acc.data_type() {
                    gltf::accessor::DataType::F32 => {}
                    _ => vxlogf!("Only float data type is acceptable for vertex attributes"),
                }
                let source = view.buffer().source();
                let offset = view.offset();
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
            let indices = vxunwrap!(primitive.indices());
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
            let gapi_engine = vxresult!(engine.gapi_engine.read());
            let vertex_buffer = vxresult!(gapi_engine.buffer_manager.write())
                .create_static_buffer_with_vec(&vertex_buffer);
            let index_buffer = vxresult!(gapi_engine.buffer_manager.write())
                .create_static_buffer_with_vec(&index_buffer);
            let uniform_buffer = vxresult!(gapi_engine.buffer_manager.write())
                .create_dynamic_buffer(size_of::<Uniform>() as isize);
            let texture = vxunwrap!(
                primitive
                    .material()
                    .pbr_metallic_roughness()
                    .base_color_texture()
            ).texture();
            let scene_manager = vxresult!(engine.scene_manager.read());
            let texture_manager = vxresult!(scene_manager.texture_manager.write());
            let texture = texture_manager.load_gltf::<Texture2D>(&texture, engine, data);
            let descriptor_set = Arc::new(
                gapi_engine.create_descriptor_set(&vxresult!(texture.read()).get_image_view()),
            ); // todo
            let obj_base = ObjectBase::new();
            Base {
                obj_base,
                texture,
                descriptor_set,
                uniform_buffer,
                vertex_buffer,
                index_buffer,
                indices_count,
            }
    }

    pub fn new_with_material(
        texture: Arc<RwLock<Texture>>,
        vertices: &[f32],
        indices: &[u32],
        engine: &Engine,
    ) -> Self {
        let gapi_engine = vxresult!(engine.gapi_engine.read());
        let buffer_manager = vxresult!(gapi_engine.buffer_manager.write());
        let vertex_buffer = buffer_manager.create_static_buffer_with_vec(vertices);
        let index_buffer = buffer_manager.create_static_buffer_with_vec(indices);
        let uniform_buffer = buffer_manager.create_dynamic_buffer(size_of::<Uniform>() as isize);
        let descriptor_set = Arc::new(
            gapi_engine.create_descriptor_set(&vxresult!(texture.read()).get_image_view()),
        ); // todo move it to material
        let obj_base = ObjectBase::new();
        Base {
            obj_base,
            texture,
            descriptor_set,
            uniform_buffer,
            vertex_buffer,
            index_buffer,
            indices_count: indices.len() as u32,
        }
    }
}

impl CoreObject for Base {
    fn get_id(&self) -> Id {
        self.obj_base.get_id()
    }
}

impl Object for Base {
    fn get_name(&self) -> Option<String> {
        self.obj_base.get_name()
    }

    fn set_name(&mut self, name: &str) {
        self.obj_base.set_name(name);
        vxunimplemented!(); //it must update corresponding manager
    }

    fn render(&self, engine: &Engine) {
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
//     fn render(&mut self, scene_uniform: &SceneUniform) {
//         let mvp = scene_uniform.view_projection;
//         for geo in &mut self.geometries {
//             geo.uniform_buffer
//                 .update(unsafe { transmute(mvp.as_ptr()) });
//             let eng = vxresult!(self.engine.read());
//             let eng = vxresult!(eng.gapi_engine.read());
//             eng.render_main_pipeline(
//                 &geo.descriptor_set,
//                 &geo.uniform_buffer,
//                 &geo.vertex_buffer,
//                 &geo.index_buffer,
//                 geo.indices_count,
//             );
//         }
//     }
}