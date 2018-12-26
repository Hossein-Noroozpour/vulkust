use super::super::core::gx3d::{Gx3DReader, Table as Gx3dTable};
use super::super::core::object::Object as CoreObject;
use super::super::core::types::{Id, Real};
use super::buffer::Static as StaticBuffer;
use super::command::Buffer as CmdBuffer;
use super::engine::Engine;
use super::model::Model;
use super::object::{Base as ObjectBase, Object};
use super::scene::Scene;
use std::collections::BTreeMap;
use std::mem::size_of;
use std::sync::{Arc, RwLock, Weak};

use cgmath;
use cgmath::prelude::*;
use gltf;

#[repr(u8)]
#[cfg_attr(debug_mode, derive(Debug))]
pub enum TypeId {
    Base = 1,
}

pub trait Mesh: Object {
    fn is_shadow_caster(&self) -> bool;
    fn is_transparent(&self) -> bool;
    fn get_occlusion_culling_radius(&self) -> Real;
    fn update(&mut self, usize);
    fn render_gbuffer(&self, &mut CmdBuffer, usize);
    fn render_shadow(&self, &mut CmdBuffer, usize);
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Manager {
    engine: Option<Weak<RwLock<Engine>>>,
    meshes: BTreeMap<Id, Weak<RwLock<Mesh>>>,
    name_to_id: BTreeMap<String, Id>,
    gx3d_table: Option<Gx3dTable>,
}

impl Manager {
    pub(crate) fn new() -> Self {
        Manager {
            engine: None,
            meshes: BTreeMap::new(),
            name_to_id: BTreeMap::new(),
            gx3d_table: None,
        }
    }

