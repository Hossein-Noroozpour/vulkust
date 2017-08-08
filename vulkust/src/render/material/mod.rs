use std::sync::Arc;
use std::cell::RefCell;
use std::mem::transmute;
use super::super::core::application::ApplicationTrait;
use super::super::math::matrix::Mat4x4;
use super::super::math::vector::Vec3;
use super::super::system::os::OsApplication;
use super::super::system::file::File;
use super::shader::{Id as ShaderId, read_id, Shader};
use super::texture::Texture;

pub const FLOAT_SIZE: u64 = 4;
pub const POSITION_ELEMENT: u64 = FLOAT_SIZE * 3;
pub const NORMAL_ELEMENT: u64 = FLOAT_SIZE * 3;
pub const UV_ELEMENT: u64 = FLOAT_SIZE * 2;
pub const POSITION_VERTEX_SIZE: u64 = POSITION_ELEMENT;
pub const POSITION_NORMAL_VERTEX_SIZE: u64 = POSITION_ELEMENT + NORMAL_ELEMENT;
pub const POSITION_UV_VERTEX_SIZE: u64 = POSITION_ELEMENT + UV_ELEMENT;
pub const POSITION_NORMAL_UV_VERTEX_SIZE: u64 = POSITION_ELEMENT + NORMAL_ELEMENT + UV_ELEMENT;
pub const WHITE_ID: ShaderId = 0;
pub const DIRECTIONAL_TEXTURED_SPECULATED_NOCUBE_FULLSHADOW_OPAQUE_ID: ShaderId = 2207629967616;

pub trait Material {
    fn get_vertex_size(&self) -> u64;
}

pub struct White {
    pub shader: Arc<Shader>,
}

impl White {
    pub fn new<CoreApp>(file: &mut File, os_app: &mut OsApplication<CoreApp>) -> Self
    where CoreApp: ApplicationTrait {
        let device = os_app.render_engine.logical_device.as_ref().unwrap().clone();
        let shader = os_app.asset_manager.get_shader(WHITE_ID, device);
        White {
            shader: shader,
        }
    }
}

impl Material for White {
    fn get_vertex_size(&self) -> u64 {
        return POSITION_VERTEX_SIZE;
    }
}

pub struct DirectionalTexturedSpeculatedNocubeFullshadowOpaque {
    pub shader: Arc<Shader>,
    pub texture: Arc<Texture>,
    pub speculation_color: Vec3<f32>,
    pub speculation_intensity: f32,
}

impl DirectionalTexturedSpeculatedNocubeFullshadowOpaque {
    pub fn new<CoreApp>(file: &mut File, os_app: &mut OsApplication<CoreApp>) -> Self
    where CoreApp: ApplicationTrait {
        let device = os_app.render_engine.logical_device.as_ref().unwrap().clone();
        let shader = os_app.asset_manager.get_shader(
            DIRECTIONAL_TEXTURED_SPECULATED_NOCUBE_FULLSHADOW_OPAQUE_ID, device);
        let texture = os_app.asset_manager.get_texture(
            file.read_id(), os_app);
        let speculation_color = Vec3::new_from_file(file);
        let speculation_intensity = file.read_type();
        DirectionalTexturedSpeculatedNocubeFullshadowOpaque {
            shader: shader,
            texture: texture,
            speculation_color: speculation_color,
            speculation_intensity: speculation_intensity,
        }
    }
}

impl Material for DirectionalTexturedSpeculatedNocubeFullshadowOpaque {
    fn get_vertex_size(&self) -> u64 {
        return POSITION_NORMAL_UV_VERTEX_SIZE;
    }
}

pub fn read_material<CoreApp>(file: &mut File, os_app: &mut OsApplication<CoreApp>) ->
        Arc<RefCell<Material>> where CoreApp: ApplicationTrait {
    let shader_id = read_id(file);
    return match shader_id {
        WHITE_ID => Arc::new(RefCell::new(White::new(file, os_app))),
        DIRECTIONAL_TEXTURED_SPECULATED_NOCUBE_FULLSHADOW_OPAQUE_ID =>
            Arc::new(RefCell::new(
                DirectionalTexturedSpeculatedNocubeFullshadowOpaque::new(file, os_app))),
        _ => {logf!("Unexpected shader id!");},
    };
}
