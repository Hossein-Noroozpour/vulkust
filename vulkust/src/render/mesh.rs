use std::sync::Arc;
use std::cell::RefCell;
use super::super::core::application::ApplicationTrait;
use super::super::math::matrix::Mat4x4;
use super::super::system::os::OsApplication;
use super::super::system::file::File;
use super::material::read_material;

pub struct Mesh {

}

impl Mesh {
    pub fn new<CoreApp>(file: &mut File, os_app: &mut OsApplication<CoreApp>) -> Self
    where
        CoreApp: ApplicationTrait,
    {
        let material = read_material(file, os_app);
        logf!("Unimplmented");
    }
}
