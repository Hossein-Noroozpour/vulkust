pub mod manager;

use super::super::core::application::ApplicationTrait;
use super::super::system::os::OsApplication;
use super::super::system::file::File;

pub trait Model {}

pub struct Basic {}

impl Basic {
    pub fn new<CoreApp>(file: &mut File, os_app: *mut OsApplication<CoreApp>) -> Self
    where CoreApp: ApplicationTrait {
        logf!("Not implemented");
    }
}

impl Model for Basic {}
