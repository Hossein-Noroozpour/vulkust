use std::collections::BTreeMap;
use std::sync::{Weak, Arc};
use std::io::{Seek, SeekFrom};
use super::super::core::application::ApplicationTrait;
use super::super::system::os::OsApplication;
use super::super::system::file::File;
use super::{Audio};

pub struct Manager {
    pub cached: BTreeMap<u64, Weak<Audio>>,
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
        let count: u64 = file.read_type();
        self.offsets.resize(count as usize, 0);
        for i in 0..count as usize {
            self.offsets[i] = file.read_type();
        }
    }
}
