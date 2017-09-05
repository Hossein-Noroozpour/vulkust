use std::collections::BTreeMap;
use std::sync::{Arc, Weak};
use std::io::{Seek, SeekFrom};
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
    cached: Cacher<u64, VAL>,
    offsets: Vec<u64>,
}

impl<VAL: ?Sized> FileCacher<VAL> {
    pub fn new() -> Self {
        FileCacher {
            cached: Cacher::new(),
            offsets: Vec::new(),
        }
    }

    pub fn read_offsets(&mut self, file: &Arc<DebugCell<File>>) {
        let file = file.borrow_mut();
        let count = file.read_count() as usize;
        self.offsets.resize(count, 0);
        for i in 0..count {
            self.offsets[i] = file.read_offset();
        }
    }

    pub fn get<F>(
        &mut self, id: u64, file: &Arc<DebugCell<File>>, new: &F
    ) -> Arc<DebugCell<VAL>>
    where F: Fn() -> Arc<DebugCell<VAL>> {
        self.cached.get(id, &|| {
            #[cfg(cacher_debug)]
            {
                if id as usize > self.offsets.len() {
                    logf!("Id is is out of the range.");
                }
                let offset = self.offsets[id as usize];
                match file.borrow_mut().seek(SeekFrom::Start(offset)) {
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
                let _ = file.borrow_mut().seek(SeekFrom::Start(self.offsets[id as usize]));
            }
            new()
        })
    }
}
