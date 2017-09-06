use std::sync::Arc;
use super::super::super::system::file::File;
use super::super::super::util::cache::FileCacher;
use super::super::super::util::cell::DebugCell;
use super::{Id, Texture, Texture2D};

pub struct Manager {
    pub cached: FileCacher<Texture>,
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

    pub fn get(&mut self, id: Id) -> Arc<DebugCell<Texture>> {
        self.cached.get(id, &|| {
            let type_id = self.cached.get_file().borrow_mut().read_id();
            match type_id {
                10 => Arc::new(DebugCell::new(Texture2D::new(self.cached.get_file()))),
                _ => {
                    logf!(
                        "{} {} {} {} {}",
                        "Requsted texture with Id:",
                        id,
                        "found but type:",
                        type_id,
                        "is not implemented yet."
                    );
                }
            }
        })
    }
}
