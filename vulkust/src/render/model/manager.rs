use std::mem::transmute;
use std::sync::Arc;
use super::super::super::core::application::ApplicationTrait;
use super::super::super::system::file::File;
use super::super::super::util::cache::FileCacher;
use super::super::super::util::cell::DebugCell;
use super::super::engine::RenderEngine;
use super::{read_model, Model};

pub struct Manager {
    pub cached: FileCacher<Model>,
}

impl Manager {
    pub fn new(file: Arc<DebugCell<File>>) -> Self {
        Manager {
            cached: FileCacher::new(file),
        }
    }

    pub fn read_table(&mut self) {
        self.cached.read_offsets();
    }

    pub fn get<CoreApp>(
        &mut self,
        id: u64,
        engine: &mut RenderEngine<CoreApp>,
    ) -> Arc<DebugCell<Model>> 
    where
        CoreApp: ApplicationTrait,
    {
        let file = self.cached.get_file().clone();
        let engine: usize = unsafe { transmute(engine) };
        self.cached.get(id, &|| {
            let engine: &mut RenderEngine<CoreApp> = unsafe { transmute(engine) };
            read_model(&file, engine)
        })
    }
}
