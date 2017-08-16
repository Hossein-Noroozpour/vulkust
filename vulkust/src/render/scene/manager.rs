use std::cell::RefCell;
use std::collections::BTreeMap;
use std::sync::{Arc, Weak};
use std::io::{Seek, SeekFrom};
use super::super::super::system::file::File;
use super::super::super::core::application::ApplicationTrait;
use super::super::engine::RenderEngine;
use super::{BasicScene, Scene};

pub struct Manager {
    pub cached: BTreeMap<u64, Weak<RefCell<Scene>>>,
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
        let count = file.read_count();
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
    ) -> Arc<RefCell<Scene>>
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
        #[cfg(scene_import_debug)]
        logi!("scene with id {} has offset {}", id, offset);
        match file.seek(SeekFrom::Start(offset)) {
            Ok(o) => if o < offset {
                logf!("Seeked offset does not match!");
            },
            _ => {
                logf!("Can not seek to the requested offset.");
            }
        }
        let scene = RefCell::new(BasicScene::new(file, engine));
        let scene: Arc<RefCell<Scene>> = Arc::new(scene);
        self.cached.insert(id, Arc::downgrade(&scene));
        return scene;
    }
}
