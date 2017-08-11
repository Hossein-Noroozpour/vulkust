use std::cell::RefCell;
use std::collections::BTreeMap;
use std::sync::{Arc, Weak};
use std::io::{Seek, SeekFrom};
use super::super::super::system::file::File;
use super::super::buffer::Buffer;
use super::super::texture::manager::Manager as TextureManager;
use super::super::shader::manager::Manager as ShaderManager;
use super::{read_model, Model};

pub struct Manager {
    pub cached: BTreeMap<u64, Weak<RefCell<Model>>>,
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
        let count: u64 = file.read_count();
        self.offsets.resize(count as usize, 0);
        for i in 0..count as usize {
            self.offsets[i] = file.read_offset();
        }
    }

    pub fn get(
        &mut self,
        id: u64,
        file: &mut File,
        vertices_buffer: &mut Buffer,
        indices_buffer: &mut Buffer,
        texture_manager: &mut TextureManager,
        shader_manager: &mut ShaderManager,
    ) -> Arc<RefCell<Model>> {
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
        let l: Arc<RefCell<Model>> = read_model(
            file, self, vertices_buffer, indices_buffer, texture_manager, shader_manager);
        self.cached.insert(id, Arc::downgrade(&l));
        return l;
    }
}
