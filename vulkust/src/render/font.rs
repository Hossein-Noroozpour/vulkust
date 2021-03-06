use super::super::core::gx3d::Table as Gx3dTable;
use super::super::core::object::{Base as ObjectBase, Object as CoreObject};
use super::super::core::types::Id;
use super::super::system::file::File;
use rusttype::Font as TypeFont;
use std::collections::BTreeMap;
use std::default::Default;
#[cfg(debug_mode)]
use std::fmt;
use std::io::Read;
use std::sync::{Arc, RwLock, Weak};

pub trait Font: CoreObject {
    fn get_font(&self) -> &TypeFont;
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Manager {
    fonts: BTreeMap<Id, Weak<RwLock<dyn Font>>>,
    name_to_id: BTreeMap<String, Id>,
    default: Arc<RwLock<dyn Font>>,
    gx3d_table: Option<Gx3dTable>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            fonts: BTreeMap::new(),
            name_to_id: BTreeMap::new(),
            default: Arc::new(RwLock::new(Base::default())),
            gx3d_table: None,
        }
    }

    pub(crate) fn set_gx3d_table(&mut self, gx3d_table: Gx3dTable) {
        self.gx3d_table = Some(gx3d_table);
    }

    pub(crate) fn get_default(&self) -> &Arc<RwLock<dyn Font>> {
        return &self.default;
    }

    pub fn load_ttf(&mut self, name: &str) -> Arc<RwLock<dyn Font>> {
        if let Some(font) = self.name_to_id.get(name) {
            if let Some(font) = self.fonts.get(font) {
                if let Some(font) = font.upgrade() {
                    return font;
                }
            }
        }
        let font = Base::new_ttf(name);
        let name = name.to_string();
        let id = font.get_id();
        self.name_to_id.insert(name, id);
        let font: Arc<RwLock<dyn Font>> = Arc::new(RwLock::new(font));
        self.fonts.insert(id, Arc::downgrade(&font));
        return font;
    }
}

pub struct Base {
    pub obj_base: ObjectBase,
    pub name: Option<String>,
    pub font: TypeFont<'static>,
}

#[cfg(debug_mode)]
impl fmt::Debug for Base {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Base {{ obj_base: {:?}, name: {:?} }}",
            self.obj_base, self.name
        )
    }
}

impl Base {
    pub fn new_ttf(name: &str) -> Self {
        if !name.ends_with(".ttf") {
            vx_log_f!("Only TTF file is acceptable, not {}.", name);
        }
        let file = format!("data/fonts/{}", name);
        let mut file = vx_result!(File::open(&file));
        let mut data = Vec::new();
        vx_result!(file.read_to_end(&mut data));
        let font = vx_unwrap!(TypeFont::try_from_vec(data));
        let obj_base = ObjectBase::new();
        let name = Some(name.to_string());
        Base {
            obj_base,
            name,
            font,
        }
    }
}

impl Default for Base {
    fn default() -> Self {
        let font = include_bytes!(concat!(env!("OUT_DIR"), "/render/fonts/Ubuntu-B.ttf"));
        let font = font.to_vec();
        let font = vx_unwrap!(TypeFont::try_from_vec(font));
        let obj_base = ObjectBase::new();
        let name = None;
        Self {
            obj_base,
            name,
            font,
        }
    }
}

impl CoreObject for Base {
    fn get_id(&self) -> Id {
        self.obj_base.get_id()
    }
}

impl Font for Base {
    fn get_font(&self) -> &TypeFont {
        &self.font
    }
}
