pub mod manager;
pub mod stage;

use std::fmt::Debug;
use super::super::system::file::File;
use super::super::system::os::OsApplication;
use super::super::core::application::ApplicationTrait;
use self::stage::Stage;

pub trait ShaderTrait: Debug {
    fn as_shader(&self) -> &Shader {
        logf!("This object can not convert to Shader.");
    }
}

#[derive(Debug)]
pub struct Shader {
    pub vertex: Stage,
    pub fragment: Stage,
}

impl Shader {
    pub fn new<CoreApp>(file: &mut File, os_app: *mut OsApplication<CoreApp>) -> Self
    where
        CoreApp: ApplicationTrait,
    {
        let size: u64 = file.read_type();
        logi!("shader size is: {}", size);
        let vertex = file.read_bytes(size as usize);
        let size: u64 = file.read_type();
        logi!("shader size is: {}", size);
        let fragment = file.read_bytes(size as usize);
        Shader {
            vertex: Stage::new(vertex, os_app),
            fragment: Stage::new(fragment, os_app),
        }
    }
}

impl ShaderTrait for Shader {
    fn as_shader(&self) -> &Shader {
        return self;
    }
}
