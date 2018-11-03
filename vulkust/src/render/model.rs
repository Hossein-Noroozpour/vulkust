use super::super::core::gx3d::{Gx3DReader, Table as Gx3dTable};
use super::super::core::object::Object as CoreObject;
use super::super::core::types::{Id, Real};
use super::super::physics::collider::{read as read_collider, Collider, Ghost as GhostCollider};
use super::buffer::DynamicBuffer;
use super::camera::Camera;
use super::command::Buffer as CmdBuffer;
use super::descriptor::Set as DescriptorSet;
use super::engine::Engine;
use super::mesh::Mesh;
use super::object::{Base as ObjectBase, Loadable, Object, Transferable};
use super::scene::Scene;
use std::collections::BTreeMap;
use std::f32::MAX as F32MAX;
use std::mem::size_of;
use std::sync::{Arc, RwLock, Weak};

use gltf;
use math;

pub trait Model: Object + Transferable {
    fn update(&mut self, &Scene, &Camera, usize);
    fn add_mesh(&mut self, Arc<RwLock<Mesh>>);
    fn clear_meshes(&mut self);
    fn get_meshes(&self) -> &BTreeMap<Id, Arc<RwLock<Mesh>>>;
    fn bring_all_child_models(&self) -> Vec<(Id, Arc<RwLock<Model>>)>;
    fn has_shadow(&self) -> bool;
    fn get_occlusion_culling_radius(&self) -> Real;
    fn get_uniform(&self) -> &Uniform;
    fn render_gbuffer(&self, &mut CmdBuffer, usize);
    fn render_shadow(&self, &mut CmdBuffer, usize);
}

pub trait DefaultModel: Model + Sized {
    fn default(&Engine) -> Self;
}

#[repr(u8)]
#[cfg_attr(debug_mode, derive(Debug))]
pub enum TypeId {
    Dynamic = 1,
    Static = 2,
    Widget = 3,
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Manager {
    engine: Option<Weak<RwLock<Engine>>>,
    models: BTreeMap<Id, Weak<RwLock<Model>>>,
    name_to_id: BTreeMap<String, Id>,
    gx3d_table: Option<Gx3dTable>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            engine: None,
            models: BTreeMap::new(),
            name_to_id: BTreeMap::new(),
            gx3d_table: None,
        }
    }

    pub fn load_gx3d(&mut self, engine: &Engine, id: Id) -> Arc<RwLock<Model>> {
        if let Some(model) = self.models.get(&id) {
            if let Some(model) = model.upgrade() {
                return model;
            }
        }
        let gx3d_table = vxunwrap!(self.gx3d_table.as_mut());
        gx3d_table.goto(id);
        let reader = &mut gx3d_table.get_mut_reader();
        let t = reader.read_type_id();
        let model: Arc<RwLock<Model>> = if t == TypeId::Static as u8 {
            Arc::new(RwLock::new(Base::new_with_gx3d(engine, reader, id)))
        } else if t == TypeId::Dynamic as u8 {
            vxunimplemented!()
        } else if t == TypeId::Widget as u8 {
            vxunimplemented!()
        } else {
            vxunexpected!()
        };
        self.models.insert(id, Arc::downgrade(&model));
        return model;
    }

    pub fn create<M>(&mut self) -> Arc<RwLock<M>>
    where
        M: 'static + DefaultModel,
    {
        let eng = vxunwrap!(vxunwrap!(&self.engine).upgrade());
        let eng = vxresult!(eng.read());
        let m = M::default(&*eng);
        let id = m.get_id();
        let m1 = Arc::new(RwLock::new(m));
        let m2: Arc<RwLock<Model>> = m1.clone();
        self.models.insert(id, Arc::downgrade(&m2));
        return m1;
    }

    pub(crate) fn set_gx3d_table(&mut self, gx3d_table: Gx3dTable) {
        self.gx3d_table = Some(gx3d_table);
    }

    pub(crate) fn set_engine(&mut self, e: Weak<RwLock<Engine>>) {
        self.engine = Some(e);
    }
}

#[repr(C)]
#[cfg_attr(debug_mode, derive(Debug))]
pub struct Uniform {
    model: math::Matrix4<Real>,
}

impl Uniform {
    fn new_with_gltf(node: &gltf::Node) -> Self {
        let m = node.transform().matrix();
        let model = math::Matrix4::new(
            m[0][0], m[0][1], m[0][2], m[0][3], m[1][0], m[1][1], m[1][2], m[1][3], m[2][0],
            m[2][1], m[2][2], m[2][3], m[3][0], m[3][1], m[3][2], m[3][3],
        );
        Uniform { model }
    }

    fn new_with_gx3d(reader: &mut Gx3DReader) -> Self {
        let model = math::Matrix4::new(
            reader.read(),
            reader.read(),
            reader.read(),
            reader.read(),
            reader.read(),
            reader.read(),
            reader.read(),
            reader.read(),
            reader.read(),
            reader.read(),
            reader.read(),
            reader.read(),
            reader.read(),
            reader.read(),
            reader.read(),
            reader.read(),
        );
        Uniform { model }
    }

