use std::cell::RefCell;
use std::collections::BTreeMap;
use std::sync::{Arc, Weak};
use std::io::{Seek, SeekFrom};
use super::super::core::application::ApplicationTrait;
use super::super::system::os::OsApplication;
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
        let count: u64 = file.read_type();
        self.offsets.resize(count as usize, 0);
        for i in 0..count as usize {
            self.offsets[i] = file.read_type();
        }
    }

    pub fn get<CoreApp>(
        &mut self,
        id: u64,
        file: &mut File,
        os_app: *mut OsApplication<CoreApp>,
    ) -> Arc<RefCell<Audio>>
    where
        CoreApp: ApplicationTrait,
    {
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
            10 => Arc::new(RefCell::new(Music::new(file, os_app))),
            20 => Arc::new(RefCell::new(Voice::new(file, os_app))),
            _ => {
                logf!("Uexpected value");
            }
        };
        self.cached.insert(id, Arc::downgrade(&aud));
        return aud;
    }
}
