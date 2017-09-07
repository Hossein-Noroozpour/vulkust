use std::collections::BTreeMap;
use std::io::{Seek, SeekFrom};
use std::mem::transmute;
use std::sync::{Arc, Weak};
use super::super::system::file::File;
use super::cell::DebugCell;

pub struct Cacher<ID, VAL: ?Sized> where ID: Ord {
    cached: BTreeMap<ID, Weak<DebugCell<VAL>>>,
}

impl<ID, VAL: ?Sized> Cacher<ID, VAL> where ID: Ord {
    pub fn new() -> Self {
        Cacher {
            cached: BTreeMap::new(),
        }
    }

    pub fn get<F>(&mut self, id: ID, new: &F) -> Arc<DebugCell<VAL>>
    where F: Fn() -> Arc<DebugCell<VAL>> {
        match self.cached.get(&id) {
            Some(res) => match res.upgrade() {
                Some(res) => {
                    return res;
                }
                None => {}
            },
            None => {}
        }
        let res = new();
        self.cached.insert(id, Arc::downgrade(&res));
        return res;
    }
}

pub struct FileCacher<VAL: ?Sized> {
    file: Arc<DebugCell<File>>,
    cached: Cacher<u64, VAL>,
    offsets: Vec<u64>,
}

impl<VAL: ?Sized> FileCacher<VAL> {
    pub fn new(file: Arc<DebugCell<File>>) -> Self {
        FileCacher {
            file: file,
            cached: Cacher::new(),
            offsets: Vec::new(),
        }
    }

    pub fn read_offsets(&mut self) {
        let mut file = self.file.borrow_mut();
        let count = file.read_count() as usize;
        self.offsets.resize(count, 0);
        for i in 0..count {
            self.offsets[i] = file.read_offset();
        }
    }

    pub fn get<'a, F>(
        &'a mut self, id: u64, new: &F
    ) -> Arc<DebugCell<VAL>>
    where F: Fn() -> Arc<DebugCell<VAL>> {
        let self_ptr: &'static usize = unsafe { transmute(&self) };
        let self2 = *self_ptr;
        self.cached.get(id, &|| {
            let self2: &'a mut FileCacher<VAL> = unsafe { transmute(self2) };
            #[cfg(cacher_debug)]
            {
                if id as usize > self2.offsets.len() {
                    logf!("Id is is out of the range.");
                }
                let offset = self2.offsets[id as usize];
                match self2.file.borrow_mut().seek(SeekFrom::Start(offset)) {
                    Ok(o) => if o < offset {
                        logf!("Seeked offset does not match!");
                    },
                    _ => {
                        logf!("Can not seek to the requested offset.");
                    }
                };
            }
            #[cfg(not(cacher_debug))] 
            {
                let _ = self2.file.borrow_mut().seek(SeekFrom::Start(self2.offsets[id as usize])).unwrap();
            }
            new()
        })
    }

    pub fn get_file(&self) -> &Arc<DebugCell<File>> {
        &self.file
    }
}
