use super::super::core::object::Object as CoreObject;
use super::super::core::types::{Id, Real};
use super::super::physics::collider::{read as read_collider, Collider, Ghost as GhostCollider};
use super::buffer::DynamicBuffer;
use super::camera::Camera;
use super::command::Buffer as CmdBuffer;
use super::descriptor::Set as DescriptorSet;
use super::engine::Engine;
use super::gx3d::{Gx3DReader, Table as Gx3dTable};
use super::mesh::Mesh;
use super::object::{Base as ObjectBase, Loadable, Object, Transferable};
use super::scene::Scene;
use super::light::VisibilityData as LightVisibilityData;
use std::collections::BTreeMap;
use std::mem::size_of;
use std::sync::{Arc, RwLock, Weak};
use std::f32::MAX as F32MAX;

use gltf;
use math;

pub trait Model: Object + Transferable {
    fn update(&mut self, scene: &Scene, camera: &Camera);
    fn add_mesh(&mut self, mesh: Arc<RwLock<Mesh>>);
    fn clear_meshes(&mut self);
    fn get_meshes_count(&self) -> usize;
    fn bring_all_child_models(&self) -> Vec<(Id, Arc<RwLock<Model>>)>;
    fn has_shadow(&self) -> bool;
    fn get_occlusion_culling_radius(&self) -> Real;
    fn set_light_visibility_data(&mut self, Id, Box<LightVisibilityData>);
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
    pub models: BTreeMap<Id, Weak<RwLock<Model>>>,
    pub name_to_id: BTreeMap<String, Id>,
    pub gx3d_table: Option<Gx3dTable>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
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
        let reader = &mut gx3d_table.reader;
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

    pub fn create<M>(&mut self, eng: &Engine) -> Arc<RwLock<M>>
    where
        M: 'static + DefaultModel,
    {
        let m = M::default(eng);
        let id = m.get_id();
        let m1 = Arc::new(RwLock::new(m));
        let m2: Arc<RwLock<Model>> = m1.clone();
        self.models.insert(id, Arc::downgrade(&m2));
        return m1;
    }
}

#[repr(C)]
#[cfg_attr(debug_mode, derive(Debug))]
pub struct Uniform {
    pub model: math::Matrix4<Real>,
    // todo, I think its not gonna be needed,
    // because of cascaded shadow
    // pub directional_biased_model: math::Matrix4<Real>,
    // pub sun_mvp: math::Matrix4<Real>,
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
    is_in_light: BTreeMap<Id, Box<LightVisibilityData>>,
    is_visible: bool,
    distance_from_camera: Real,
    collider: Arc<RwLock<Collider>>,
    uniform: Uniform,
    uniform_buffer: Arc<RwLock<DynamicBuffer>>,
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

    fn render(&self, cmd: &mut CmdBuffer, frame_number: usize) {
        if !self.is_visible {
            return;
        }
        self.obj_base.render(cmd, frame_number);
        {
            let mut uniform_buffer = vxresult!(self.uniform_buffer.write());
            uniform_buffer.update(&self.uniform, frame_number);
            let buffer = uniform_buffer.get_buffer(frame_number);
            let buffer = vxresult!(buffer.read());
            cmd.bind_gbuff_model_descriptor(&*self.descriptor_set, &*buffer);
        }
        for (_, mesh) in &self.meshes {
            vxresult!(mesh.read()).render(cmd, frame_number);
        }
    }

    fn disable_rendering(&mut self) {
        self.obj_base.disable_rendering();
    }

    fn enable_rendering(&mut self) {
        self.obj_base.enable_rendering();
    }

    fn update(&mut self) {
        self.obj_base.update();
    }

    fn is_rendarable(&self) -> bool {
        return self.obj_base.is_rendarable();
    }
}

impl Loadable for Base {
    fn new_with_gltf(node: &gltf::Node, engine: &Engine, data: &[u8]) -> Self {
        let obj_base = ObjectBase::new();
        let scene_manager = vxresult!(engine.scene_manager.read());
        let mut mesh_manager = vxresult!(scene_manager.mesh_manager.write());
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
        let gapi_engine = vxresult!(engine.gapi_engine.read());
        let uniform_buffer = Arc::new(RwLock::new(
            vxresult!(gapi_engine.buffer_manager.write())
                .create_dynamic_buffer(size_of::<Uniform>() as isize),
        ));
        let mut descriptor_manager = vxresult!(gapi_engine.descriptor_manager.write());
        let descriptor_set = descriptor_manager.create_buffer_only_set(uniform_buffer.clone());
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
            is_in_light: BTreeMap::new(),
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
        let scene_manager = vxresult!(eng.scene_manager.read());
        let mut mesh_manager = vxresult!(scene_manager.mesh_manager.write());
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
        let gapi_engine = vxresult!(eng.gapi_engine.read());
        let uniform_buffer = Arc::new(RwLock::new(
            vxresult!(gapi_engine.buffer_manager.write())
                .create_dynamic_buffer(size_of::<Uniform>() as isize),
        ));
        let mut descriptor_manager = vxresult!(gapi_engine.descriptor_manager.write());
        let descriptor_set = descriptor_manager.create_buffer_only_set(uniform_buffer.clone());
        let descriptor_set = Arc::new(descriptor_set);
        let center = math::Vector3::new(uniform.model.w.x, uniform.model.w.y, uniform.model.w.z);
        vxtodo!(); // not tested
        Base {
            obj_base,
            is_dynamic: false, // todo there must be a dynamic struct
            has_shadow_caster,
            has_transparent,
            occlusion_culling_radius,
            is_in_light: BTreeMap::new(),
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
    fn update(&mut self, scene: &Scene, camera: &Camera) {
        self.is_visible = camera.is_in_frustum(self.occlusion_culling_radius, &self.center);
        if !self.is_visible {
            return;
        }
        if self.has_transparent {
            let dis = camera.get_location() - self.center;
            self.distance_from_camera = math::dot(dis, dis);
        }
        for (_, mesh) in &self.meshes {
            let mut mesh = vxresult!(mesh.write());
            Object::update(&mut *mesh);
            Mesh::update(&mut *mesh, scene, self);
        }
    }

    fn clear_meshes(&mut self) {
        self.meshes.clear();
        self.has_shadow_caster = false;
        self.has_transparent = false;
        self.occlusion_culling_radius = 0.0;
        self.is_in_light.clear();
        self.is_visible = false;
    }

    fn get_meshes_count(&self) -> usize {
        return self.meshes.len();
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

    fn set_light_visibility_data(&mut self, id: Id, lvd: Box<LightVisibilityData>) {
        self.is_in_light.insert(id, lvd);
    }
}

impl DefaultModel for Base {
    fn default(eng: &Engine) -> Self {
        let gapi_engine = vxresult!(eng.gapi_engine.read());
        let uniform_buffer = Arc::new(RwLock::new(
            vxresult!(gapi_engine.buffer_manager.write())
                .create_dynamic_buffer(size_of::<Uniform>() as isize),
        ));
        let mut descriptor_manager = vxresult!(gapi_engine.descriptor_manager.write());
        let descriptor_set = descriptor_manager.create_buffer_only_set(uniform_buffer.clone());
        let descriptor_set = Arc::new(descriptor_set);
        Base {
            obj_base: ObjectBase::new(),
            is_dynamic: true,
            has_shadow_caster: false,
            has_transparent: false,
            occlusion_culling_radius: 0.0,
            is_in_light: BTreeMap::new(),
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
