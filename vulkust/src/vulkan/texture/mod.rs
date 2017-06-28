use super::super::core::application::ApplicationTrait;
use super::super::system::os::OsApplication;

// TODO

#[derive(Debug)]
pub struct Texture2D {}

impl Texture2D {
    pub fn new<CoreApp>(_data: Vec<u8>, _os_app: *mut OsApplication<CoreApp>) -> Self
    where
        CoreApp: ApplicationTrait,
    {
        Texture2D {}
    }
}
