use std::default::Default;
use std::sync::Arc;
use std::mem::{size_of, transmute};
use super::super::core::application::ApplicationTrait;
use super::super::math::matrix::Mat4x4;
use super::super::math::vector::Vec3;
use super::super::system::file::File;
use super::super::util::cell::DebugCell;
use super::buffer::{SceneDynamics, UniformBuffer};
use super::engine::RenderEngine;
use super::model::UniformData as MdlUniData;
use super::pipeline::Pipeline;
use super::scene::UniformData as ScnUniData;
use super::shader;
use super::shader::{read_id, Id as ShaderId};
use super::texture::Texture;

pub const FLOAT_SIZE: u64 = 4;
pub const POSITION_ELEMENT: u64 = FLOAT_SIZE * 3;
pub const NORMAL_ELEMENT: u64 = FLOAT_SIZE * 3;
pub const UV_ELEMENT: u64 = FLOAT_SIZE * 2;
pub const POSITION_VERTEX_SIZE: u64 = POSITION_ELEMENT;
pub const POSITION_NORMAL_VERTEX_SIZE: u64 = POSITION_ELEMENT + NORMAL_ELEMENT;
pub const POSITION_UV_VERTEX_SIZE: u64 = POSITION_ELEMENT + UV_ELEMENT;
pub const POSITION_NORMAL_UV_VERTEX_SIZE: u64 = POSITION_ELEMENT + NORMAL_ELEMENT + UV_ELEMENT;

pub trait Material {
    fn update_uniform(&mut self, sud: &ScnUniData, mud: &MdlUniData, frame_index: usize);
    fn get_pipeline(&self) -> &Arc<DebugCell<Pipeline>>;
    fn get_vertex_size(&self) -> usize;
    fn init_uniforms(
        &mut self, scene_dynamics: &Vec<Arc<DebugCell<SceneDynamics>>>);
}

struct Base {
    uniform_size: usize,
    uniforms: Vec<Arc<DebugCell<UniformBuffer>>>,
    pipeline: Arc<DebugCell<Pipeline>>,
}

impl Base {
    fn new<CoreApp>(
        uniform_size: usize,
        shader_id: ShaderId,
        engine: &mut RenderEngine<CoreApp>
    ) -> Self where CoreApp: ApplicationTrait {
        Base {
            uniform_size: uniform_size,
            uniforms: Vec::new(),
            pipeline: engine.pipeline_manager.as_mut().unwrap().borrow_mut().get(shader_id),
        }
    }

    fn init_uniforms(
        &mut self, 
        scene_dynamics: &Vec<Arc<DebugCell<SceneDynamics>>>) {
        self.uniforms.clear();
        for sd in scene_dynamics {
            self.uniforms.push(sd.borrow_mut().create_uniform(self.uniform_size));
        }
    }
}

pub struct DirectionalTexturedSpeculatedNocubeFullshadowOpaque {
    texture: Arc<DebugCell<Texture>>,
    base: Base,
    uniform_data: DirectionalTexturedSpeculatedNocubeFullshadowOpaqueUniform,
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
    pub fn new<CoreApp>(
        file: &Arc<DebugCell<File>>,
        engine: &mut RenderEngine<CoreApp>
    ) -> Self where CoreApp: ApplicationTrait {
        let texture_id = file.borrow_mut().read_id();
        let offset = file.borrow_mut().tell();
        let texture = engine.os_app.asset_manager.get_texture(texture_id);
        let shader = engine.os_app.asset_manager.get_shader(
            shader::DIRECTIONAL_TEXTURED_SPECULATED_NOCUBE_FULLSHADOW_OPAQUE_ID,
            engine.logical_device.as_ref().unwrap().clone(),
        );
        file.borrow_mut().goto(offset);
        let speculation_color = Vec3::new_from_file(file);
        let speculation_intensity = file.borrow_mut().read_type();
        #[cfg(material_debug)]
        {
            logi!("speculation_color: {:?}", speculation_color);
            logi!("speculation_intensity: {}", speculation_intensity);
        }
        let mut uni = DirectionalTexturedSpeculatedNocubeFullshadowOpaqueUniform::default();
        uni.spec_color = speculation_color;
        uni.spec_intensity = speculation_intensity;
        DirectionalTexturedSpeculatedNocubeFullshadowOpaque {
            texture: texture,
            base: Base::new(
                size_of::<DirectionalTexturedSpeculatedNocubeFullshadowOpaqueUniform>(),
                shader::DIRECTIONAL_TEXTURED_SPECULATED_NOCUBE_FULLSHADOW_OPAQUE_ID,
                engine
            ),
            uniform_data: DirectionalTexturedSpeculatedNocubeFullshadowOpaqueUniform::default(),
        }
    }
}

