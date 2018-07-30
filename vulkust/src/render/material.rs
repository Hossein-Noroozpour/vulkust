use super::super::core::types::{Id, TypeId};
use super::buffer::DynamicBuffer;
use super::engine::Engine;
use super::gx3d::Gx3DReader;
use super::texture::{Manager as TextureManager, Texture};
use std::mem::size_of;
use std::sync::{Arc, RwLock};

#[repr(u8)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum Field {
    Float = 1,
    Texture = 2,
    Vector = 3,
}

#[repr(C)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Uniform {
    pub metallic_factor: f32,
    pub roughness_factor: f32,
    pub normal_scale: f32,
    pub occlusion_strength: f32,
    pub alpha: f32,
    pub alpha_cutoff: f32,
}

impl Uniform {
    pub fn new() -> Self {
        Uniform {
            metallic_factor: 1.0,
            roughness_factor: 1.0,
            normal_scale: 1.0,
            occlusion_strength: 1.0,
            alpha: 1.0,
            alpha_cutoff: 0.0,
        }
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub enum TranslucencyMode {
    Cutoff,
    Tansparent,
    Opaque,
}

#[cfg_attr(debug_assertions, derive(Debug))]
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
    pub uniform_buffer: Arc<RwLock<DynamicBuffer>>,
}

impl Material {
    pub fn new_with_gx3d(engine: &Arc<RwLock<Engine>>, reader: &mut Gx3DReader) -> Self {
        let eng = vxresult!(engine.read());
        let gapi_engine = vxresult!(eng.gapi_engine.read());
        let uniform_buffer = vxresult!(gapi_engine.buffer_manager.write())
            .create_dynamic_buffer(size_of::<Uniform>() as isize);
        let uniform_buffer = Arc::new(RwLock::new(uniform_buffer));
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
        let read_tex = |engine: &Arc<RwLock<Engine>>,
                        reader: &mut Gx3DReader,
                        texture_manager: &mut TextureManager| {
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
            texture_manager.load_gx3d(engine, reader.read())
        } else if t == Field::Vector as TypeId {
            let color = read_color(reader);
            if color[3] < 254 {
                translucency = TranslucencyMode::Tansparent;
            }
            texture_manager.create_2d_with_color(engine, color)
        } else {
            vxunexpected!()
        };
        // BaseColorFactor
        let base_color_factor = read_tex(engine, reader, &mut *texture_manager);
        // DoubleSided
        let _double_sided = read_value(reader); // maybe in future I think about it
                                                // Emissive
        let emissive = read_tex(engine, reader, &mut *texture_manager);
        // EmissiveFactor
        let emissive_factor = read_tex(engine, reader, &mut *texture_manager);
        // MetallicFactor
        uniform.metallic_factor = read_value(reader);
        // MetallicRoughness
        let metallic_roughness = read_tex(engine, reader, &mut *texture_manager);
        // Normal
        let normal = read_tex(engine, reader, &mut *texture_manager);
        // NormalScale
        uniform.normal_scale = read_value(reader);
        // Occlusion
        let occlusion = read_tex(engine, reader, &mut *texture_manager);
        // OcclusionStrength
        uniform.occlusion_strength = read_value(reader);
        // RoughnessFactor
        uniform.roughness_factor = read_value(reader);
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
        }
    }
}
