use std::sync::Arc;
use std::mem::transmute;
use super::super::super::core::application::ApplicationTrait;
use super::super::super::system::file::File;
use super::super::super::util::cache::FileCacher;
use super::super::super::util::cell::DebugCell;
use super::super::engine::RenderEngine;
use super::{BasicScene, Scene};

pub struct Manager {
    pub cached: FileCacher<Scene>,
}

impl Manager {
    pub fn new(file: Arc<DebugCell<File>>) -> Self {
        Manager {
            cached: FileCacher::new(file),
        }
    }

    pub fn read_table(&mut self) {
        self.cached.read_offsets()
    }

    pub fn get<CoreApp>(
        &mut self,
        id: u64,
        engine: &mut RenderEngine<CoreApp>,
    ) -> Arc<DebugCell<Scene>>
    where
        CoreApp: ApplicationTrait,
    {
        let file = self.cached.get_file().clone();
        let engine: usize = unsafe { transmute(engine) };
        self.cached.get(id, &|| {
            let engine: &mut RenderEngine<CoreApp> = unsafe { transmute(engine) };
            Arc::new(DebugCell::new(BasicScene::new(&file, engine)))
        })
    }
}
