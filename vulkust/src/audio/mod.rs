pub mod manager;

use super::system::file::File;

pub trait Audio {}

struct Basic {
    pub file_content: Vec<u8>,
}

impl Basic {
    pub fn new(file: &mut File) -> Self {
        let size: u64 = file.read_type();
        let file_content = file.read_bytes(size as usize);
        Basic {
            file_content: file_content,
        }
    }
}

pub struct Music {
    b: Basic,
}

impl Music {
    pub fn new(file: &mut File) -> Self {
        Music {
            b: Basic::new(file),
        }
    }
}

impl Audio for Music {}

pub struct Voice {
    b: Basic,
}

impl Voice {
    pub fn new(file: &mut File) -> Self {
        Voice {
            b: Basic::new(file),
        }
    }
}

impl Audio for Voice {}
