use std::collections::BTreeMap;
use std::sync::{Arc, Weak};
use std::io::{Seek, SeekFrom};
use super::super::super::system::file::File;
use super::super::super::util::cell::DebugCell;
use super::perspective::Perspective;
use super::Camera;

pub struct Manager {
    pub cached: BTreeMap<u64, Weak<DebugCell<Camera<f32>>>>,
    pub offsets: Vec<u64>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            cached: BTreeMap::new(),
            offsets: Vec::new(),
        }
    }

    pub fn read_table(&mut self, file: &mut File) {
        let count = file.read_count();
        self.offsets.resize(count as usize, 0);
        for i in 0..count as usize {
            self.offsets[i] = file.read_offset();
        }
    }

    pub fn get(&mut self, id: u64, file: &mut File, ratio: f32) -> Arc<DebugCell<Camera<f32>>> {
        match self.cached.get(&id) {
            Some(res) => match res.upgrade() {
                Some(res) => {
                    return res;
                }
                None => {}
            },
            None => {}
        }
        let offset = self.offsets[id as usize];
        match file.seek(SeekFrom::Start(offset)) {
            Ok(o) => if o < offset {
                logf!("Seeked offset does not match!");
            },
            _ => {
                logf!("Can not seek to the requested offset.");
            }
        }
        let camera = Perspective::new(file, ratio);
        let camera: Arc<DebugCell<Camera<f32>>> = Arc::new(DebugCell::new(camera));
        self.cached.insert(id, Arc::downgrade(&camera));
        return camera;
    }
}
