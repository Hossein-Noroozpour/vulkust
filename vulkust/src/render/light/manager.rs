use std::sync::Arc;
use super::super::super::system::file::File;
use super::super::super::util::cell::DebugCell;
use super::super::super::util::cache::FileCacher;
use super::{Light, Sun};

pub struct Manager {
    pub cached: FileCacher<Light>,
}

impl Manager {
    pub fn new(file: Arc<DebugCell<File>>) -> Self {
        Manager {
            cached: FileCacher::new(file),
        }
    }

    pub fn read_table(&mut self) {
        self.cached.read_offsets();
    }

    pub fn get(&mut self, id: u64) -> Arc<DebugCell<Light>> {
        self.cached.get(id, &|| {
            Arc::new(DebugCell::new(Sun::new(self.cached.get_file())))
        })
    }
}
