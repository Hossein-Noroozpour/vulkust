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
use std::mem::size_of;
use std::mem::transmute;
use std::sync::{Arc, RwLock, Weak};
// use super::material::Material;

use gltf;
use math;
use math::Matrix;

pub trait Mesh: Object {
    fn render(&mut self, _: &SceneUniform); // todo scene uniform is gonna move and this function is in the object
}

pub trait DefaultMesh: Mesh {
    fn default(&Arc<RwLock<Engine>>) -> Self;
}

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

impl Geometry {
    pub fn new(
        texture: Arc<RwLock<Texture>>,
        vertices: &[f32],
        indices: &[u32],
        engine: &Arc<RwLock<Engine>>,
    ) -> Self {
        let eng = vxresult!(engine.read());
        let gapi_engine = vxresult!(eng.gapi_engine.read());
        let vertex_buffer =
            vxresult!(gapi_engine.buffer_manager.write()).create_static_buffer_with_vec(vertices);
        let index_buffer =
            vxresult!(gapi_engine.buffer_manager.write()).create_static_buffer_with_vec(indices);
        vxlogi!("dfdfsdfsdfsdfsdfsdfsdf {}", size_of::<Uniform>());
        let uniform_buffer = vxresult!(gapi_engine.buffer_manager.write())
            .create_dynamic_buffer(size_of::<Uniform>() as isize);
        let descriptor_set = Arc::new(
            gapi_engine.create_descriptor_set(&vxresult!(texture.read()).get_image_view()),
        ); // todo
        Geometry {
            texture,
            descriptor_set,
            uniform_buffer,
            vertex_buffer,
            index_buffer,
            indices_count: indices.len() as u32,
        }
    }
}

pub struct Base {
    pub obj_base: ObjectBase,
    pub engine: Arc<RwLock<Engine>>,
    pub geometries: Vec<Geometry>,
    // pub material: Arc<RwLock<Material>>,
}

impl Base {
    pub fn new_with_gltf(
        engine: &Arc<RwLock<Engine>>,
        mesh: gltf::Mesh,
        texture_manager: &Arc<RwLock<TextureManager>>,
        data: &[u8],
    ) -> Self {
        let engine_clone = engine.clone();
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
            let eng = vxresult!(engine.read());
            let gapi_engine = vxresult!(eng.gapi_engine.read());
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
                vxresult!(texture_manager.write()).load_gltf::<Texture2D>(&texture, engine, data);
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
            engine: engine_clone,
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
    fn get_name(&self) -> Option<String> {
        self.obj_base.get_name()
    }

    fn set_name(&mut self, name: &str) {
        self.obj_base.set_name(name);
        vxunimplemented!(); //it must update corresponding manager
    }

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
        let mvp = scene_uniform.view_projection;
        for geo in &mut self.geometries {
            geo.uniform_buffer
                .update(unsafe { transmute(mvp.as_ptr()) });
            let eng = vxresult!(self.engine.read());
            let eng = vxresult!(eng.gapi_engine.read());
            eng.render_main_pipeline(
                &geo.descriptor_set,
                &geo.uniform_buffer,
                &geo.vertex_buffer,
                &geo.index_buffer,
                geo.indices_count,
            );
        }
    }
}

impl DefaultMesh for Base {
    fn default(engine: &Arc<RwLock<Engine>>) -> Self {
        Base {
            obj_base: ObjectBase::new(),
            engine: engine.clone(),
            geometries: Vec::new(),
        }
    }
}
