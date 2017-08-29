use std::default::Default;
use std::sync::Arc;
use super::super::math::matrix::Mat4x4;
use super::super::math::vector::Vec3;
use super::super::system::file::File;
use super::super::util::cell::DebugCell;
use super::device::logical::Logical as LogicalDevice;
use super::descriptor::Set as DescriptorSet;
use super::buffer::Manager as BufferManager;
use super::model::UniformData as MdlUniData;
use super::pipeline::Pipeline;
use super::scene::UniformData as ScnUniData;
use super::shader;
use super::shader::{read_id, Shader};
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

pub trait Material {
    fn update_uniform(&self, sud: &ScnUniData, mud: &MdlUniData, frame_index: usize);
}

pub struct DirectionalTexturedSpeculatedNocubeFullshadowOpaque {
    pub shader: Arc<Shader>,
    pub texture: Arc<Texture>,
    pub uniforms: Vec<&'static mut DirectionalTexturedSpeculatedNocubeFullshadowOpaqueUniform>,
    pub uniforms_ranges: Vec<(usize, usize)>,
    pub pipeline: Option<Arc<Pipeline>>,
    pub descriptor_set: Option<Arc<DescriptorSet>>,
}

#[repr(C)]
#[derive(Default)]
struct DirectionalTexturedSpeculatedNocubeFullshadowOpaqueUniform {
    pub mvp: Mat4x4<f32>,
    pub transform: Mat4x4<f32>,
    pub eye_loc: Vec3<f32>,
    pub sun_dir: Vec3<f32>,
    pub spec_color: Vec3<f32>,
    pub spec_intensity: f32,
}

impl DirectionalTexturedSpeculatedNocubeFullshadowOpaque {
    pub fn new(
        file: &mut File,
        logical_device: Arc<LogicalDevice>,
        shader_manager: &mut ShaderManager,
        texture_manager: &mut TextureManager,
        buffer_manager: &mut BufferManager,
    ) -> Self {
        let texture_id = file.read_id();
        let offset = file.tell();
        let texture = texture_manager.get(texture_id, file);
        let shader = shader_manager.get(
            shader::DIRECTIONAL_TEXTURED_SPECULATED_NOCUBE_FULLSHADOW_OPAQUE_ID,
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
        let mut uni = DirectionalTexturedSpeculatedNocubeFullshadowOpaqueUniform::default();
        uni.spec_color = speculation_color;
        uni.spec_intensity = speculation_intensity;
        let (uniforms, uniforms_ranges) = buffer_manager.add_u(&uni);
        DirectionalTexturedSpeculatedNocubeFullshadowOpaque {
            shader: shader,
            texture: texture,
            uniforms: uniforms,
            uniforms_ranges: uniforms_ranges,
            pipeline: None, // TODO
            descriptor_set: None, // TODO
        }
    }
}

impl Material for DirectionalTexturedSpeculatedNocubeFullshadowOpaque {
    fn update_uniform(&self, sud: &ScnUniData, mud: &MdlUniData, frame_index: usize) {
        self.uniforms[frame_index].mvp = mud.mvp;
        self.uniforms[frame_index].transform = mud.m;
        self.uniforms[frame_index].eye_loc = sud.eye_loc;
        self.uniforms[frame_index].sun_dir = sud.sun_dir;
    }
}

pub struct White {
    pub shader: Arc<Shader>,
    pub pipeline: Option<Arc<Pipeline>>,
    pub descriptor_set: Option<Arc<DescriptorSet>>,
}

impl White {
    pub fn new(
        file: &mut File,
        logical_device: Arc<LogicalDevice>,
        shader_manager: &mut ShaderManager,
    ) -> Self {
        let shader = shader_manager.get(shader::WHITE_ID, file, logical_device);
        White { 
            shader: shader,
            pipeline: None, // TODO
            descriptor_set: None, // TODO
        }
    }
}

impl Material for White {
    fn update_uniform(&self, _sud: &ScnUniData, _mud: &MdlUniData, _frame_index: usize) {
        logf!("White shader does not implement this function because this is special!!!");
    }
}

pub fn read_material(
    file: &mut File,
    logical_device: Arc<LogicalDevice>,
    shader_manager: &mut ShaderManager,
    texture_manager: &mut TextureManager,
    buffer_manager: &mut BufferManager
) -> Arc<DebugCell<Material>> {
    let shader_id = read_id(file);
    return match shader_id {
        WHITE_ID => {
            logf!("This shader must not be send to material");
        }
        DIRECTIONAL_TEXTURED_SPECULATED_NOCUBE_FULLSHADOW_OPAQUE_ID => Arc::new(DebugCell::new(
            DirectionalTexturedSpeculatedNocubeFullshadowOpaque::new(
                file,
                logical_device,
                shader_manager,
                texture_manager,
                buffer_manager,
            ),
        )),
        _ => {
            logf!("Unexpected shader id!");
        }
    };
}
