use std::collections::BTreeMap;
use std::sync::{Weak, Arc};
use std::io::{Seek, SeekFrom};
use super::super::super::core::application::ApplicationTrait;
use super::super::super::system::file::File;
use super::super::super::system::os::OsApplication;
use super::{ShaderTrait, Shader};

#[derive(Debug)]
pub struct Manager {
    pub cached: BTreeMap<u64, Weak<ShaderTrait>>,
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
            logi!("Shader with id: {} and offset {} loaded.", id, offset);
            self.offsets.insert(id, offset);
        }
    }

    pub fn get<CoreApp>(
        &mut self,
        id: u64,
        file: &mut File,
        os_app: *mut OsApplication<CoreApp>,
    ) -> Arc<ShaderTrait>
    where
        CoreApp: ApplicationTrait,
    {
        match self.cached.get(&id) {
            Some(res) => {
                match res.upgrade() {
                    Some(res) => {
                        return res;
                    }
                    None => {}
                }
            }
            None => {}
        }
        match self.offsets.get(&id) {
            Some(offset) => {
                match file.seek(SeekFrom::Start(*offset)) {
                    Ok(o) => {
                        if o < *offset {
                            logf!("Seeked offset does not match!");
                        }
                    }
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
            1 => Shader::new(file, os_app),
            _ => {
                logf!("Requsted shader Id: {} not found.", id);
            }
        };
        let shader: Arc<ShaderTrait> = Arc::new(shader);
        self.cached.insert(id, Arc::downgrade(&shader));
        return shader;
    }
}
