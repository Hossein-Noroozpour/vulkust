use super::super::core::object::{Base as ObjectBase, Object as CoreObject};
use super::super::core::types::Id;
use super::engine::Engine;
use super::gx3d::Table as Gx3dTable;
use super::image::View as ImageView;
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock, Weak};

use gltf;

pub trait Texture: CoreObject {
    fn get_image_view(&self) -> &Arc<ImageView>;
}

pub trait Loadable: Sized {
    fn new_with_gltf(&gltf::Texture, &Engine, &[u8]) -> Self;
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Manager {
    textures: BTreeMap<Id, Weak<RwLock<Texture>>>,
    name_to_id: BTreeMap<String, Id>,
    pub gx3d_table: Option<Gx3dTable>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            textures: BTreeMap::new(),
            name_to_id: BTreeMap::new(),
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

    pub fn load_gx3d(&mut self, engine: &Arc<RwLock<Engine>>, id: Id) -> Arc<RwLock<Texture>> {
        vxunimplemented!();
    }

    pub fn create_2d_with_pixels(
        &mut self,
        width: u32,
        height: u32,
        engine: &Arc<RwLock<Engine>>,
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
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Texture2D {
    pub obj_base: ObjectBase,
    pub name: Option<String>,
    pub image_view: Arc<ImageView>,
}

impl Texture2D {
    pub fn new_with_pixels(
        width: u32,
        height: u32,
        engine: &Arc<RwLock<Engine>>,
        data: &[u8],
    ) -> Self {
        let engine = vxresult!(engine.read());
        let engine = vxresult!(engine.gapi_engine.read());
        let image_view = engine.create_texture_2d_with_pixels(width, height, data);
        Texture2D {
            obj_base: ObjectBase::new(),
            name: None,
            image_view,
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
        Texture2D {
            obj_base,
            name: Some(name),
            image_view,
        }
    }
}
