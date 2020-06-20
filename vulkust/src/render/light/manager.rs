use super::super::super::core::gx3d::{Gx3DReader, Table as Gx3dTable};
use super::super::super::core::types::Id;
use super::super::engine::Engine;
use super::super::object::Loadable;
use super::{DefaultLighting, DirectionalBase, Light, PointBase, Sun, TypeId};
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock, Weak};

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Manager {
    engine: Option<Weak<RwLock<Engine>>>,
    lights: BTreeMap<Id, Weak<RwLock<dyn Light>>>,
    name_to_id: BTreeMap<String, Id>,
    gx3d_table: Option<Gx3dTable>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            engine: None,
            lights: BTreeMap::new(),
            name_to_id: BTreeMap::new(),
            gx3d_table: None,
        }
    }

    pub(crate) fn set_gx3d_table(&mut self, gx3d_table: Gx3dTable) {
        self.gx3d_table = Some(gx3d_table);
    }

    pub fn create<L>(&mut self) -> Arc<RwLock<L>>
    where
        L: 'static + Light + DefaultLighting,
    {
        let eng = vx_unwrap!(vx_unwrap!(&self.engine).upgrade());
        let eng = vx_result!(eng.read());
        let result = L::default(&*eng);
        let id = result.get_id();
        let result = Arc::new(RwLock::new(result));
        let light: Arc<RwLock<dyn Light>> = result.clone();
        self.lights.insert(id, Arc::downgrade(&light));
        return result;
    }

    pub fn load_gx3d(&mut self, eng: &Engine, id: Id) -> Arc<RwLock<dyn Light>> {
        if let Some(light) = self.lights.get(&id) {
            if let Some(light) = light.upgrade() {
                return light;
            }
        }
        let table = vx_unwrap!(&mut self.gx3d_table);
        table.goto(id);
        let reader: &mut Gx3DReader = table.get_mut_reader();
        let type_id = reader.read_type_id();
        let result: Arc<RwLock<dyn Light>> = if type_id == TypeId::Sun as u8 {
            if reader.read_bool() {
                Arc::new(RwLock::new(Sun::new_with_gx3d(eng, reader, id)))
            } else {
                Arc::new(RwLock::new(DirectionalBase::new_with_gx3d(eng, reader, id)))
            }
        } else if type_id == TypeId::Lamp as u8 {
            if reader.read_bool() {
                vx_unimplemented!();
            } else {
                Arc::new(RwLock::new(PointBase::new_with_gx3d(eng, reader, id)))
            }
        } else {
            vx_unexpected!();
        };
        self.lights.insert(id, Arc::downgrade(&result));
        return result;
    }

    pub(crate) fn set_engine(&mut self, e: Weak<RwLock<Engine>>) {
        self.engine = Some(e);
    }
}
