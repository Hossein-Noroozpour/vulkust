use super::super::core::types::Id;
use super::super::core::object::Object as CoreObject;
use super::gx3d::Table as Gx3dTable;
use super::object::{Loadable, Object};
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

pub trait Model {}

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

pub struct Base {

}

impl Base {
}

impl CoreObject for Base {
    
}

impl Object for Base {}

impl Loadable for Base {
    
}

impl Model for Base {}