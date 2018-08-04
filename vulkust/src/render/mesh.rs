use super::super::core::debug::Debug;
use super::super::core::object::Object as CoreObject;
use super::super::core::types::Id;
use super::buffer::{DynamicBuffer, StaticBuffer};
use super::descriptor::Set as DescriptorSet;
use super::engine::Engine;
use super::material::Material;
use super::gx3d::{Gx3DReader, Table as Gx3dTable};
use super::object::{Base as ObjectBase, Object};
use super::scene::Uniform as SceneUniform;
use super::texture::{Manager as TextureManager, Texture, Texture2D};
use std::collections::BTreeMap;
use std::mem::size_of;
use std::mem::transmute;
use std::sync::{Arc, RwLock, Weak};
// use super::material::Material;

use gltf;
use math;
use math::Matrix;

#[repr(u8)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum TypeId {
    Base = 1,
}

pub trait Mesh: Object {
    fn is_shadow_caster(&self) -> bool;
    fn is_transparent(&self) -> bool;
    fn get_occlusion_culling_radius(&self) -> f32;
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

    pub fn load_gltf(
        &mut self,
        primitive: &gltf::Primitive,
        engine: &Engine,
        data: &[u8],
    ) -> Arc<RwLock<Mesh>> {
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

    pub fn load_gx3d(&mut self, engine: &Arc<RwLock<Engine>>, id: Id) -> Arc<RwLock<Mesh>> {
        if let Some(mesh) = self.meshes.get(&id) {
            if let Some(mesh) = mesh.upgrade() {
                return mesh;
            }
        }
        let gx3d_table = vxunwrap!(self.gx3d_table.as_mut());
        gx3d_table.goto(id);
        let reader = &mut gx3d_table.reader;
        let t = reader.read_type_id();
        let mesh: Arc<RwLock<Mesh>> = if t == TypeId::Base as u8 {
            Arc::new(RwLock::new(Base::new_with_gx3d(engine, reader, id)))
        } else {
            vxunimplemented!();
        };
        self.meshes.insert(id, Arc::downgrade(&mesh));
        return mesh;
    }
}

#[repr(C)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Uniform {
    pub mvp: math::Matrix4<f32>,
}


#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Base {
    pub obj_base: ObjectBase,
    pub material: Material,
    pub vertex_buffer: StaticBuffer,
    pub index_buffer: StaticBuffer,
    pub indices_count: u32,
}

impl Base {
    pub fn new_with_gltf_primitive(
        primitive: &gltf::Primitive,
        engine: &Engine,
        data: &[u8],
    ) -> Self {
        let material = Material::new_with_gltf(engine, &primitive.material());
        let count = vxunwrap!(primitive.get(&gltf::Semantic::Positions)).count();
        let mut vertex_buffer = vec![0u8; count * size_of::<f32>() * 12];
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
                        buffer_index += 36; // 48 - 12
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
                        buffer_index += 36; // 48 - 12
                    }
                }
                gltf::Semantic::Tangents => {
                    let mut buffer_index = 24; // previous ending index
                    let mut data_index = offset;
                    for _ in 0..count {
                        for _ in 0..16 {
                            vertex_buffer[buffer_index] = data[data_index];
                            buffer_index += 1;
                            data_index += 1;
                        }
                        buffer_index += 32; // 48 - 16
                    }
                }
                gltf::Semantic::TexCoords(uv_count) => {
                    if uv_count > 0 {
                        vxlogf!("UV index must be zero.");
                    }
                    let mut buffer_index = 40; // previous ending index
                    let mut data_index = offset;
                    for _ in 0..count {
                        for _ in 0..8 {
                            vertex_buffer[buffer_index] = data[data_index];
                            buffer_index += 1;
                            data_index += 1;
                        }
                        buffer_index += 40; // 48 - 8
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
        let index_buffer = &data[offset..end];
        let indices_count = indices_count as u32;
        let gapi_engine = vxresult!(engine.gapi_engine.read());
        let vertex_buffer = vxresult!(gapi_engine.buffer_manager.write())
            .create_static_buffer_with_vec(&vertex_buffer);
        let index_buffer = vxresult!(gapi_engine.buffer_manager.write())
            .create_static_buffer_with_vec(&index_buffer);
        let obj_base = ObjectBase::new();
        Base {
            obj_base,
            material,
            vertex_buffer,
            index_buffer,
            indices_count,
        }
    }

    pub fn new_with_material(
        material: Material,
        vertices: &[f32],
        indices: &[u32],
        engine: &Engine,
    ) -> Self {
        let gapi_engine = vxresult!(engine.gapi_engine.read());
        let mut buffer_manager = vxresult!(gapi_engine.buffer_manager.write());
        let vertex_buffer = buffer_manager.create_static_buffer_with_vec(vertices);
        let index_buffer = buffer_manager.create_static_buffer_with_vec(indices);
        let uniform_buffer = buffer_manager.create_dynamic_buffer(size_of::<Uniform>() as isize);
        let obj_base = ObjectBase::new();
        Base {
            obj_base,
            material,
            vertex_buffer,
            index_buffer,
            indices_count: indices.len() as u32,
        }
    }

    pub fn new_with_gx3d(engine: &Arc<RwLock<Engine>>, reader: &mut Gx3DReader, my_id: Id) -> Self {
        let number_of_vertex_attribute = reader.read_u8() as usize;
        #[cfg(debug_assertions)]
        {
            if number_of_vertex_attribute != 12 {
                vxunexpected!();
            }
        }
        let vertex_count = reader.read::<u64>() as usize;
        #[cfg(debug_gx3d)]
        vxlogi!("Number of vertices is: {}", vertex_count);
        let number_of_floats = vertex_count * number_of_vertex_attribute;
        let mut vertices = vec![0f32; number_of_floats];
        for i in 0..number_of_floats {
            vertices[i] = reader.read();
        }
        let indices = reader.read_array::<u32>();
        let material = Material::new_with_gx3d(engine, reader);
        let obj_base = ObjectBase::new_with_id(my_id);
        let engine = vxresult!(engine.read());
        let gapi_engine = vxresult!(engine.gapi_engine.read());
        let mut buffer_manager = vxresult!(gapi_engine.buffer_manager.write());
        let vertex_buffer = buffer_manager.create_static_buffer_with_vec(&vertices);
        let index_buffer = buffer_manager.create_static_buffer_with_vec(&indices);
        let indices_count = indices.len() as u32;
        #[cfg(debug_gx3d)]
        vxlogi!("Number of indices is: {}", indices_count);
        Base {
            obj_base,
            material,
            vertex_buffer,
            index_buffer,
            indices_count,
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

    fn render(&self, engine: &Engine) {}

    fn update(&mut self) {}

    fn disable_rendering(&mut self) {
        self.obj_base.disable_rendering()
    }

    fn enable_rendering(&mut self) {
        self.obj_base.enable_rendering()
    }
}

impl Mesh for Base {
    fn is_shadow_caster(&self) -> bool {
        true
        // todo
    }

    fn is_transparent(&self) -> bool {
        false
        // todo
    }
    
    fn get_occlusion_culling_radius(&self) -> f32 {
        vxunimplemented!()
    } // todo

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
