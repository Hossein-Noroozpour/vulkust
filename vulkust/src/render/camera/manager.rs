use std::sync::Arc;
use super::super::super::system::file::File;
use super::super::super::util::cache::FileCacher;
use super::super::super::util::cell::DebugCell;
use super::perspective::Perspective;
use super::Camera;

pub struct Manager {
    pub cached: FileCacher<Camera<f32>>,
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

    pub fn get(&mut self, id: u64, ratio: f32) -> Arc<DebugCell<Camera<f32>>> {
        self.cached.get(id, &|| {
            Arc::new(DebugCell::new(Perspective::new(self.cached.get_file(), ratio)))
        })
    }
}
