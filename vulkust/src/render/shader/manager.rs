use std::collections::BTreeMap;
use std::sync::{Weak, Arc, Mutex};
use super::super::super::io::read::Read;
use super::super::super::core::resource::manager::Manager as ResourceManager;
use super::super::super::core::resource::Resource;
use super::Shader;

pub struct Manager<File> where File: Read {
    pub cached: BTreeMap<u64, Weak<Resource>>,
    pub offsets: BTreeMap<u64, u64>,
    pub file: Arc<Mutex<File>>,
}

impl<File> ResourceManager<File> for Manager<File> where File: Read {
    fn read_tabale(&mut self, file: Arc<Mutex<File>>) {
        self.file = file.clone();
        let mut file = file.lock().unwrap();
        let count = file.read_type::<u16>();
        for _ in 0..count {
            let id = file.read_type::<u16>() as u64;
            let offset = file.read_type::<u32>() as u64;
            self.offsets.insert(id, offset);
        }
    }
    fn get_resource(&mut self, id: u64) -> Arc<Resource> {
        match self.cached.get(&id) {
            Some(res) => {
                match res.upgrade() {
                    Some(res) => {
                        return res;
                    },
                    None => {},
                }
            },
            None => {},
        }
        let shader = Shader {};
        // TODO: do reading of shader in here.
        let r = Arc::new(shader);
        return r;
    }
}
