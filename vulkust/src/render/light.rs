use super::super::core::object::create_id;
use super::super::core::types::Id;
use super::camera::Orthographic;
use math::{Matrix4, Vector3};
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

pub trait Light {}

#[derive(Default)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Sun {
    camera: Orthographic,
}

impl Sun {}

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
