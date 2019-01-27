use super::super::super::core::gx3d::{Gx3DReader, Table as Gx3dTable};
use super::super::super::core::object::Object as CoreObject;
use super::super::super::core::types::{Id, TypeId};
use super::super::engine::Engine;
use super::super::gapi::GraphicApiEngine;
use super::super::image::View as ImageView;
use super::super::sampler::Sampler;
use super::{Cube, Loadable, Texture, Texture2D, TextureType};
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock, Weak};

use gltf;

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Manager {
    engine: Option<Weak<RwLock<Engine>>>,
    textures: BTreeMap<Id, Weak<RwLock<Texture>>>,
    name_to_id: BTreeMap<String, Id>,
    color_to_id: BTreeMap<[u8; 4], Id>,
    gx3d_table: Option<Gx3dTable>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            engine: None,
            textures: BTreeMap::new(),
            name_to_id: BTreeMap::new(),
            color_to_id: BTreeMap::new(),
            gx3d_table: None,
        }
    }

    pub(crate) fn set_gx3d_table(&mut self, gx3d_table: Gx3dTable) {
        self.gx3d_table = Some(gx3d_table);
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
        let reader: &mut Gx3DReader = &mut table.get_mut_reader();
        let t = reader.read_type_id();
        let texture: Arc<RwLock<Texture>> = if t == TextureType::T2D as TypeId {
            Arc::new(RwLock::new(Texture2D::new_with_gx3d(engine, reader, id)))
        } else if t == TextureType::Cube as TypeId {
            Arc::new(RwLock::new(Cube::new_with_gx3d(engine, reader, id)))
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
        engine: &GraphicApiEngine,
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
        engine: &GraphicApiEngine,
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

    pub fn create_2d_with_view_sampler(
        &mut self,
        image_view: Arc<ImageView>,
        sampler: Arc<Sampler>,
    ) -> Arc<RwLock<Texture>> {
        let tex = Texture2D::new_with_view_sampler(image_view, sampler);
        let id = tex.get_id();
        let tex: Arc<RwLock<Texture>> = Arc::new(RwLock::new(tex));
        self.textures.insert(id, Arc::downgrade(&tex));
        // todo make a refrencable/hashable object based on the image-view and sampler
        // todo maybe this is something unnecessary
        return tex;
    }

    pub(crate) fn set_engine(&mut self, e: Weak<RwLock<Engine>>) {
        self.engine = Some(e);
    }
}
