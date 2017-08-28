use std::cell::DebugCell;
use std::collections::BTreeMap;
use std::sync::{Arc, Weak};
use std::io::{Seek, SeekFrom};
use super::super::super::core::application::ApplicationTrait;
use super::super::super::system::file::File;
use super::super::buffer::Manager as BufferManager;
use super::super::engine::RenderEngine;
use super::super::texture::manager::Manager as TextureManager;
use super::super::shader::manager::Manager as ShaderManager;
use super::{read_model, Model};

pub struct Manager {
    pub cached: BTreeMap<u64, BTreeMap<u64, Weak<DebugCell<Model>>>>,
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

    pub fn get<CoreApp>(
        &mut self,
        id: u64,
        file: &mut File,
        engine: &mut RenderEngine<CoreApp>,
    ) -> Arc<DebugCell<Model>> 
    where
        CoreApp: ApplicationTrait,
    {
        let w_buffer = buffer_manager.get_id();
        match self.cached.get(&w_buffer) {
            Some(cached) => match cached.get(&id) {
                Some(res) => match res.upgrade() {
                    Some(res) => {
                        #[cfg(model_import_debug)]
                        logi!("Model with id {}", id);
                        return res;
                    }
                    None => {}
                },
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
        let l: Arc<DebugCell<Model>> =
            read_model(file, self, buffer_manager, texture_manager, shader_manager);
        let mut cached = BTreeMap::new();
        cached.insert(id, Arc::downgrade(&l));
        self.cached.insert(w_buffer, cached);
        return l;
    }
}
