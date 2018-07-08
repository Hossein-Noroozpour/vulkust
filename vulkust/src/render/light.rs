use super::super::core::object::{Object as CoreObject};
use super::super::core::types::Id;
use super::camera::Orthographic;
use super::object::{Object};
use math::{Matrix4, Vector3};
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

pub trait Light: CoreObject + Object {}

pub trait DefaultLighting {
    fn default(size: f32, name: &str) -> Self;
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Sun {
    camera: Orthographic,
}

impl Sun {

}

impl CoreObject for Sun {
    fn get_id(&self) -> Id {
        self.camera.get_id()
    }
}

impl Object for Sun {
    fn name(&self) -> &str {
        self.camera.name()
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
    fn default(size: f32, name: &str) -> Self {
        Sun {
            camera: Orthographic::new(size, name)
        }
    }
}

pub struct Manager {
    pub cameras: BTreeMap<Id, Arc<RwLock<Light>>>,
    pub name_to_id: BTreeMap<String, Id>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            cameras: BTreeMap::new(),
            name_to_id: BTreeMap::new(),
        }
    }

    pub fn create<L>(&mut self, name: &str) -> Arc<RwLock<L>>
    where
        L: 'static + Light + DefaultLighting,
    {
        let result = Arc::new(RwLock::new(L::default(1.0, name)));
        let light: Arc<RwLock<Light>> = result.clone();
        let id = vxresult!(light.read()).get_id();
        self.cameras.insert(id, light);
        self.name_to_id.insert(name.to_string(), id);
        return result;
    }
}
