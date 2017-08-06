use std::sync::Arc;
use std::cell::RefCell;
use super::super::core::application::ApplicationTrait;
use super::super::math::matrix::Mat4x4;
use super::super::system::os::OsApplication;
use super::super::system::file::File;
use super::shader::{Id as ShaderId, read_id, Shader};

pub const WHITE_ID: ShaderId = 0;
pub const DIRECTIONAL_TEXTURED_SPECULATED_NOCUBE_FULLSHADOW_OPAQUE_ID: ShaderId = 2207629967616;

pub trait Material {}

pub struct White {
    pub shader: Arc<Shader>,
}

impl White {
    pub fn new<CoreApp>(file: &mut File, os_app: *mut OsApplication<CoreApp>) -> Self
    where CoreApp: ApplicationTrait {
        let shader = unsafe { (*os_app).asset_manager.get_shader(WHITE_ID, os_app) };
        White {
            shader: shader,
        }
    }
}

impl Material for White {}

pub struct DirectionalTexturedSpeculatedNocubeFullshadowOpaque {}

impl DirectionalTexturedSpeculatedNocubeFullshadowOpaque {
    pub fn new() -> Self {
        logf!("Unimplemented");
        DirectionalTexturedSpeculatedNocubeFullshadowOpaque {
        }
    }
}

impl Material for DirectionalTexturedSpeculatedNocubeFullshadowOpaque {}

pub fn read_material<CoreApp>(file: &mut File, os_app: *mut OsApplication<CoreApp>) ->
        Arc<RefCell<Material>> where CoreApp: ApplicationTrait {
    let shader_id = read_id(file);
    if shader_id == WHITE_ID {
        return Arc::new(RefCell::new(White::new(file, os_app)));
    }
    if shader_id == DIRECTIONAL_TEXTURED_SPECULATED_NOCUBE_FULLSHADOW_OPAQUE_ID {
        return Arc::new(RefCell::new(DirectionalTexturedSpeculatedNocubeFullshadowOpaque::new()));
    }
    logf!("Unexpected shader id!");
}
