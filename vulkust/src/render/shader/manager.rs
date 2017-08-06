use std::collections::BTreeMap;
use std::sync::{Arc, Weak};
use std::io::{Seek, SeekFrom};
use std::ops::{BitOrAssign, ShlAssign};
use std::default::Default;
use super::super::super::core::application::ApplicationTrait;
use super::super::super::system::file::File;
use super::super::super::system::os::OsApplication;
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
        let count: u64 = file.read_type();
        for _ in 0..count {
            let id = read_id(file);
            let offset = file.read_type();
            // logi!("Shader with id: {} and offset {} loaded.", id, offset);
            self.offsets.insert(id, offset);
        }
    }

    pub fn get<CoreApp>(
        &mut self,
        id: u64,
        file: &mut File,
        os_app: *mut OsApplication<CoreApp>,
    ) -> Arc<Shader>
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
            1 => TwoStage::new(file, os_app),
            _ => {
                logf!("Requsted shader Id: {} not found.", id);
            }
        };
        let shader: Arc<Shader> = Arc::new(shader);
        self.cached.insert(id, Arc::downgrade(&shader));
        return shader;
    }
}
