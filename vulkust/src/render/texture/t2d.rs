use super::super::super::core::gx3d::Gx3DReader;
use super::super::super::core::object::{Base as ObjectBase, Object as CoreObject};
use super::super::super::core::types::{Id, Size};
use super::super::engine::Engine;
use super::super::gapi::GraphicApiEngine;
use super::super::image::View as ImageView;
use super::super::sampler::Sampler;
use super::{Loadable, Texture};
use std::sync::Arc;

use gltf;
use image;

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Texture2D {
    obj_base: ObjectBase,
    name: Option<String>,
    image_view: Arc<ImageView>,
    sampler: Arc<Sampler>,
}

impl Texture2D {
    pub(crate) fn new_with_pixels(
        width: u32,
        height: u32,
        engine: &GraphicApiEngine,
        data: &[u8],
    ) -> Self {
        Self::new_with_base_pixels(ObjectBase::new(), width, height, engine, data)
    }

    pub(crate) fn new_with_base_pixels_name(
        obj_base: ObjectBase,
        width: u32,
        height: u32,
        engine: &GraphicApiEngine,
        data: &[u8],
        name: Option<String>,
    ) -> Self {
        let image_view = engine.create_texture_2d_with_pixels(width, height, data);
        let sampler = engine.get_linear_repeat_sampler().clone();
        Texture2D {
            obj_base,
            name,
            image_view,
            sampler,
        }
    }

    pub(crate) fn new_with_base_pixels(
        obj_base: ObjectBase,
        width: u32,
        height: u32,
        engine: &GraphicApiEngine,
        data: &[u8],
    ) -> Self {
        return Self::new_with_base_pixels_name(obj_base, width, height, engine, data, None);
    }

    pub(super) fn new_with_view_sampler(image_view: Arc<ImageView>, sampler: Arc<Sampler>) -> Self {
        Texture2D {
            obj_base: ObjectBase::new(),
            name: None,
            image_view,
            sampler,
        }
    }
}

impl CoreObject for Texture2D {
    fn get_id(&self) -> Id {
        self.obj_base.get_id()
    }
}

impl Texture for Texture2D {
    fn get_image_view(&self) -> &Arc<ImageView> {
        &self.image_view
    }

    fn get_sampler(&self) -> &Arc<Sampler> {
        &self.sampler
    }
}

impl Loadable for Texture2D {
    fn new_with_gltf(texture: &gltf::Texture, engine: &Engine, data: &[u8]) -> Self {
        let name = vx_unwrap!(texture.source().name()).to_string();
        let obj_base = ObjectBase::new();
        let view = match texture.source().source() {
            gltf::image::Source::View { view, mime_type: _ } => view,
            _ => vx_log_f!("Only embeded and view texture resources is acceptable."),
        };
        if let Some(_) = view.stride() {
            vx_log_f!("Stride is not acceptable in textures.");
        }
        let offset = view.offset();
        let length = view.length();
        match view.buffer().source() {
            gltf::buffer::Source::Bin => {}
            _ => vx_log_f!("Only embeded and view texture resources is acceptable."),
        }
        let img = vx_result!(image::load_from_memory(&data[offset..offset + length])).to_rgba();
        let (width, height) = img.dimensions();
        let img = img.into_raw();
        let geng = vx_result!(engine.get_gapi_engine().read());
        Self::new_with_base_pixels_name(obj_base, width, height, &geng, &img, Some(name))
    }

    fn new_with_gx3d(engine: &Engine, reader: &mut Gx3DReader, id: Id) -> Self {
        let obj_base = ObjectBase::new_with_id(id);
        let size: Size = reader.read();
        let data = reader.read_bytes(size);
        let img = vx_result!(image::load_from_memory(&data)).to_rgba8();
        let (width, height) = img.dimensions();
        let img = img.into_raw();
        let geng = vx_result!(engine.get_gapi_engine().read());
        Self::new_with_base_pixels(obj_base, width, height, &geng, &img)
    }
}
