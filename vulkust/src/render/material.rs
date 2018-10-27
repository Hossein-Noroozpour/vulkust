use super::super::core::types::{Id, TypeId};
use super::buffer::DynamicBuffer;
use super::command::Buffer as CmdBuffer;
use super::descriptor::Set as DescriptorSet;
use super::engine::Engine;
use super::gx3d::Gx3DReader;
use super::model::Model;
use super::scene::Scene;
use super::texture::{Manager as TextureManager, Texture};
use std::default::Default;
use std::mem::size_of;
use std::sync::{Arc, RwLock};

use gltf;

#[repr(u8)]
#[cfg_attr(debug_mode, derive(Debug))]
pub enum Field {
    Float = 1,
    Texture = 2,
    Vector = 3,
}

#[repr(C)]
#[cfg_attr(debug_mode, derive(Debug))]
pub struct Uniform {
    pub alpha: f32,
    pub alpha_cutoff: f32,
    pub metallic_factor: f32,
    pub normal_scale: f32,
    pub occlusion_strength: f32,
    pub roughness_factor: f32,
}

impl Uniform {
    pub fn new() -> Self {
        Uniform {
            metallic_factor: 1.0,
            roughness_factor: 1.0,
            normal_scale: 1.0,
            occlusion_strength: 1.0,
            alpha: 1.0,
            alpha_cutoff: 1.0 / 256.0,
        }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub enum TranslucencyMode {
    Cutoff,
    Tansparent,
    Opaque,
}

impl Default for TranslucencyMode {
    fn default() -> Self {
        TranslucencyMode::Opaque
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Material {
    pub base_color: Arc<RwLock<Texture>>,
    pub base_color_factor: Arc<RwLock<Texture>>,
    pub metallic_roughness: Arc<RwLock<Texture>>,
    pub normal: Arc<RwLock<Texture>>,
    pub occlusion: Arc<RwLock<Texture>>,
    pub emissive: Arc<RwLock<Texture>>,
    pub emissive_factor: Arc<RwLock<Texture>>,
    pub translucency: TranslucencyMode,
    pub uniform: Uniform,
    uniform_buffer: DynamicBuffer,
    descriptor_set: Arc<DescriptorSet>,
}

impl Material {
    pub(crate) fn new_with_gltf(_engine: &Engine, _mat: &gltf::Material) -> Self {
        vxunimplemented!();
    }

    pub(crate) fn get_uniform_buffer(&self) -> &DynamicBuffer {
        return &self.uniform_buffer;
    }

    pub(crate) fn get_descriptor_set(&self) -> &Arc<DescriptorSet> {
        return &self.descriptor_set;
    }

    pub(crate) fn new_with_gx3d(eng: &Engine, reader: &mut Gx3DReader) -> Self {
        let gapi_engine = vxresult!(eng.gapi_engine.read());
        let uniform_buffer = vxresult!(gapi_engine.get_buffer_manager().write())
            .create_dynamic_buffer(size_of::<Uniform>() as isize);
        let scene_manager = vxresult!(eng.scene_manager.read());
        let mut texture_manager = vxresult!(scene_manager.texture_manager.write());
        let mut uniform = Uniform::new();
        let mut translucency = TranslucencyMode::Opaque;
        let read_color = |reader: &mut Gx3DReader| {
            [
                ((reader.read::<f32>() * 256.0) as u64 & 255) as u8,
                ((reader.read::<f32>() * 256.0) as u64 & 255) as u8,
                ((reader.read::<f32>() * 256.0) as u64 & 255) as u8,
                ((reader.read::<f32>() * 256.0) as u64 & 255) as u8,
            ]
        };
        let read_tex =
            |engine: &Engine, reader: &mut Gx3DReader, texture_manager: &mut TextureManager| {
                let t = reader.read_type_id();
                if t == Field::Texture as TypeId {
                    texture_manager.load_gx3d(engine, reader.read())
                } else if t == Field::Vector as TypeId {
                    texture_manager.create_2d_with_color(engine, read_color(reader))
                } else {
                    vxunexpected!()
                }
            };
        let read_value = |reader: &mut Gx3DReader| {
            let t = reader.read_type_id();
            if t != Field::Float as TypeId {
                vxunexpected!();
            }
            reader.read::<f32>()
        };
        // Alpha
        let t = reader.read_type_id();
        if t == Field::Float as TypeId {
            uniform.alpha = reader.read();
        } else if t == Field::Texture as TypeId {
            let _: Id = reader.read();
            translucency = TranslucencyMode::Tansparent;
        } else {
            vxunexpected!();
        }
        // AlphaCutoff
        let t = reader.read_type_id();
        if t == Field::Float as TypeId {
            uniform.alpha_cutoff = reader.read();
            if uniform.alpha_cutoff > 0.001 {
                translucency = TranslucencyMode::Cutoff;
            }
        } else {
            vxunexpected!();
        }
        // AlphaMode
        let t = reader.read_type_id();
        if t == Field::Float as TypeId {
            let alpha_mode: f32 = reader.read();
            if alpha_mode < 1.0 && 0.0 < alpha_mode {
                translucency = TranslucencyMode::Cutoff;
            }
        } else {
            vxunexpected!();
        }
        // BaseColor
        let t = reader.read_type_id();
        let base_color = if t == Field::Texture as TypeId {
            texture_manager.load_gx3d(&*eng, reader.read())
        } else if t == Field::Vector as TypeId {
            let color = read_color(reader);
            if color[3] < 254 {
                translucency = TranslucencyMode::Tansparent;
            }
            texture_manager.create_2d_with_color(&*eng, color)
        } else {
            vxunexpected!()
        };
        // BaseColorFactor
        let base_color_factor = read_tex(&*eng, reader, &mut *texture_manager);
        // DoubleSided
        let _double_sided = read_value(reader); // maybe in future I think about it
                                                // Emissive
        let emissive = read_tex(&*eng, reader, &mut *texture_manager);
        // EmissiveFactor
        let emissive_factor = read_tex(&*eng, reader, &mut *texture_manager);
        // MetallicFactor
        uniform.metallic_factor = read_value(reader);
        // MetallicRoughness
        let metallic_roughness = read_tex(&*eng, reader, &mut *texture_manager);
        // Normal
        let normal = read_tex(&*eng, reader, &mut *texture_manager);
        // NormalScale
        uniform.normal_scale = read_value(reader);
        // Occlusion
        let occlusion = read_tex(&*eng, reader, &mut *texture_manager);
        // OcclusionStrength
        uniform.occlusion_strength = read_value(reader);
        // RoughnessFactor
        uniform.roughness_factor = read_value(reader);
        let textures = vec![
            base_color.clone(),
            base_color_factor.clone(),
            metallic_roughness.clone(),
            normal.clone(),
            occlusion.clone(),
            emissive.clone(),
            emissive_factor.clone(),
        ];
        let gapi_engine = vxresult!(eng.gapi_engine.read());
        let mut descriptor_manager = vxresult!(gapi_engine.get_descriptor_manager().write());
        let descriptor_set = descriptor_manager.create_gbuff_set(&uniform_buffer, textures);
        let descriptor_set = Arc::new(descriptor_set);
        Material {
            base_color,
            base_color_factor,
            metallic_roughness,
            normal,
            occlusion,
            emissive,
            emissive_factor,
            translucency,
            uniform,
            uniform_buffer,
            descriptor_set,
        }
    }

    pub(crate) fn default(eng: &Engine) -> Self {
        let gapi_engine = vxresult!(eng.gapi_engine.read());
        let uniform_buffer = vxresult!(gapi_engine.get_buffer_manager().write())
            .create_dynamic_buffer(size_of::<Uniform>() as isize);
        let scene_manager = vxresult!(eng.scene_manager.read());
        let mut texture_manager = vxresult!(scene_manager.texture_manager.write());
        let uniform = Uniform::new();
        let translucency = TranslucencyMode::default();
        let base_color = texture_manager.create_2d_with_color(&*eng, [255, 255, 255, 255]);
        let base_color_factor = texture_manager.create_2d_with_color(&*eng, [255, 255, 255, 255]);
        let emissive = texture_manager.create_2d_with_color(&*eng, [255, 255, 255, 255]);
        let emissive_factor = texture_manager.create_2d_with_color(&*eng, [0, 0, 0, 0]);
        let metallic_roughness = texture_manager.create_2d_with_color(&*eng, [255, 255, 255, 255]);
        let normal = texture_manager.create_2d_with_color(&*eng, [127, 127, 255, 255]);
        let occlusion = texture_manager.create_2d_with_color(&*eng, [255, 255, 255, 255]);
        let textures = vec![
            base_color.clone(),
            base_color_factor.clone(),
            metallic_roughness.clone(),
            normal.clone(),
            occlusion.clone(),
            emissive.clone(),
            emissive_factor.clone(),
        ];
        let mut descriptor_manager = vxresult!(gapi_engine.get_descriptor_manager().write());
        let descriptor_set = descriptor_manager.create_gbuff_set(&uniform_buffer, textures);
        let descriptor_set = Arc::new(descriptor_set);
        Material {
            base_color,
            base_color_factor,
            metallic_roughness,
            normal,
            occlusion,
            emissive,
            emissive_factor,
            translucency,
            uniform,
            uniform_buffer,
            descriptor_set,
        }
    }

    pub fn finalize_textures_change(&mut self, eng: &Engine) {
        let textures = vec![
            self.base_color.clone(),
            self.base_color_factor.clone(),
            self.metallic_roughness.clone(),
            self.normal.clone(),
            self.occlusion.clone(),
            self.emissive.clone(),
            self.emissive_factor.clone(),
        ];
        let gapi_engine = vxresult!(eng.gapi_engine.read());
        let mut descriptor_manager = vxresult!(gapi_engine.get_descriptor_manager().write());
        let descriptor_set =
            descriptor_manager.create_gbuff_set(&self.uniform_buffer, textures);
        self.descriptor_set = Arc::new(descriptor_set);
    }

    pub fn update(&mut self, _scene: &Scene, _model: &Model) {}

    pub fn update_uniform_buffer(&mut self, frame_number: usize) {
        self.uniform_buffer.update(&self.uniform, frame_number);
    }

    pub fn bind(&self, cmd: &mut CmdBuffer, frame_number: usize) {
        let buffer = vxresult!(self.uniform_buffer.get_buffer(frame_number).read());
        cmd.bind_gbuff_material_descriptor(&*self.descriptor_set, &*buffer);
    }

    pub(crate) fn bind_shadow(&self, cmd: &mut CmdBuffer, frame_number: usize) {
        let buffer = vxresult!(self.uniform_buffer.get_buffer(frame_number).read());
        cmd.bind_shadow_mapper_material_descriptor(&*self.descriptor_set, &*buffer);
    }
}
