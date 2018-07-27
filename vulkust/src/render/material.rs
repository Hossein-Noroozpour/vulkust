use super::super::core::types::{Id, TypeId};
use super::buffer::DynamicBuffer;
use super::engine::Engine;
use super::gx3d::Gx3DReader;
use super::texture::Texture;
use std::mem::size_of;
use std::sync::{Arc, RwLock};

use math;

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
    pub base_color: math::Vector3<f32>,
    pub base_color_factor: math::Vector3<f32>,
    pub metallic_roughness: math::Vector3<f32>,
    pub metallic_factor: f32,
    pub roughness_factor: f32,
    pub normal: math::Vector3<f32>,
    pub normal_scale: f32,
    pub occlusion: math::Vector3<f32>,
    pub occlusion_strength: f32,
    pub emissive: math::Vector3<f32>,
    pub emissive_factor: math::Vector3<f32>,
    pub alpha: f32,
    pub alpha_cutoff: f32,
}

impl Uniform {
    pub fn new() -> Self {
        Uniform {
            base_color: math::Vector3::new(1.0, 1.0, 1.0),
            base_color_factor: math::Vector3::new(1.0, 1.0, 1.0),
            metallic_roughness: math::Vector3::new(1.0, 1.0, 1.0),
            metallic_factor: 1.0,
            roughness_factor: 1.0,
            normal: math::Vector3::new(0.5, 0.5, 1.0),
            normal_scale: 1.0,
            occlusion: math::Vector3::new(1.0, 1.0, 1.0),
            occlusion_strength: 1.0,
            emissive: math::Vector3::new(1.0, 1.0, 1.0),
            emissive_factor: math::Vector3::new(0.0, 0.0, 0.0),
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
    pub base_color_texture: Option<Arc<RwLock<Texture>>>,
    pub base_color_factor_texture: Option<Arc<RwLock<Texture>>>,
    pub metallic_roughness_texture: Option<Arc<RwLock<Texture>>>,
    pub normal_texture: Option<Arc<RwLock<Texture>>>,
    pub occlusion_texture: Option<Arc<RwLock<Texture>>>,
    pub emissive_texture: Option<Arc<RwLock<Texture>>>,
    pub emissive_factor_texture: Option<Arc<RwLock<Texture>>>,
    pub translucency: TranslucencyMode,
    pub uniform: Uniform,
    pub uniform_buffer: DynamicBuffer,
}

impl Material {
    pub fn new(engine: &Engine) -> Self {
        let gapi_engine = vxresult!(engine.gapi_engine.read());
        let uniform_buffer = vxresult!(gapi_engine.buffer_manager.write())
            .create_dynamic_buffer(size_of::<Uniform>() as isize);
        Material {
            base_color_texture: None,
            base_color_factor_texture: None,
            metallic_roughness_texture: None,
            normal_texture: None,
            occlusion_texture: None,
            emissive_texture: None,
            emissive_factor_texture: None,
            translucency: TranslucencyMode::Opaque,
            uniform: Uniform::new(),
            uniform_buffer,
        }
    }

    pub fn new_with_gx3d(engine: &Arc<RwLock<Engine>>, reader: &mut Gx3DReader) -> Self {
        let eng = vxresult!(engine.read());
        let scene_manager = vxresult!(eng.scene_manager.read());
        let mut texture_manager = vxresult!(scene_manager.texture_manager.write());
        let mut myself = Self::new(&eng);
        // Alpha
        let t = reader.read_type_id();
        if t == Field::Float as TypeId {
            myself.uniform.alpha = reader.read();
        } else if t == Field::Texture as TypeId {
            let _: Id = reader.read();
            myself.translucency = TranslucencyMode::Tansparent;
        } else {
            vxunexpected!();
        }
        // AlphaCutoff
        let t = reader.read_type_id();
        if t == Field::Float as TypeId {
            myself.uniform.alpha_cutoff = reader.read();
        } else {
            vxunexpected!();
        }
        // AlphaMode
        let t = reader.read_type_id();
        if t == Field::Float as TypeId {
            let alpha_mode: f32 = reader.read();
            if alpha_mode < 1.0 && 0.0 < alpha_mode {
                myself.translucency = TranslucencyMode::Cutoff;
            }
        } else {
            vxunexpected!();
        }
        // BaseColor
        let t = reader.read_type_id();
        if t == Field::Texture as TypeId {
            let txtid = reader.read();
            myself.base_color_texture = Some(texture_manager.load_gx3d(engine, txtid));
        } else if t == Field::Vector as TypeId {
            myself.uniform.base_color =
                math::Vector3::new(reader.read(), reader.read(), reader.read());
        } else {
            vxunexpected!();
        }
        // BaseColorFactor
        let t = reader.read_type_id();
        if t == Field::Texture as TypeId {
            let txtid = reader.read();
            myself.base_color_factor_texture = Some(texture_manager.load_gx3d(engine, txtid));
        } else if t == Field::Vector as TypeId {
            myself.uniform.base_color_factor =
                math::Vector3::new(reader.read(), reader.read(), reader.read());
        } else {
            vxunexpected!();
        }
        // DoubleSided
        let t = reader.read_type_id();
        if t == Field::Float as TypeId {
            let _double_sided: f32 = reader.read();
        // todo change pipeline based on this
        } else {
            vxunexpected!();
        }
        // Emissive
        let t = reader.read_type_id();
        if t == Field::Texture as TypeId {
            let txtid = reader.read();
            myself.emissive_texture = Some(texture_manager.load_gx3d(engine, txtid));
        } else if t == Field::Vector as TypeId {
            myself.uniform.emissive =
                math::Vector3::new(reader.read(), reader.read(), reader.read());
        } else {
            vxunexpected!();
        }
        // EmissiveFactor
        let t = reader.read_type_id();
        if t == Field::Texture as TypeId {
            let txtid = reader.read();
            myself.emissive_factor_texture = Some(texture_manager.load_gx3d(engine, txtid));
        } else if t == Field::Vector as TypeId {
            myself.uniform.emissive_factor =
                math::Vector3::new(reader.read(), reader.read(), reader.read());
        } else {
            vxunexpected!();
        }
        // MetallicFactor
        let t = reader.read_type_id();
        if t == Field::Float as TypeId {
            myself.uniform.metallic_factor = reader.read();
        } else {
            vxunexpected!();
        }
        // MetallicRoughness
        let t = reader.read_type_id();
        if t == Field::Texture as TypeId {
            let txtid = reader.read();
            myself.metallic_roughness_texture = Some(texture_manager.load_gx3d(engine, txtid));
        } else if t == Field::Vector as TypeId {
            myself.uniform.metallic_roughness =
                math::Vector3::new(reader.read(), reader.read(), reader.read());
        } else {
            vxunexpected!();
        }
        // Normal
        let t = reader.read_type_id();
        if t == Field::Texture as TypeId {
            let txtid = reader.read();
            myself.normal_texture = Some(texture_manager.load_gx3d(engine, txtid));
        } else if t == Field::Vector as TypeId {
            myself.uniform.normal = math::Vector3::new(reader.read(), reader.read(), reader.read());
        } else {
            vxunexpected!();
        }
        // NormalScale
        let t = reader.read_type_id();
        if t == Field::Float as TypeId {
            myself.uniform.normal_scale = reader.read();
        } else {
            vxunexpected!();
        }
        // Occlusion
        let t = reader.read_type_id();
        if t == Field::Texture as TypeId {
            let txtid = reader.read();
            myself.occlusion_texture = Some(texture_manager.load_gx3d(engine, txtid));
        } else if t == Field::Vector as TypeId {
            myself.uniform.occlusion =
                math::Vector3::new(reader.read(), reader.read(), reader.read());
        } else {
            vxunexpected!();
        }
        // OcclusionStrength
        let t = reader.read_type_id();
        if t == Field::Float as TypeId {
            myself.uniform.occlusion_strength = reader.read();
        } else {
            vxunexpected!();
        }
        // RoughnessFactor
        let t = reader.read_type_id();
        if t == Field::Float as TypeId {
            myself.uniform.roughness_factor = reader.read();
        } else {
            vxunexpected!();
        }
        return myself;
    }
}
