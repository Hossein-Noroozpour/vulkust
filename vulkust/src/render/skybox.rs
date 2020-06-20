use super::super::core::gx3d::{Gx3DReader, Table as Gx3dTable};
use super::super::core::object::Object as CoreObject;
use super::super::core::types::Id;
use super::buffer::Dynamic as DynamicBuffer;
use super::camera::Camera;
use super::command::Buffer as CmdBuffer;
use super::descriptor::Set as DescriptorSet;
use super::engine::Engine;
use super::material::Material;
use super::mesh::Mesh;
use super::model::Uniform;
use super::object::{Base as ObjectBase, Loadable, Object};
use std::collections::BTreeMap;
use std::mem::size_of;
use std::sync::{Arc, RwLock, Weak};

use cgmath;
use gltf;

pub trait Skybox: Object {
    fn update(&mut self, camera: &dyn Camera, frame_number: usize);
    fn get_uniform(&self) -> &Uniform;
    fn render(&self, cmd: &mut CmdBuffer, frame_number: usize);
}

pub trait DefaultSkybox: Skybox + Sized {
    fn default(engine: &Engine) -> Self;
}

#[repr(u8)]
#[cfg_attr(debug_mode, derive(Debug))]
pub enum TypeId {
    Basic = 1,
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Manager {
    engine: Option<Weak<RwLock<Engine>>>,
    skyboxes: BTreeMap<Id, Weak<RwLock<dyn Skybox>>>,
    name_to_id: BTreeMap<String, Id>,
    gx3d_table: Option<Gx3dTable>,
}

impl Manager {
    pub fn new() -> Self {
        Self {
            engine: None,
            skyboxes: BTreeMap::new(),
            name_to_id: BTreeMap::new(),
            gx3d_table: None,
        }
    }

    pub fn load_gx3d(&mut self, engine: &Engine, id: Id) -> Arc<RwLock<dyn Skybox>> {
        if let Some(skybox) = self.skyboxes.get(&id) {
            if let Some(skybox) = skybox.upgrade() {
                return skybox;
            }
        }
        let gx3d_table = vx_unwrap!(self.gx3d_table.as_mut());
        gx3d_table.goto(id);
        let reader = gx3d_table.get_mut_reader();
        let t = reader.read_type_id();
        let skybox: Arc<RwLock<dyn Skybox>> = if t == TypeId::Basic as u8 {
            Arc::new(RwLock::new(Base::new_with_gx3d(engine, reader, id)))
        } else {
            vx_unexpected!()
        };
        self.skyboxes.insert(id, Arc::downgrade(&skybox));
        return skybox;
    }

    pub fn create<M>(&mut self) -> Arc<RwLock<M>>
    where
        M: 'static + DefaultSkybox,
    {
        let eng = vx_unwrap!(vx_unwrap!(&self.engine).upgrade());
        let eng = vx_result!(eng.read());
        let m = M::default(&*eng);
        let id = m.get_id();
        let m1 = Arc::new(RwLock::new(m));
        let m2: Arc<RwLock<dyn Skybox>> = m1.clone();
        self.skyboxes.insert(id, Arc::downgrade(&m2));
        return m1;
    }

    pub(crate) fn set_gx3d_table(&mut self, gx3d_table: Gx3dTable) {
        self.gx3d_table = Some(gx3d_table);
    }

    pub(crate) fn set_engine(&mut self, e: Weak<RwLock<Engine>>) {
        self.engine = Some(e);
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Base {
    obj_base: ObjectBase,
    uniform: Uniform,
    uniform_buffer: DynamicBuffer,
    descriptor_set: Arc<DescriptorSet>,
    mesh: Arc<RwLock<dyn Mesh>>,
    material: Material,
}

impl Base {}

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
        vx_unimplemented!();
    }

    fn disable_rendering(&mut self) {
        self.obj_base.disable_rendering();
    }

    fn enable_rendering(&mut self) {
        self.obj_base.enable_rendering();
    }

    fn is_renderable(&self) -> bool {
        return self.obj_base.is_renderable();
    }
}

impl Loadable for Base {
    fn new_with_gltf(_node: &gltf::Node, _engine: &Engine, _data: &[u8]) -> Self {
        vx_unimplemented!()
    }

    fn new_with_gx3d(eng: &Engine, reader: &mut Gx3DReader, my_id: Id) -> Self {
        let obj_base = ObjectBase::new_with_id(my_id);
        let uniform = Uniform::default();
        let texture_id: Id = reader.read();
        let astmgr = eng.get_asset_manager();
        let mesh = vx_result!(astmgr.get_mesh_manager().write()).create_cube(1.0);
        let texture = vx_result!(astmgr.get_texture_manager().write()).load_gx3d(eng, texture_id);
        let mut material = Material::default(eng);
        material.set_base_color_texture(texture);
        material.finalize_textures_change(eng);
        let gapi_engine = vx_result!(eng.get_gapi_engine().read());
        let uniform_buffer = vx_result!(gapi_engine.get_buffer_manager().write())
            .create_dynamic_buffer(size_of::<Uniform>() as isize);
        let mut descriptor_manager = vx_result!(gapi_engine.get_descriptor_manager().write());
        let descriptor_set = descriptor_manager.create_buffer_only_set(&uniform_buffer);
        Self {
            obj_base,
            uniform,
            uniform_buffer,
            descriptor_set,
            mesh,
            material,
        }
    }
}

impl Skybox for Base {
    fn update(&mut self, camera: &dyn Camera, frame_number: usize) {
        let u = camera.get_uniform();
        let mvp = u.get_projection() * u.get_inversed_rotation();
        let s = (u.get_far() * (1.0 / 3.6)) + (u.get_near() * 0.9);
        let mvp = mvp * cgmath::Matrix4::from_scale(s);
        self.uniform.set_model_view_projection(mvp);
        self.uniform_buffer.update(&self.uniform, frame_number);
        vx_result!(self.mesh.write()).update(frame_number);
        self.material.update_uniform_buffer(frame_number);
    }

    fn get_uniform(&self) -> &Uniform {
        return &self.uniform;
    }

    fn render(&self, cmd: &mut CmdBuffer, frame_number: usize) {
        let buffer = self.uniform_buffer.get_buffer(frame_number);
        cmd.bind_unlit_model_descriptor(&*self.descriptor_set, &*vx_result!(buffer.read()));
        self.material.bind_unlit(cmd, frame_number);
        vx_result!(self.mesh.read()).render_unlit(cmd, frame_number);
    }
}

impl DefaultSkybox for Base {
    fn default(eng: &Engine) -> Self {
        let gapi_engine = vx_result!(eng.get_gapi_engine().read());
        let uniform_buffer = vx_result!(gapi_engine.get_buffer_manager().write())
            .create_dynamic_buffer(size_of::<Uniform>() as isize);
        let mut descriptor_manager = vx_result!(gapi_engine.get_descriptor_manager().write());
        let descriptor_set = descriptor_manager.create_buffer_only_set(&uniform_buffer);
        let astmgr = eng.get_asset_manager();
        let mesh = vx_result!(astmgr.get_mesh_manager().write()).create_cube(1.0);
        Self {
            obj_base: ObjectBase::new(),
            uniform: Uniform::default(),
            uniform_buffer,
            descriptor_set,
            mesh,
            material: Material::default(eng),
        }
    }
}
