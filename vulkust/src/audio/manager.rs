use std::sync::Arc;
use super::super::system::file::File;
use super::super::util::cache::FileCacher;
use super::super::util::cell::DebugCell;
use super::{Audio, Music, Voice};

pub struct Manager {
    pub cached: FileCacher<Audio>,
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

    pub fn get(&mut self, id: u64) -> Arc<DebugCell<Audio>> {
        let file = self.cached.get_file().clone();
        self.cached.get(id, &|| {
            let audio_type: u64 = file.borrow_mut().read_type();
            match audio_type {
                10 => Arc::new(DebugCell::new(Music::new(&file))),
                20 => Arc::new(DebugCell::new(Voice::new(&file))),
                _ => {
                    logf!("Uexpected value");
                }
            }
        })
    }
}
