use super::super::core::object::{Object as CoreObject};
use super::super::core::types::Id;
use super::camera::Orthographic;
use super::object::{Basic as BasicObject, Object};
use math::{Matrix4, Vector3};
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

pub trait Light: CoreObject + Object {}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Sun {
    obj_basic: BasicObject,
    camera: Orthographic,
}

impl Sun {

}

impl CoreObject for Sun {
    fn get_id(&self) -> Id {
        self.obj_basic.get_id()
    }
}

impl Object for Sun {}

impl Light for Sun {}

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
        L: 'static + Light + Default,
    {
        let name = name.to_string();
        let id = create_id();
        let result = Arc::new(RwLock::new(L::default()));
        let light: Arc<RwLock<Light>> = result.clone();
        self.cameras.insert(id, light);
        self.name_to_id.insert(name, id);
        return result;
    }
}
