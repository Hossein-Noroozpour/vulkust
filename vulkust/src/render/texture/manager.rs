use std::collections::BTreeMap;
use std::sync::{Arc, Weak};
use std::io::{Seek, SeekFrom};
use super::super::super::system::file::File;
use super::{Texture2D, Texture, Id};

#[derive(Debug)]
pub struct Manager {
    pub cached: BTreeMap<Id, Weak<Texture>>,
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

    pub fn get(&mut self, id: Id, file: &mut File) -> Arc<Texture> {
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
        let type_id = file.read_id();
        let texture = match type_id {
            10 => Texture2D::new(file),
            _ => {
                logf!(
                    "{} {} {} {} {}", 
                    "Requsted texture with Id:", id, 
                    "found but type:", type_id, 
                    "is not implemented yet.");
            }
        };
        let texture: Arc<Texture> = Arc::new(texture);
        self.cached.insert(id, Arc::downgrade(&texture));
        return texture;
    }
}
