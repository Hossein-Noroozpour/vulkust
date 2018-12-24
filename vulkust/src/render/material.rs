use super::super::core::gx3d::Gx3DReader;
use super::super::core::types::{Id, Real, TypeId};
use super::buffer::Dynamic as DynamicBuffer;
use super::command::Buffer as CmdBuffer;
use super::descriptor::Set as DescriptorSet;
use super::engine::Engine;
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
#[derive(Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
pub struct Uniform {
    alpha: Real,
    alpha_cutoff: Real,
    metallic_factor: Real,
    normal_scale: Real,
    occlusion_strength: Real,
    roughness_factor: Real,
}

impl Uniform {
    pub fn new() -> Self {
        Uniform {
            alpha: 1.0,
            alpha_cutoff: 0.001,
            metallic_factor: 1.0,
            normal_scale: 1.0,
            occlusion_strength: 1.0,
            roughness_factor: 1.0,
        }
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
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

    // pub(crate) fn get_uniform_buffer(&self) -> &DynamicBuffer {
    //     return &self.uniform_buffer;
    // }

    // pub(crate) fn get_descriptor_set(&self) -> &Arc<DescriptorSet> {
    //     return &self.descriptor_set;
    // }

    pub(crate) fn new_with_gx3d(eng: &Engine, reader: &mut Gx3DReader) -> Self {
        let gapi_engine = vxresult!(eng.get_gapi_engine().read());
        let uniform_buffer = vxresult!(gapi_engine.get_buffer_manager().write())
            .create_dynamic_buffer(size_of::<Uniform>() as isize);
        let mut texture_manager = vxresult!(eng.get_asset_manager().get_texture_manager().write());
        let mut uniform = Uniform::new();
        let mut translucency = TranslucencyMode::Opaque;
        let read_color = |r: &mut Gx3DReader| {
            let read = |r: &mut Gx3DReader| {
                let f = r.read::<Real>() * 255.0;
                #[cfg(debug_gx3d)]
                {
                    if f > 255.5 || f < 0.0 {
                        vxunexpected!();
                    }
                }
                f as u8
            };
            [read(r), read(r), read(r), read(r)]
        };
        let read_tex =
            |engine: &Engine, reader: &mut Gx3DReader, texture_manager: &mut TextureManager| {
                let t = reader.read_type_id();
                if t == Field::Texture as TypeId {
                    let id: Id = reader.read();
                    #[cfg(debug_gx3d)]
                    vxlogi!("Texture Id: {:?}", id);
                    texture_manager.load_gx3d(engine, id)
                } else if t == Field::Vector as TypeId {
                    let color = read_color(reader);
                    #[cfg(debug_gx3d)]
                    vxlogi!("Color: {:?}", &color);
                    texture_manager.create_2d_with_color(&gapi_engine, color)
                } else {
                    vxunexpected!()
                }
            };
        let read_value = |reader: &mut Gx3DReader| {
            let t = reader.read_type_id();
            if t != Field::Float as TypeId {
                vxunexpected!();
            }
            let v = reader.read::<Real>();
            #[cfg(debug_gx3d)]
            vxlogi!("Value: {:?}", v);
            v
        };
        let mut alpha = None;
        let mut alpha_cutoff = None;
        let mut alpha_mode = None;
        let mut base_color = None;
        let mut base_color_factor = None;
        let mut double_sided = None;
        let mut emissive = None;
        let mut emissive_factor = None;
        let mut metallic_factor = None;
        let mut metallic_roughness = None;
        let mut normal = None;
        let mut normal_scale = None;
        let mut occlusion = None;
        let mut occlusion_strength = None;
        let mut roughness_factor = None;
        for _ in 0..15 {
            let it = reader.read_type_id();
            match it {
                1 => {
                    // Alpha
                    #[cfg(debug_gx3d)]
                    {
                        if alpha.is_some() {
                            vxunexpected!();
                        }
                    }
                    let t = reader.read_type_id();
                    if t == Field::Float as TypeId {
                        alpha = Some(reader.read());
                    } else if t == Field::Texture as TypeId {
                        let _: Id = reader.read();
                        alpha = Some(1.0);
                        translucency = TranslucencyMode::Tansparent;
                    } else {
                        vxunexpected!();
                    }
                    #[cfg(debug_gx3d)]
                    vxlogi!("Alpha is: {:?}", uniform.alpha);
                }
                2 => {
                    // AlphaCutoff
                    #[cfg(debug_gx3d)]
                    {
                        if alpha_cutoff.is_some() {
                            vxunexpected!();
                        }
                    }
                    let t = reader.read_type_id();
                    if t == Field::Float as TypeId {
                        alpha_cutoff = Some(reader.read());
                        if vxunwrap!(alpha_cutoff) > 0.003 {
                            translucency = TranslucencyMode::Cutoff;
                        }
                    } else {
                        vxunexpected!();
                    }
                    #[cfg(debug_gx3d)]
                    vxlogi!("Alpha cutoff is: {:?}", uniform.alpha_cutoff);
                }
                3 => {
                    // AlphaMode
                    #[cfg(debug_gx3d)]
                    {
                        if alpha_mode.is_some() {
                            vxunexpected!();
                        }
                    }
                    let t = reader.read_type_id();
                    if t == Field::Float as TypeId {
                        alpha_mode = Some(reader.read());
                        let alpha_mode = vxunwrap!(alpha_mode);
                        if alpha_mode < 1.0 && 0.0 < alpha_mode {
                            translucency = TranslucencyMode::Cutoff;
                        }
                    } else {
                        vxunexpected!();
                    }
                    #[cfg(debug_gx3d)]
                    vxlogi!("Translucency is: {:?}", translucency);
                }
                4 => {
                    // BaseColor
                    #[cfg(debug_gx3d)]
                    {
                        if base_color.is_some() {
                            vxunexpected!();
                        }
                    }
                    let t = reader.read_type_id();
                    base_color = Some(if t == Field::Texture as TypeId {
                        let id: Id = reader.read();
                        #[cfg(debug_gx3d)]
                        vxlogi!("Base color is: texture<{:?}>", id);
                        texture_manager.load_gx3d(&*eng, id)
                    } else if t == Field::Vector as TypeId {
                        let color = read_color(reader);
                        if color[3] < 254 {
                            translucency = TranslucencyMode::Tansparent;
                        }
                        #[cfg(debug_gx3d)]
                        vxlogi!("Base color is: {:?}", &color);
                        texture_manager.create_2d_with_color(&*gapi_engine, color)
                    } else {
                        vxunexpected!()
                    });
                }
                5 => {
                    // BaseColorFactor
                    #[cfg(debug_gx3d)]
                    {
                        if base_color_factor.is_some() {
                            vxunexpected!();
                        }
                    }
                    #[cfg(debug_gx3d)]
                    vxlogi!("Base color factor");
                    base_color_factor = Some(read_tex(&*eng, reader, &mut *texture_manager));
                }
                6 => {
                    // DoubleSided maybe // in future I gonna think about it
                    #[cfg(debug_gx3d)]
                    {
                        if double_sided.is_some() {
                            vxunexpected!();
                        }
                    }
                    #[cfg(debug_gx3d)]
                    vxlogi!("Double sided");
                    double_sided = Some(read_value(reader));
                }
                7 => {
                    // Emissive
                    #[cfg(debug_gx3d)]
                    {
                        if emissive.is_some() {
                            vxunexpected!();
                        }
                    }
                    #[cfg(debug_gx3d)]
                    vxlogi!("Emissive");
                    emissive = Some(read_tex(&*eng, reader, &mut *texture_manager));
                }
                8 => {
                    // EmissiveFactor
                    #[cfg(debug_gx3d)]
                    {
                        if emissive_factor.is_some() {
                            vxunexpected!();
                        }
                    }
                    #[cfg(debug_gx3d)]
                    vxlogi!("Emissive factor");
                    emissive_factor = Some(read_tex(&*eng, reader, &mut *texture_manager));
                }
                9 => {
                    // MetallicFactor
                    #[cfg(debug_gx3d)]
                    {
                        if metallic_factor.is_some() {
                            vxunexpected!();
                        }
                    }
                    #[cfg(debug_gx3d)]
                    vxlogi!("Metallic Factor");
                    metallic_factor = Some(read_value(reader));
                }
                10 => {
                    // MetallicRoughness
                    #[cfg(debug_gx3d)]
                    {
                        if metallic_roughness.is_some() {
                            vxunexpected!();
                        }
                    }
                    #[cfg(debug_gx3d)]
                    vxlogi!("MetallicRoughness");
                    metallic_roughness = Some(read_tex(&*eng, reader, &mut *texture_manager));
                }
                11 => {
                    // Normal
                    #[cfg(debug_gx3d)]
                    {
                        if normal.is_some() {
                            vxunexpected!();
                        }
                    }
                    #[cfg(debug_gx3d)]
                    vxlogi!("Normal");
                    normal = Some(read_tex(&*eng, reader, &mut *texture_manager));
                }
                12 => {
                    // NormalScale
                    #[cfg(debug_gx3d)]
                    {
                        if normal_scale.is_some() {
                            vxunexpected!();
                        }
                    }
                    #[cfg(debug_gx3d)]
                    vxlogi!("NormalScale");
                    normal_scale = Some(read_value(reader));
                }
                13 => {
                    // Occlusion
                    #[cfg(debug_gx3d)]
                    {
                        if occlusion.is_some() {
                            vxunexpected!();
                        }
                    }
                    #[cfg(debug_gx3d)]
                    vxlogi!("Occlusion");
                    occlusion = Some(read_tex(&*eng, reader, &mut *texture_manager));
                }
                14 => {
                    // OcclusionStrength
                    #[cfg(debug_gx3d)]
                    {
                        if occlusion_strength.is_some() {
                            vxunexpected!();
                        }
                    }
                    #[cfg(debug_gx3d)]
                    vxlogi!("OcclusionStrength");
                    occlusion_strength = Some(read_value(reader));
                }
                15 => {
                    // RoughnessFactor
                    #[cfg(debug_gx3d)]
                    {
                        if roughness_factor.is_some() {
                            vxunexpected!();
                        }
                    }
                    #[cfg(debug_gx3d)]
                    vxlogi!("RoughnessFactor");
                    roughness_factor = Some(read_value(reader));
                }
                c @ _ => vxlogf!("Unexpected input key {}", c),
            }
        }
        uniform.alpha = vxunwrap!(alpha);
        uniform.alpha_cutoff = vxunwrap!(alpha_cutoff);
        uniform.metallic_factor = vxunwrap!(metallic_factor);
        uniform.normal_scale = vxunwrap!(normal_scale);
        uniform.occlusion_strength = vxunwrap!(occlusion_strength);
        uniform.roughness_factor = vxunwrap!(roughness_factor);
        let _: Real = vxunwrap!(alpha_mode);
        let base_color = vxunwrap!(base_color);
        let base_color_factor = vxunwrap!(base_color_factor);
        let _: Real = vxunwrap!(double_sided);
        let emissive = vxunwrap!(emissive);
        let emissive_factor = vxunwrap!(emissive_factor);
        let metallic_roughness = vxunwrap!(metallic_roughness);
        let normal = vxunwrap!(normal);
        let occlusion = vxunwrap!(occlusion);
        let textures = vec![
            base_color.clone(),
            base_color_factor.clone(),
            metallic_roughness.clone(),
            normal.clone(),
            occlusion.clone(),
            emissive.clone(),
            emissive_factor.clone(),
        ];
        let gapi_engine = vxresult!(eng.get_gapi_engine().read());
        let mut descriptor_manager = vxresult!(gapi_engine.get_descriptor_manager().write());
        let descriptor_set = descriptor_manager.create_gbuff_set(&uniform_buffer, textures);
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

    pub fn default(eng: &Engine) -> Self {
        let gapi_engine = vxresult!(eng.get_gapi_engine().read());
        let uniform_buffer = vxresult!(gapi_engine.get_buffer_manager().write())
            .create_dynamic_buffer(size_of::<Uniform>() as isize);
        let mut texture_manager = vxresult!(eng.get_asset_manager().get_texture_manager().write());
        let uniform = Uniform::new();
        let translucency = TranslucencyMode::default();
        let base_color = texture_manager.create_2d_with_color(&*gapi_engine, [255, 255, 255, 255]);
        let base_color_factor =
            texture_manager.create_2d_with_color(&*gapi_engine, [255, 255, 255, 255]);
        let emissive = texture_manager.create_2d_with_color(&*gapi_engine, [255, 255, 255, 255]);
        let emissive_factor = texture_manager.create_2d_with_color(&*gapi_engine, [0, 0, 0, 0]);
        let metallic_roughness =
            texture_manager.create_2d_with_color(&*gapi_engine, [255, 255, 255, 255]);
        let normal = texture_manager.create_2d_with_color(&*gapi_engine, [127, 127, 255, 255]);
        let occlusion = texture_manager.create_2d_with_color(&*gapi_engine, [255, 255, 255, 255]);
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
        let gapi_engine = vxresult!(eng.get_gapi_engine().read());
        let mut descriptor_manager = vxresult!(gapi_engine.get_descriptor_manager().write());
        self.descriptor_set = descriptor_manager.create_gbuff_set(&self.uniform_buffer, textures);
    }

    pub fn update_uniform_buffer(&mut self, frame_number: usize) {
        self.uniform_buffer.update(&self.uniform, frame_number);
    }

    pub fn bind_gbuffer(&self, cmd: &mut CmdBuffer, frame_number: usize) {
        let buffer = vxresult!(self.uniform_buffer.get_buffer(frame_number).read());
        cmd.bind_gbuff_material_descriptor(&*self.descriptor_set, &*buffer);
    }

    pub(crate) fn bind_shadow(&self, cmd: &mut CmdBuffer, frame_number: usize) {
        let buffer = vxresult!(self.uniform_buffer.get_buffer(frame_number).read());
        cmd.bind_shadow_mapper_material_descriptor(&*self.descriptor_set, &*buffer);
    }

    pub fn set_base_color(&mut self, eng: &Engine, r: u8, g: u8, b: u8, a: u8) {
        let mut texmgr = vxresult!(eng.get_asset_manager().get_texture_manager().write());
        self.base_color =
            texmgr.create_2d_with_color(&*vxresult!(eng.get_gapi_engine().read()), [r, g, b, a]);
    }

    pub fn set_metallic_factor(&mut self, v: Real) {
        self.uniform.metallic_factor = v;
    }

    pub fn set_roughness_factor(&mut self, v: Real) {
        self.uniform.roughness_factor = v;
    }
}