impl Material for DirectionalTexturedSpeculatedNocubeFullshadowOpaque {
    fn update_uniform(&mut self, sud: &ScnUniData, mud: &MdlUniData, frame_index: usize) {
        self.uniform_data.mvp = mud.mvp;
        self.uniform_data.transform = mud.m;
        self.uniform_data.eye_loc = sud.eye_loc;
        self.uniform_data.sun_dir = sud.sun_dir;
        self.base.uniforms[frame_index].borrow_mut().upload(
            unsafe { transmute(&self.uniform_data) });
    }

    fn get_pipeline(&self) -> &Arc<DebugCell<Pipeline>> {
        &self.base.pipeline
    }

    fn get_vertex_size(&self) -> usize {
        POSITION_NORMAL_UV_VERTEX_SIZE as usize
    }

    fn init_uniforms(
        &mut self, 
        scene_dynamics: &Vec<Arc<DebugCell<SceneDynamics>>>) {
        self.base.init_uniforms(scene_dynamics);
    }
}

#[repr(C)]
#[derive(Default)]
struct WhiteUniform {
    pub mvp: Mat4x4<f32>,
}

pub struct White {
    uniform_data: WhiteUniform,
    base: Base,
}

impl White {
    pub fn new<CoreApp>(
        engine: &mut RenderEngine<CoreApp>
    ) -> Self where CoreApp: ApplicationTrait {
        White { 
            uniform_data: WhiteUniform::default(),
            base: Base::new(
                size_of::<WhiteUniform>(),
                shader::WHITE_ID,
                engine),
        }
    }
}

impl Material for White {
    fn update_uniform(&mut self, sud: &ScnUniData, mud: &MdlUniData, frame_index: usize) {
        self.uniform_data.mvp = mud.mvp;
        self.base.uniforms[frame_index].borrow_mut().upload(
            unsafe { transmute(&self.uniform_data) });
    }

    fn get_pipeline(&self) -> &Arc<DebugCell<Pipeline>> {
        &self.base.pipeline
    }

    fn get_vertex_size(&self) -> usize {
        POSITION_VERTEX_SIZE as usize
    }

    fn init_uniforms(
        &mut self, 
        scene_dynamics: &Vec<Arc<DebugCell<SceneDynamics>>>) {
        self.base.init_uniforms(scene_dynamics);
    }
}

pub fn read_material<CoreApp>(
    file: &Arc<DebugCell<File>>,
    engine: &mut RenderEngine<CoreApp>
) -> Arc<DebugCell<Material>> 
where CoreApp: ApplicationTrait {
    let shader_id = read_id(file);
    return match shader_id {
        WHITE_ID => {
            logf!("This shader must not be send to material");
        }
        DIRECTIONAL_TEXTURED_SPECULATED_NOCUBE_FULLSHADOW_OPAQUE_ID => Arc::new(DebugCell::new(
            DirectionalTexturedSpeculatedNocubeFullshadowOpaque::new(
                file,
                engine,
            ),
        )),
        _ => {
            logf!("Unexpected shader id!");
        }
    };
}