    pub(crate) fn get_model(&self) -> &math::Matrix4<Real> {
        return &self.model;
    }

    fn default() -> Self {
        let m = math::Matrix4::new(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        );
        Uniform { model: m }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Base {
    obj_base: ObjectBase,
    is_dynamic: bool,
    has_shadow_caster: bool,
    has_transparent: bool,
    occlusion_culling_radius: Real,
    is_visible: bool,
    distance_from_camera: Real,
    collider: Arc<RwLock<Collider>>,
    uniform: Uniform,
    uniform_buffer: DynamicBuffer,
    descriptor_set: Arc<DescriptorSet>,
    meshes: BTreeMap<Id, Arc<RwLock<Mesh>>>,
    children: BTreeMap<Id, Arc<RwLock<Model>>>,
    center: math::Vector3<Real>,
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

    fn set_name(&mut self, _name: &str) {
        self.obj_base.get_name();
        vxunimplemented!();
    }

    fn disable_rendering(&mut self) {
        self.obj_base.disable_rendering();
    }

    fn enable_rendering(&mut self) {
        self.obj_base.enable_rendering();
    }

    fn is_rendarable(&self) -> bool {
        return self.obj_base.is_rendarable();
    }
}

impl Loadable for Base {
    fn new_with_gltf(node: &gltf::Node, engine: &Engine, data: &[u8]) -> Self {
        let obj_base = ObjectBase::new();
        let mut mesh_manager = vxresult!(engine.get_asset_manager().get_mesh_manager().write());
        let model = vxunwrap!(node.mesh());
        let primitives = model.primitives();
        let mut meshes = BTreeMap::new();
        let mut has_shadow_caster = false;
        let mut has_transparent = false;
        let mut occlusion_culling_radius = 0.0001;
        for primitive in primitives {
            let mesh = mesh_manager.load_gltf(&primitive, &engine, data);
            let id = {
                let mesh = vxresult!(mesh.read());
                has_shadow_caster |= mesh.is_shadow_caster();
                has_transparent |= mesh.is_transparent();
                let occ = mesh.get_occlusion_culling_radius();
                if occ > occlusion_culling_radius {
                    occlusion_culling_radius = occ;
                }
                mesh.get_id()
            };
            meshes.insert(id, mesh);
        }
        if node.children().count() > 0 {
            vxunimplemented!(); // todo support children
        }
        let gapi_engine = vxresult!(engine.get_gapi_engine().read());
        let uniform_buffer = vxresult!(gapi_engine.get_buffer_manager().write())
            .create_dynamic_buffer(size_of::<Uniform>() as isize);
        let mut descriptor_manager = vxresult!(gapi_engine.get_descriptor_manager().write());
        let descriptor_set = descriptor_manager.create_buffer_only_set(&uniform_buffer);
        let descriptor_set = Arc::new(descriptor_set);
        let uniform = Uniform::new_with_gltf(node);
        let center = math::Vector3::new(uniform.model.w.x, uniform.model.w.y, uniform.model.w.z);
        vxtodo!(); // not tested
        Base {
            obj_base,
            is_dynamic: true,
            has_shadow_caster,
            has_transparent,
            occlusion_culling_radius,
            is_visible: false,
            distance_from_camera: F32MAX,
            collider: Arc::new(RwLock::new(GhostCollider::new())),
            uniform,
            uniform_buffer,
            descriptor_set,
            meshes,
            children: BTreeMap::new(),
            center,
        }
    }

    fn new_with_gx3d(eng: &Engine, reader: &mut Gx3DReader, my_id: Id) -> Self {
        let obj_base = ObjectBase::new_with_id(my_id);
        let uniform = Uniform::new_with_gx3d(reader);
        let occlusion_culling_radius = reader.read();
        let collider = read_collider(reader);
        let meshes_ids = reader.read_array();
        let mut mesh_manager = vxresult!(eng.get_asset_manager().get_mesh_manager().write());
        let mut meshes = BTreeMap::new();
        let mut has_shadow_caster = false;
        let mut has_transparent = false;
        for mesh_id in meshes_ids {
            let mesh = mesh_manager.load_gx3d(eng, mesh_id);
            {
                let mesh = vxresult!(mesh.read());
                has_shadow_caster |= mesh.is_shadow_caster();
                has_transparent |= mesh.is_transparent();
            }
            meshes.insert(mesh_id, mesh);
        }
        let gapi_engine = vxresult!(eng.get_gapi_engine().read());
        let uniform_buffer = vxresult!(gapi_engine.get_buffer_manager().write())
            .create_dynamic_buffer(size_of::<Uniform>() as isize);
        let mut descriptor_manager = vxresult!(gapi_engine.get_descriptor_manager().write());
        let descriptor_set = descriptor_manager.create_buffer_only_set(&uniform_buffer);
        let descriptor_set = Arc::new(descriptor_set);
        let center = math::Vector3::new(uniform.model.w.x, uniform.model.w.y, uniform.model.w.z);
        vxtodo!(); // not tested
        Base {
            obj_base,
            is_dynamic: false, // todo there must be a dynamic struct
            has_shadow_caster,
            has_transparent,
            occlusion_culling_radius,
            is_visible: false,
            distance_from_camera: F32MAX,
            collider,
            uniform,
            uniform_buffer,
            descriptor_set,
            meshes,
            children: BTreeMap::new(),
            center,
        }
    }
}

impl Transferable for Base {
    fn set_orientation(&mut self, _: &math::Quaternion<Real>) {
        vxunimplemented!();
    }

    fn set_location(&mut self, _: &math::Vector3<Real>) {
        vxunimplemented!();
    }

    fn get_location(&self) -> &math::Vector3<Real> {
        return &self.center;
    }

    fn move_local_z(&mut self, _: Real) {
        vxunimplemented!();
    }

    fn move_local_x(&mut self, _: Real) {
        vxunimplemented!();
    }

    fn rotate_local_x(&mut self, _: Real) {
        vxunimplemented!();
    }

    fn rotate_global_z(&mut self, _: Real) {
        vxunimplemented!();
    }
}

impl Model for Base {
    fn update(&mut self, scene: &Scene, camera: &Camera, frame_number: usize) {
        self.is_visible = camera.is_in_frustum(self.occlusion_culling_radius, &self.center);
        if !self.is_visible {
            return;
        }
        if self.has_transparent {
            let dis = camera.get_location() - self.center;
            self.distance_from_camera = math::dot(dis, dis);
        }
        self.uniform_buffer.update(&self.uniform, frame_number);
        for (_, m) in &self.meshes {
            vxresult!(m.write()).update(scene, self, frame_number);
        }
    }

    fn clear_meshes(&mut self) {
        self.meshes.clear();
        self.has_shadow_caster = false;
        self.has_transparent = false;
        self.occlusion_culling_radius = 0.0;
        self.is_visible = false;
    }

    fn get_uniform(&self) -> &Uniform {
        return &self.uniform;
    }

    fn get_meshes(&self) -> &BTreeMap<Id, Arc<RwLock<Mesh>>> {
        return &self.meshes;
    }

    fn add_mesh(&mut self, mesh: Arc<RwLock<Mesh>>) {
        let id = {
            let mesh = vxresult!(mesh.read());
            let radius = mesh.get_occlusion_culling_radius();
            if self.occlusion_culling_radius < radius {
                self.occlusion_culling_radius = radius;
            }
            mesh.get_id()
        };
        self.meshes.insert(id, mesh);
    }

    fn bring_all_child_models(&self) -> Vec<(Id, Arc<RwLock<Model>>)> {
        let mut result = Vec::new();
        for (id, model) in &self.children {
            result.push((*id, model.clone()));
            let mut models = vxresult!(model.read()).bring_all_child_models();
            result.append(&mut models);
        }
        return result;
    }

    fn has_shadow(&self) -> bool {
        return self.has_shadow_caster;
    }

    fn get_occlusion_culling_radius(&self) -> Real {
        return self.occlusion_culling_radius;
    }

    fn render_gbuffer(&self, cmd: &mut CmdBuffer, frame_number: usize) {
        if !self.is_visible {
            return;
        }
        let buffer = self.uniform_buffer.get_buffer(frame_number);
        cmd.bind_gbuff_model_descriptor(&*self.descriptor_set, &*vxresult!(buffer.read()));
        for (_, mesh) in &self.meshes {
            vxresult!(mesh.read()).render_gbuffer(cmd, frame_number);
        }
    }

    fn render_shadow(&self, cmd: &mut CmdBuffer, frame_number: usize) {
        for (_, mesh) in &self.meshes {
            let mesh = vxresult!(mesh.read());
            mesh.render_shadow(cmd, frame_number);
        }
    }
}

impl DefaultModel for Base {
    fn default(eng: &Engine) -> Self {
        let gapi_engine = vxresult!(eng.get_gapi_engine().read());
        let uniform_buffer = vxresult!(gapi_engine.get_buffer_manager().write())
            .create_dynamic_buffer(size_of::<Uniform>() as isize);
        let mut descriptor_manager = vxresult!(gapi_engine.get_descriptor_manager().write());
        let descriptor_set = descriptor_manager.create_buffer_only_set(&uniform_buffer);
        let descriptor_set = Arc::new(descriptor_set);
        Base {
            obj_base: ObjectBase::new(),
            is_dynamic: true,
            has_shadow_caster: true,
            has_transparent: false,
            occlusion_culling_radius: 0.0,
            is_visible: false,
            distance_from_camera: 100000.0,
            collider: Arc::new(RwLock::new(GhostCollider::new())),
            uniform: Uniform::default(),
            uniform_buffer,
            descriptor_set,
            meshes: BTreeMap::new(),
            children: BTreeMap::new(),
            center: math::Vector3::new(0.0, 0.0, 0.0),
        }
    }
}
