use std::ptr::null_mut;
use super::super::core::application::ApplicationTrait;
use super::super::system::os::OsApplication;

#[derive(Debug)]
pub struct Texture2D {
}

impl Texture2D {
    pub fn new<CoreApp>(data: Vec<u8>, os_app: *mut OsApplication<CoreApp>) -> Self
            where CoreApp: ApplicationTrait {
        Texture2D {
        }
    }
}
