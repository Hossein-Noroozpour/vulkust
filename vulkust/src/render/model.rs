use super::super::core::types::Id;
use super::super::core::object::Object as CoreObject;
use super::gx3d::{ Gx3DReader, Table as Gx3dTable};
use super::object::{Base as ObjectBase, Loadable, Object};
use super::mesh::{Base as MeshBase, Mesh};
use super::engine::Engine;
use super::texture::{Texture2D, Loadable as TextureLoadable};
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};
use std::mem::size_of;

use math;
use gltf;

pub trait Model: Object {}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Manager {
    pub models: BTreeMap<Id, Arc<RwLock<Model>>>,
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
}

#[repr(C)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Uniform {
    pub model: math::Matrix4<f32>,
    pub model_view_projection: math::Matrix4<f32>,
    // todo, I think its not gonna be needed, 
    // because of cascaded shadow
    // pub directional_biased_model: math::Matrix4<f32>,
    // pub sun_mvp: math::Matrix4<f32>,
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Base {
    pub obj_base: ObjectBase,
    pub is_dynamic: bool,
    pub has_shadow_caster: bool,
    pub has_transparent: bool,
    pub occlusion_culling_radius: f32,
    pub is_in_sun: Vec<bool>,
    pub is_in_camera: Vec<bool>,
    pub distance_from_cameras: Vec<f32>,
    pub meshes: BTreeMap<Id, Arc<RwLock<Mesh>>>,
    pub children: BTreeMap<Id, Arc<RwLock<Model>>>,
    // pub collider: Arc<RwLock<Collider>>,
}

impl Base {
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
        self.obj_base.get_name();
        vxunimplemented!();
    }

    fn render(&self, engine: &Engine) {
        if !self.obj_base.renderable {
            return;
        }
        self.obj_base.render(engine);
        vxunimplemented!();
    }

    fn disable_rendering(&mut self) {
        self.obj_base.disable_rendering();
    }

    fn enable_rendering(&mut self) {
        self.obj_base.enable_rendering();
    }

    fn update(&mut self) {
        vxunimplemented!();
    }
}

impl Loadable for Base {
    fn new_with_gltf(node: &gltf::Node, eng: &Arc<RwLock<Engine>>, data: &[u8]) -> Self {
        let obj_base = ObjectBase::new();
        let engine = eng.clone();
        let model = vxunwrap!(node.mesh());
        let primitives = model.primitives();
        let mut meshes = BTreeMap::new();
        for primitive in primitives {
            let engine = vxresult!(eng.read());
            let scene_manager = vxresult!(engine.scene_manager.read());
            let mesh_manager = vxresult!(scene_manager.mesh_manager.write());
            let mesh = mesh_manager.load_gltf(primitive, eng, data);
            let id = vxresult!(mesh.read()).get_id();
            meshes.insert(id, mesh);
        }
        Base {
            obj_base,
            meshes,
            // buffer: buffer,
            // buffer_manager: buffer_manager,
            // material: material,
        }
    }

    fn new_with_gx3d(engine: &Arc<RwLock<Engine>>, reader: &mut Gx3DReader, my_id: Id) -> Self {
        let obj_base = ObjectBase::new_with_id(my_id);
    }
}

impl Model for Base {}