pub mod manager;
pub mod stage;

use std::fmt::Debug;
use std::mem::transmute;
use super::super::system::file::File;
use super::super::system::os::OsApplication;
use super::super::core::application::ApplicationTrait;
use self::stage::Stage;

pub trait ShaderTrait: Debug {
}

#[derive(Debug)]
pub struct Shader {
    pub vertex: Stage,
    pub fragment: Stage,
}

impl Shader {
    pub fn new<CoreApp>(file: &mut File, os_app: *mut OsApplication<CoreApp>) -> Self
            where CoreApp: ApplicationTrait {
        let device = unsafe { (*os_app).metal_device };
        let size: u64 = file.read_type();
        let vertex = file.read_bytes(size as usize);
        let size: u64 = file.read_type();
        let fragment = file.read_bytes(size as usize);
        Shader {
            vertex: Stage::new(vertex, device),
            fragment: Stage::new(fragment, device),
        }
    }
}

impl ShaderTrait for Shader {
}