    pub(crate) fn set_gx3d_table(&mut self, gx3d_table: Gx3dTable) {
        self.gx3d_table = Some(gx3d_table);
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

    pub fn load_gx3d(&mut self, engine: &Engine, id: Id) -> Arc<RwLock<Mesh>> {
        if let Some(mesh) = self.meshes.get(&id) {
            if let Some(mesh) = mesh.upgrade() {
                return mesh;
            }
        }
        let gx3d_table = vxunwrap!(self.gx3d_table.as_mut());
        gx3d_table.goto(id);
        let reader = gx3d_table.get_mut_reader();
        let t = reader.read_type_id();
        let mesh: Arc<RwLock<Mesh>> = if t == TypeId::Base as u8 {
            Arc::new(RwLock::new(Base::new_with_gx3d(engine, reader, id)))
        } else {
            vxunimplemented!();
        };
        self.meshes.insert(id, Arc::downgrade(&mesh));
        return mesh;
    }

    pub fn add(&mut self, mesh: &Arc<RwLock<Mesh>>) {
        let id = vxresult!(mesh.read()).get_id();
        self.meshes.insert(id, Arc::downgrade(&mesh));
    }

    pub fn create(
        &mut self,
        vertices: &[Real],
        indices: &[u32],
        occlusion_culling_radius: Real,
        engine: &Engine,
    ) -> Arc<RwLock<Mesh>> {
        let mesh = Base::new(vertices, indices, occlusion_culling_radius, engine);
        let mesh_id = mesh.get_id();
        let mesh: Arc<RwLock<Mesh>> = Arc::new(RwLock::new(mesh));
        self.meshes.insert(mesh_id, Arc::downgrade(&mesh));
        return mesh;
    }

    pub fn create_cube(&mut self, aspect: Real) -> Arc<RwLock<Mesh>> {
        let eng = vxunwrap!(vxunwrap!(&self.engine).upgrade());
        let eng = vxresult!(eng.read());
        let vertices = [
            -aspect, -aspect, aspect, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0,
            // vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
            aspect, -aspect, aspect, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0,
            // vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
            -aspect, aspect, aspect, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0,
            // vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
            aspect, aspect, aspect, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0,
            //----------------------------------------------------------------------------------
            -aspect, -aspect, -aspect, 0.0, 0.0, -1.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0,
            // vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
            aspect, -aspect, -aspect, 0.0, 0.0, -1.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0,
            // vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
            -aspect, aspect, -aspect, 0.0, 0.0, -1.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0,
            // vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
            aspect, aspect, -aspect, 0.0, 0.0, -1.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0,
            //----------------------------------------------------------------------------------
            aspect, -aspect, -aspect, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0,
            // vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
            aspect, aspect, -aspect, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0,
            // vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
            aspect, -aspect, aspect, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0,
            // vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
            aspect, aspect, aspect, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0,
            //----------------------------------------------------------------------------------
            -aspect, -aspect, -aspect, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0,
            // vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
            -aspect, aspect, -aspect, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0,
            // vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
            -aspect, -aspect, aspect, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0,
            // vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
            -aspect, aspect, aspect, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0,
            //----------------------------------------------------------------------------------
            -aspect, aspect, -aspect, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0,
            // vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
            aspect, aspect, -aspect, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0,
            // vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
            -aspect, aspect, aspect, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0,
            // vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
            aspect, aspect, aspect, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0,
            //----------------------------------------------------------------------------------
            -aspect, -aspect, -aspect, 0.0, -1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0,
            // vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
            aspect, -aspect, -aspect, 0.0, -1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0,
            // vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
            -aspect, -aspect, aspect, 0.0, -1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0,
            // vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
            aspect, -aspect, aspect, 0.0, -1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0,
            0.0,
            //----------------------------------------------------------------------------------
        ];
        let indices = [
            0, 1, 2, 1, 3, 2, 4, 6, 5, 5, 6, 7, 8, 9, 10, 9, 11, 10, 12, 14, 13, 13, 14, 15, 16,
            18, 17, 17, 18, 19, 20, 21, 22, 21, 23, 22,
        ];
        self.create(&vertices, &indices, aspect.abs() * 1.732050809, &*eng)
    }

    pub fn set_engine(&mut self, e: Weak<RwLock<Engine>>) {
        self.engine = Some(e);
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Base {
    obj_base: ObjectBase,
    vertex_buffer: StaticBuffer,
    index_buffer: StaticBuffer,
    indices_count: u32,
    occlusion_culling_radius: Real,
}

impl Base {
    pub fn new_with_gltf_primitive(
        primitive: &gltf::Primitive,
        engine: &Engine,
        data: &[u8],
    ) -> Self {
        let count = vxunwrap!(primitive.get(&gltf::Semantic::Positions)).count();
        let mut vertex_buffer = vec![0u8; count * size_of::<Real>() * 12];
        let occlusion_culling_radius = {
            let mut center = cgmath::Vector3::new(0.0, 0.0, 0.0);
            let p1 = &primitive.bounding_box().max;
            let p1 = cgmath::Vector3::new(p1[0], p1[1], p1[2]);
            center += p1;
            let p2 = &primitive.bounding_box().max;
            center += cgmath::Vector3::new(p2[0], p2[1], p2[2]);
            center *= 0.5;
            p1.distance(center)
        };
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
        let gapi_engine = vxresult!(engine.get_gapi_engine().read());
        let vertex_buffer = vxresult!(gapi_engine.get_buffer_manager().write())
            .create_static_buffer_with_vec(&vertex_buffer);
        let index_buffer = vxresult!(gapi_engine.get_buffer_manager().write())
            .create_static_buffer_with_vec(&index_buffer);
        let obj_base = ObjectBase::new();
        Base {
            obj_base,
            vertex_buffer,
            index_buffer,
            indices_count,
            occlusion_culling_radius,
        }
    }

    pub fn new(
        vertices: &[Real],
        indices: &[u32],
        occlusion_culling_radius: Real,
        engine: &Engine,
    ) -> Self {
        let gapi_engine = vxresult!(engine.get_gapi_engine().read());
        let mut buffer_manager = vxresult!(gapi_engine.get_buffer_manager().write());
        let vertex_buffer = buffer_manager.create_static_buffer_with_vec(vertices);
        let index_buffer = buffer_manager.create_static_buffer_with_vec(indices);
        let obj_base = ObjectBase::new();
        Base {
            obj_base,
            vertex_buffer,
            index_buffer,
            indices_count: indices.len() as u32,
            occlusion_culling_radius,
        }
    }

    pub fn new_with_gx3d(engine: &Engine, reader: &mut Gx3DReader, my_id: Id) -> Self {
        let number_of_vertex_attribute = reader.read_u8() as usize;
        #[cfg(debug_mode)]
        {
            if number_of_vertex_attribute != 12 {
                vxunexpected!();
            }
        }
        let vertex_count = reader.read::<u64>() as usize;
        #[cfg(debug_gx3d)]
        vxlogi!("Number of vertices is: {}", vertex_count);
        let number_of_floats = vertex_count * number_of_vertex_attribute;
        let mut vertices = vec![0.0; number_of_floats];
        for i in 0..number_of_floats {
            vertices[i] = reader.read();
        }
        let indices = reader.read_array::<u32>();
        let occlusion_culling_radius = reader.read();
        let obj_base = ObjectBase::new_with_id(my_id);
        let gapi_engine = vxresult!(engine.get_gapi_engine().read());
        let mut buffer_manager = vxresult!(gapi_engine.get_buffer_manager().write());
        let vertex_buffer = buffer_manager.create_static_buffer_with_vec(&vertices);
        let index_buffer = buffer_manager.create_static_buffer_with_vec(&indices);
        let indices_count = indices.len() as u32;
        #[cfg(debug_gx3d)]
        vxlogi!("Number of indices is: {}", indices_count);
        #[cfg(debug_gx3d)]
        vxlogi!("Occlusion culling radius is: {}", occlusion_culling_radius);
        Base {
            obj_base,
            vertex_buffer,
            index_buffer,
            indices_count,
            occlusion_culling_radius,
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

    fn disable_rendering(&mut self) {
        self.obj_base.disable_rendering()
    }

    fn enable_rendering(&mut self) {
        self.obj_base.enable_rendering()
    }

    fn is_renderable(&self) -> bool {
        return self.obj_base.is_renderable();
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

    fn get_occlusion_culling_radius(&self) -> Real {
        return self.occlusion_culling_radius;
    }

    fn update(&mut self, _frame_number: usize) {}

    fn render_gbuffer(&self, cmd: &mut CmdBuffer, frame_number: usize) {
        cmd.render_gbuff(&self.vertex_buffer, &self.index_buffer, self.indices_count);
    }

    fn render_shadow(&self, cmd: &mut CmdBuffer, frame_number: usize) {
        cmd.render_shadow_mapper(&self.vertex_buffer, &self.index_buffer, self.indices_count);
    }
}
