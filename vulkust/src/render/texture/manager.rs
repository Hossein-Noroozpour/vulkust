use std::collections::BTreeMap;
use std::sync::{Arc, Weak};
use std::io::{Seek, SeekFrom};
use super::super::super::core::application::ApplicationTrait;
use super::super::super::system::file::File;
use super::super::super::system::os::OsApplication;
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
        let count: u64 = file.read_type();
        self.offsets.resize(count as usize, 0);
        for i in 0..count as usize {
            self.offsets[i] = file.read_type();
        }
    }

    pub fn get<CoreApp>(
        &mut self,
        id: Id,
        file: &mut File,
        os_app: *mut OsApplication<CoreApp>,
    ) -> Arc<Texture>
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
        let texture = match id {
            1 => Texture2D::new(file, os_app),
            _ => {
                logf!("Requsted texture Id: {} not found.", id);
            }
        };
        let texture: Arc<Texture> = Arc::new(texture);
        self.cached.insert(id, Arc::downgrade(&texture));
        return texture;
    }
}
