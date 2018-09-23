use super::super::core::object::{Base as ObjectBase, Object as CoreObject};
use super::super::core::types::{Id, Size, TypeId};
use super::engine::Engine;
use super::gx3d::{Gx3DReader, Table as Gx3dTable};
use super::image::View as ImageView;
use super::sampler::Sampler;
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock, Weak};

use gltf;
use image;

#[repr(u8)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum TextureType {
    T2D = 1,
    T3D = 2,
    Cube = 3,
}

pub trait Texture: CoreObject {
    fn get_image_view(&self) -> &Arc<ImageView>;
    fn get_sampler(&self) -> &Arc<Sampler>;
}

pub trait Loadable: Sized {
    fn new_with_gltf(&gltf::Texture, &Engine, &[u8]) -> Self;
    fn new_with_gx3d(&Engine, &mut Gx3DReader, Id) -> Self;
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Manager {
    textures: BTreeMap<Id, Weak<RwLock<Texture>>>,
    name_to_id: BTreeMap<String, Id>,
    color_to_id: BTreeMap<[u8; 4], Id>,
    pub gx3d_table: Option<Gx3dTable>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            textures: BTreeMap::new(),
            name_to_id: BTreeMap::new(),
            color_to_id: BTreeMap::new(),
            gx3d_table: None,
        }
    }

    pub fn load_gltf<T>(
        &mut self,
        texture: &gltf::Texture,
        engine: &Engine,
        data: &[u8],
    ) -> Arc<RwLock<Texture>>
    where
        T: 'static + Loadable + Texture,
    {
        let name = vxunwrap!(texture.source().name()).to_string();
        if let Some(id) = self.name_to_id.get(&name) {
            if let Some(t) = self.textures.get(id) {
                if let Some(t) = t.upgrade() {
                    vxlogi!("cached");
                    return t;
                }
            }
        }
        let texture: Arc<RwLock<Texture>> =
            Arc::new(RwLock::new(T::new_with_gltf(texture, engine, data)));
        let id = vxresult!(texture.read()).get_id();
        let weak = Arc::downgrade(&texture);
        self.name_to_id.insert(name, id);
        self.textures.insert(id, weak);
        return texture;
    }

    pub fn load_gx3d(&mut self, engine: &Engine, id: Id) -> Arc<RwLock<Texture>> {
        if let Some(t) = self.textures.get(&id) {
            if let Some(t) = t.upgrade() {
                return t;
            }
        }
        let table = vxunwrap!(&mut self.gx3d_table);
        table.goto(id);
        let reader: &mut Gx3DReader = &mut table.reader;
        let t = reader.read_type_id();
        let texture: Arc<RwLock<Texture>> = if t == TextureType::T2D as TypeId {
            Arc::new(RwLock::new(Texture2D::new_with_gx3d(engine, reader, id)))
        } else {
            vxunexpected!();
        };
        self.textures.insert(id, Arc::downgrade(&texture));
        return texture;
    }

    pub fn create_2d_with_pixels(
        &mut self,
        width: u32,
        height: u32,
        engine: &Engine,
        data: &[u8],
    ) -> Arc<RwLock<Texture2D>> {
        let tex = Texture2D::new_with_pixels(width, height, engine, data);
        let id = tex.get_id();
        let tex = Arc::new(RwLock::new(tex));
        let t: Arc<RwLock<Texture>> = tex.clone();
        let t = Arc::downgrade(&t);
        self.textures.insert(id, t);
        return tex;
    }

    pub fn create_2d_with_color(
        &mut self,
        engine: &Engine,
        color: [u8; 4],
    ) -> Arc<RwLock<Texture>> {
        if let Some(id) = self.color_to_id.get(&color) {
            if let Some(t) = self.textures.get(id) {
                if let Some(t) = t.upgrade() {
                    vxlogi!("color texture cached.");
                    return t;
                }
            }
        }
        let tex = Texture2D::new_with_pixels(1, 1, engine, &color);
        let id = tex.get_id();
        let tex: Arc<RwLock<Texture>> = Arc::new(RwLock::new(tex));
        self.textures.insert(id, Arc::downgrade(&tex));
        self.color_to_id.insert(color, id);
        return tex;
    }

    pub fn create_2d_with_view_sampler(&mut self, image_view: Arc<ImageView>, sampler: Arc<Sampler>) -> Arc<RwLock<Texture>> {
        let tex = Texture2D::new_with_view_sampler(image_view, sampler);
        let id = tex.get_id();
        let tex: Arc<RwLock<Texture>> = Arc::new(RwLock::new(tex));
        self.textures.insert(id, Arc::downgrade(&tex));
        // todo make a refrencable/hashable object based on the image-view and sampler
        // todo maybe this is something unnecessary
        return tex;
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Texture2D {
    pub obj_base: ObjectBase,
    pub name: Option<String>,
    pub image_view: Arc<ImageView>,
    pub sampler: Arc<Sampler>,
}

impl Texture2D {
    pub fn new_with_pixels(width: u32, height: u32, engine: &Engine, data: &[u8]) -> Self {
        Self::new_with_base_pixels(ObjectBase::new(), width, height, engine, data)
    }

    pub fn new_with_base_pixels(
        obj_base: ObjectBase,
        width: u32,
        height: u32,
        engine: &Engine,
        data: &[u8],
    ) -> Self {
        let engine = vxresult!(engine.gapi_engine.read());
        let image_view = engine.create_texture_2d_with_pixels(width, height, data);
        let sampler = engine.sampler.clone();
        Texture2D {
            obj_base,
            name: None,
            image_view,
            sampler,
        }
    }

    fn new_with_view_sampler(image_view: Arc<ImageView>, sampler: Arc<Sampler>) -> Self {
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
        let name = vxunwrap!(texture.source().name()).to_string();
        let obj_base = ObjectBase::new();
        let view = match texture.source().source() {
            gltf::image::Source::View { view, mime_type: _ } => view,
            _ => vxlogf!("Only embeded and view texture resources is acceptable."),
        };
        if let Some(_) = view.stride() {
            vxlogf!("Stride is not acceptable in textures.");
        }
        let offset = view.offset();
        let length = view.length();
        match view.buffer().source() {
            gltf::buffer::Source::Bin => {}
            _ => vxlogf!("Only embeded and view texture resources is acceptable."),
        }
        let engine = vxresult!(engine.gapi_engine.read());
        let image_view = engine.create_texture_with_bytes(&data[offset..offset + length]);
        let sampler = engine.sampler.clone();
        // todo call new_with_pixels do not create it your self
        Texture2D {
            obj_base,
            name: Some(name),
            image_view,
            sampler,
        }
    }

    fn new_with_gx3d(engine: &Engine, reader: &mut Gx3DReader, id: Id) -> Self {
        let obj_base = ObjectBase::new_with_id(id);
        let size: Size = reader.read();
        let data = reader.read_bytes(size);
        let img = vxresult!(image::load_from_memory(&data)).to_rgba();
        let (width, height) = img.dimensions();
        let img = img.into_raw();
        Self::new_with_base_pixels(obj_base, width, height, engine, &img)
    }
}
