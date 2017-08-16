use std::sync::Arc;
use std::cell::RefCell;
use super::super::math::vector::Vec3;
use super::super::system::file::File;
use super::device::logical::Logical as LogicalDevice;
use super::shader::{read_id, Id as ShaderId, Shader};
use super::shader::manager::Manager as ShaderManager;
use super::texture::Texture;
use super::texture::manager::Manager as TextureManager;
use super::vertex::Attribute as VertexAttribute;

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
    fn get_vertex_attributes(&self) -> Vec<VertexAttribute>;
    fn get_shader(&self) -> &Arc<Shader>;
}

pub struct DirectionalTexturedSpeculatedNocubeFullshadowOpaque {
    pub shader: Arc<Shader>,
    pub texture: Arc<Texture>,
    pub speculation_color: Vec3<f32>,
    pub speculation_intensity: f32,
}

impl DirectionalTexturedSpeculatedNocubeFullshadowOpaque {
    pub fn new(
        file: &mut File,
        logical_device: Arc<LogicalDevice>,
        shader_manager: &mut ShaderManager,
        texture_manager: &mut TextureManager,
    ) -> Self {
        let texture_id = file.read_id();
        let offset = file.tell();
        let texture = texture_manager.get(texture_id, file);
        let shader = shader_manager.get(
            DIRECTIONAL_TEXTURED_SPECULATED_NOCUBE_FULLSHADOW_OPAQUE_ID,
            file,
            logical_device,
        );
        file.goto(offset);
        let speculation_color = Vec3::new_from_file(file);
        let speculation_intensity = file.read_type();
        #[cfg(material_debug)]
        {
            logi!("speculation_color: {:?}", speculation_color);
            logi!("speculation_intensity: {}", speculation_intensity);
        }
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

    fn get_vertex_attributes(&self) -> Vec<VertexAttribute> {
        return vec![
            VertexAttribute::Vec3F32,
            VertexAttribute::Vec3F32,
            VertexAttribute::Vec2F32,
        ];
    }

    fn get_shader(&self) -> &Arc<Shader> {
        &self.shader
    }
}

pub struct White {
    pub shader: Arc<Shader>,
}

impl White {
    pub fn new(
        file: &mut File,
        logical_device: Arc<LogicalDevice>,
        shader_manager: &mut ShaderManager,
    ) -> Self {
        let shader = shader_manager.get(WHITE_ID, file, logical_device);
        White { shader: shader }
    }
}

impl Material for White {
    fn get_vertex_size(&self) -> u64 {
        return POSITION_VERTEX_SIZE;
    }

    fn get_vertex_attributes(&self) -> Vec<VertexAttribute> {
        return vec![VertexAttribute::Vec3F32];
    }

    fn get_shader(&self) -> &Arc<Shader> {
        &self.shader
    }
}

pub fn read_material(
    file: &mut File,
    logical_device: Arc<LogicalDevice>,
    shader_manager: &mut ShaderManager,
    texture_manager: &mut TextureManager,
) -> Arc<RefCell<Material>> {
    let shader_id = read_id(file);
    return match shader_id {
        WHITE_ID => {
            logf!("This shader must not be send to material");
        }
        DIRECTIONAL_TEXTURED_SPECULATED_NOCUBE_FULLSHADOW_OPAQUE_ID => Arc::new(RefCell::new(
            DirectionalTexturedSpeculatedNocubeFullshadowOpaque::new(
                file,
                logical_device,
                shader_manager,
                texture_manager,
            ),
        )),
        _ => {
            logf!("Unexpected shader id!");
        }
    };
}
