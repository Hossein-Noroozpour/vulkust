pub mod manager;

use super::core::application::ApplicationTrait;
use super::system::os::OsApplication;
use super::system::file::File;

pub trait Audio {}

struct Basic {
    pub file_content: Vec<u8>
}

impl Basic {
    pub fn new<CoreApp>(
        file: &mut File,
        os_app: *mut OsApplication<CoreApp>,
    ) -> Self
    where CoreApp: ApplicationTrait {
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
    pub fn new<CoreApp>(
        file: &mut File,
        os_app: *mut OsApplication<CoreApp>,
    ) -> Self
    where CoreApp: ApplicationTrait {
        Music {
            b: Basic::new(file, os_app),
        }
    }
}

impl Audio for Music {}

pub struct Voice {
    b: Basic,
}

impl Voice {
    pub fn new<CoreApp>(
        file: &mut File,
        os_app: *mut OsApplication<CoreApp>,
    ) -> Self
    where CoreApp: ApplicationTrait {
        Voice {
            b: Basic::new(file, os_app),
        }
    }
}

impl Audio for Voice {}
