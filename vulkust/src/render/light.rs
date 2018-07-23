use super::super::core::object::Object as CoreObject;
use super::super::core::types::Id;
use super::camera::Orthographic;
use super::engine::Engine;
use super::object::Object;
// use math::{Matrix4, Vector3};
use super::gx3d::Table as Gx3dTable;
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

pub trait Light: Object {
    // fn set_cascaded_frustums() // todo
}

pub trait DefaultLighting {
    fn default(eng: &Arc<RwLock<Engine>>, size: f32) -> Self;
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Sun {
    camera: Orthographic,
}

impl Sun {}

impl CoreObject for Sun {
    fn get_id(&self) -> Id {
        self.camera.get_id()
    }
}

impl Object for Sun {
    fn get_name(&self) -> Option<String> {
        self.camera.get_name()
    }

    fn set_name(&mut self, name: &str) {
        self.camera.set_name(name);
        vxunimplemented!(); //it must update corresponding manager
    }

    fn render(&self) {
        vxlogf!("Sun light does not implement rendering.");
    }

    fn disable_rendering(&mut self) {
        self.camera.disable_rendering()
    }

    fn enable_rendering(&mut self) {
        self.camera.enable_rendering()
    }

    fn update(&mut self) {
        self.camera.update();
    }
}

impl Light for Sun {}

impl DefaultLighting for Sun {
    fn default(eng: &Arc<RwLock<Engine>>, size: f32) -> Self {
        Sun {
            camera: Orthographic::new(eng, size),
        }
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Manager {
    pub lights: BTreeMap<Id, Arc<RwLock<Light>>>,
    pub name_to_id: BTreeMap<String, Id>,
    pub gx3d_table: Option<Gx3dTable>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            lights: BTreeMap::new(),
            name_to_id: BTreeMap::new(),
            gx3d_table: None,
        }
    }

    pub fn create<L>(&mut self, eng: &Arc<RwLock<Engine>>, name: &str) -> Arc<RwLock<L>>
    where
        L: 'static + Light + DefaultLighting,
    {
        let result = Arc::new(RwLock::new(L::default(eng, 1.0)));
        let light: Arc<RwLock<Light>> = result.clone();
        let id = vxresult!(light.read()).get_id();
        self.lights.insert(id, light);
        self.name_to_id.insert(name.to_string(), id);
        return result;
    }
}
