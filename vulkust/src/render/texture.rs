use super::super::core::object::{create_id, Object as CoreObject};
use super::super::core::types::Id;
use super::engine::GraphicApiEngine;
use super::image::View as ImageView;
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock, Weak};

use gltf;

pub trait Texture: CoreObject {
    fn get_image_view(&self) -> &Arc<ImageView> {
        vxunimplemented!();
    }
}

pub trait Loadable: Sized {
    fn new_with_gltf(&gltf::Texture, &Arc<RwLock<GraphicApiEngine>>, &Vec<u8>) -> Self {
        vxunexpected!();
    }
}

pub struct Manager {
    cached: BTreeMap<Id, Weak<RwLock<Texture>>>,
    name_to_id: BTreeMap<String, Id>,
    pub gapi_engine: Arc<RwLock<GraphicApiEngine>>,
}

impl Manager {
    pub fn new(engine: &Arc<RwLock<GraphicApiEngine>>) -> Self {
        let gapi_engine = engine.clone();
        Manager {
            cached: BTreeMap::new(),
            name_to_id: BTreeMap::new(),
            gapi_engine,
        }
    }

    pub fn get_with_gltf<T>(
        &mut self,
        texture: &gltf::Texture,
        data: &Vec<u8>,
    ) -> Arc<RwLock<Texture>>
    where
        T: 'static + Loadable + Texture,
    {
        let name = vxunwrap_o!(texture.source().name()).to_string();
        if let Some(id) = self.name_to_id.get(&name) {
            if let Some(t) = self.cached.get(id) {
                if let Some(t) = t.upgrade() {
                    vxlogi!("cached");
                    return t;
                }
            }
        }
        let texture: Arc<RwLock<Texture>> = Arc::new(RwLock::new(T::new_with_gltf(
            texture,
            &self.gapi_engine,
            data,
        )));
        let id = vxresult!(texture.read()).get_id();
        let weak = Arc::downgrade(&texture);
        self.name_to_id.insert(name, id);
        self.cached.insert(id, weak);
        return texture;
    }
}

pub struct Texture2D {
    pub id: Id,
    pub name: String,
    pub image_view: Arc<ImageView>,
}

impl CoreObject for Texture2D {
    fn get_id(&self) -> Id {
        self.id
    }
}

impl Texture for Texture2D {
    fn get_image_view(&self) -> &Arc<ImageView> {
        &self.image_view
    }
}

impl Loadable for Texture2D {
    fn new_with_gltf(
        texture: &gltf::Texture,
        engine: &Arc<RwLock<GraphicApiEngine>>,
        data: &Vec<u8>,
    ) -> Self {
        let name = vxunwrap_o!(texture.source().name()).to_string();
        let id = create_id();
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
        let image_view =
            vxresult!(engine.read()).create_texture_with_bytes(&data[offset..offset + length]);
        Texture2D {
            id,
            name,
            image_view,
        }
    }
}
