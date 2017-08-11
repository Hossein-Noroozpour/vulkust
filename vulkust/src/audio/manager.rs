use std::cell::RefCell;
use std::collections::BTreeMap;
use std::sync::{Arc, Weak};
use std::io::{Seek, SeekFrom};
use super::super::system::file::File;
use super::{Audio, Music, Voice};

pub struct Manager {
    pub cached: BTreeMap<u64, Weak<RefCell<Audio>>>,
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

    pub fn get(&mut self, id: u64, file: &mut File) -> Arc<RefCell<Audio>> {
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
        let audio_type: u64 = file.read_type();
        let aud: Arc<RefCell<Audio>> = match audio_type {
            10 => Arc::new(RefCell::new(Music::new(file))),
            20 => Arc::new(RefCell::new(Voice::new(file))),
            _ => {
                logf!("Uexpected value");
            }
        };
        self.cached.insert(id, Arc::downgrade(&aud));
        return aud;
    }
}
