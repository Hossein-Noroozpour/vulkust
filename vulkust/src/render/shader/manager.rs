use std::collections::BTreeMap;
use std::sync::Arc;
use std::io::{Seek, SeekFrom};
use super::super::super::system::file::File;
use super::super::super::util::cache::Cacher;
use super::super::super::util::cell::DebugCell;
use super::super::device::logical::Logical as LogicalDevice;
use super::{read_id, Shader, TwoStage};

pub struct Manager {
    pub file: Arc<DebugCell<File>>,
    pub cached: Cacher<u64, Shader>,
    pub offsets: BTreeMap<u64, u64>,
}

impl Manager {
    pub fn new(file: Arc<DebugCell<File>>) -> Self {
        Manager {
            file: file,
            cached: Cacher::new(),
            offsets: BTreeMap::new(),
        }
    }

    pub fn read_table(&mut self) {
        let count = self.file.borrow_mut().read_count();
        for _ in 0..count {
            let id = read_id(&self.file);
            let offset = self.file.borrow_mut().read_offset();
            #[cfg(shader_import_debug)]
            logi!("Shader with id: {} and offset {} loaded.", id, offset);
            self.offsets.insert(id, offset);
        }
    }

    pub fn get(
        &mut self,
        id: u64,
        logical_device: Arc<LogicalDevice>,
    ) -> Arc<DebugCell<Shader>> {
        self.cached.get(id, &|| {
                match self.offsets.get(&id) {
                    Some(offset) => {
                        match self.file.borrow_mut().seek(SeekFrom::Start(*offset)) {
                            Ok(o) => if o < *offset {
                                logf!("Seeked offset does not match!");
                            },
                            _ => {
                                logf!("Can not seek to the requested offset.");
                            }
                        };
                    },
                    None => {
                        logf!("Requested shader {} does not exist.", id);
                    }
                };
                Arc::new(DebugCell::new(TwoStage::new(&self.file, logical_device))) 
            }
        )
    }
}
