use std::collections::BTreeMap;
use std::sync::{Weak, Arc};
use super::super::super::io::read::Read;
use super::super::super::core::resource::manager::Manager as ResourceManager;
use super::super::super::core::resource::Resource;
use super::Shader;

pub struct Manager {
    pub cached: BTreeMap<u64, Weak<Shader>>,
    pub offsets: BTreeMap<u64, u64>,
}

impl ResourceManager for Manager {
    fn read_tabale(&mut self, file: &mut Read) {
        let count = file.read_type::<u16>();
        for _ in 0..count {
            let id = file.read_type::<u16>() as u64;
            let offset = file.read_type::<u32>() as u64;
            self.offsets.insert(id, offset);
        }
    }
    fn get_resource(id: u64) -> Arc<Resource> {
        
    }
}
