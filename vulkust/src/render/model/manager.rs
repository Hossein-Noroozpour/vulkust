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
    pub fn new() -> Self {
        Manager {
            cached: FileCacher::new(),
        }
    }

    pub fn read_tables(&mut self, file: &Arc<DebugCell<File>>) {
        self.cached.read_offsets(file);
    }

    pub fn get<CoreApp>(
        &mut self,
        id: u64,
        file: &Arc<DebugCell<File>>,
        engine: &mut RenderEngine<CoreApp>,
    ) -> Arc<DebugCell<Model>> 
    where
        CoreApp: ApplicationTrait,
    {
        self.cached.get(id, file, &|| {
            read_model(file, engine)
        })
    }
}
