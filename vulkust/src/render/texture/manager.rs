use std::collections::BTreeMap;
use std::sync::{Weak, Arc};
use std::io::{Seek, SeekFrom};
use super::super::super::core::application::ApplicationTrait;
use super::super::super::system::file::File;
use super::super::super::system::os::OsApplication;
use super::{TextureTrait, Texture2D};

#[derive(Debug)]
pub struct Manager {
    pub cached: BTreeMap<u64, Weak<TextureTrait>>,
    pub offsets: BTreeMap<u64, u64>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            cached: BTreeMap::new(),
            offsets: BTreeMap::new(),
        }
    }

    pub fn read_tabale(&mut self, file: &mut File) {
        let count: u64 = file.read_type();
        for _ in 0..count {
            let id: u64 = file.read_type();
            let offset: u64 = file.read_type();
            logi!("Texture with id: {} and offset {} loaded.", id, offset);
            self.offsets.insert(id, offset);
        }
    }

    pub fn get<CoreApp>(
        &mut self, id: u64,
        file: &mut File, os_app: *mut OsApplication<CoreApp>)-> Arc<TextureTrait>
            where CoreApp: ApplicationTrait {
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
        match self.offsets.get(&id) {
            Some(offset) => {
                match file.seek(SeekFrom::Start(*offset)) {
                    Ok(o) => {
                        if o < *offset {
                            logf!("Seeked offset does not match!");
                        }
                    },
                    _ => {
                        logf!("Can not seek to the requested offset.");
                    },
                };
            },
            None => { logf!("Requested texture {} does not exist.", id); },
        };
        let texture = match id {
            1 => { Texture2D::new(file, os_app) },
            _ => { logf!("Requsted texture Id: {} not found.", id); },
        };
        let texture: Arc<TextureTrait> = Arc::new(texture);
        self.cached.insert(id, Arc::downgrade(&texture));
        return texture;
    }
}
