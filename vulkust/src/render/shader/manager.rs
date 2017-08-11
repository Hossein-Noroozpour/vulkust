use std::collections::BTreeMap;
use std::sync::{Arc, Weak};
use std::io::{Seek, SeekFrom};
use super::super::super::system::file::File;
use super::super::device::logical::Logical as LogicalDevice;
use super::{TwoStage, Shader, read_id};

pub struct Manager {
    pub cached: BTreeMap<u64, Weak<Shader>>,
    pub offsets: BTreeMap<u64, u64>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            cached: BTreeMap::new(),
            offsets: BTreeMap::new(),
        }
    }

    pub fn read_table(&mut self, file: &mut File) {
        let count = file.read_count();
        for _ in 0..count {
            let id = read_id(file);
            let offset = file.read_offset();
            #[cfg(shader_import_debug)]
            logi!("Shader with id: {} and offset {} loaded.", id, offset);
            self.offsets.insert(id, offset);
        }
    }

    pub fn get(
        &mut self, id: u64, file: &mut File, logical_device: Arc<LogicalDevice>) -> Arc<Shader> {
        match self.cached.get(&id) {
            Some(res) => match res.upgrade() {
                Some(res) => {
                    return res;
                }
                None => {}
            },
            None => {}
        }
        match self.offsets.get(&id) {
            Some(offset) => {
                match file.seek(SeekFrom::Start(*offset)) {
                    Ok(o) => if o < *offset {
                        logf!("Seeked offset does not match!");
                    },
                    _ => {
                        logf!("Can not seek to the requested offset.");
                    }
                };
            }
            None => {
                logf!("Requested shader {} does not exist.", id);
            }
        };
        let shader = match id {
            1 => TwoStage::new(file, logical_device),
            _ => {
                logf!("Requsted shader Id: {} not found.", id);
            }
        };
        let shader: Arc<Shader> = Arc::new(shader);
        self.cached.insert(id, Arc::downgrade(&shader));
        return shader;
    }
}
