use super::super::super::core::gx3d::Table as Gx3dTable;
use super::super::super::core::types::{Id, TypeId as CoreTypeId};
use super::super::super::system::file::File;
use super::super::engine::Engine;
use super::game::Game;
use super::ui::Ui;
use super::{DefaultScene, Loadable, Scene, TypeId};
use std::collections::BTreeMap;
use std::io::BufReader;
use std::sync::{Arc, RwLock, Weak};

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Manager {
    engine: Option<Weak<RwLock<Engine>>>,
    scenes: BTreeMap<Id, Weak<RwLock<dyn Scene>>>,
    name_to_id: BTreeMap<String, Id>,
    gx3d_table: Option<Gx3dTable>,
}

impl Manager {
    pub(crate) fn new() -> Self {
        let scenes = BTreeMap::new();
        let name_to_id = BTreeMap::new();
        Manager {
            engine: None,
            scenes,
            name_to_id,
            gx3d_table: None,
        }
    }

    pub(crate) fn set_gx3d_table(&mut self, gx3d_table: Gx3dTable) {
        self.gx3d_table = Some(gx3d_table);
    }

    pub(crate) fn set_engine(&mut self, engine: Weak<RwLock<Engine>>) {
        self.engine = Some(engine);
    }

    pub fn load_gltf<S>(&mut self, file_name: &str, scene_name: &str) -> Arc<RwLock<S>>
    where
        S: 'static + Loadable,
    {
        let file = Self::load_gltf_struct(file_name);
        let scene = Self::fetch_gltf_scene(&file, scene_name);
        let scene = {
            let engine = vx_unwrap!(&self.engine);
            let engine = vx_unwrap!(engine.upgrade());
            let engine = vx_result!(engine.read());
            Arc::new(RwLock::new(S::new_with_gltf(
                &*engine,
                &scene,
                vx_unwrap!(&file.blob),
            )))
        };
        let s: Arc<RwLock<dyn Scene>> = scene.clone();
        self.add_scene(&s);
        return scene;
    }

    pub fn load_gx3d(&mut self, id: Id) -> Arc<RwLock<dyn Scene>> {
        let scene: Arc<RwLock<dyn Scene>> = {
            let table = vx_unwrap!(&mut self.gx3d_table);
            table.goto(id);
            let reader = table.get_mut_reader();
            let type_id = reader.read_type_id();
            if type_id == TypeId::GAME as CoreTypeId {
                let engine = vx_unwrap!(&self.engine);
                let engine = vx_unwrap!(engine.upgrade());
                let engine = vx_result!(engine.read());
                Arc::new(RwLock::new(Game::new_with_gx3d(&engine, reader, id)))
            } else if type_id == TypeId::UI as CoreTypeId {
                let engine = vx_unwrap!(&self.engine);
                let engine = vx_unwrap!(engine.upgrade());
                let engine = vx_result!(engine.read());
                Arc::new(RwLock::new(Ui::new_with_gx3d(&engine, reader, id)))
            } else {
                vx_unexpected!();
            }
        };
        self.add_scene(&scene);
        return scene;
    }

    pub fn create<S>(&mut self) -> Arc<RwLock<S>>
    where
        S: 'static + DefaultScene,
    {
        let scene = {
            let engine = vx_unwrap!(&self.engine);
            let engine = vx_unwrap!(engine.upgrade());
            let engine = vx_result!(engine.read());
            Arc::new(RwLock::new(S::default(&engine)))
        };
        let s: Arc<RwLock<dyn Scene>> = scene.clone();
        self.add_scene(&s);
        scene
    }

    pub fn fetch_gltf_scene<'a>(file: &'a gltf::Gltf, scene_name: &str) -> gltf::Scene<'a> {
        let scenes = file.scenes();
        for scene in scenes {
            if vx_unwrap!(scene.name()) == scene_name {
                return scene;
            }
        }
        vx_unexpected!();
    }

    pub fn load_gltf_struct(file_name: &str) -> gltf::Gltf {
        let file = BufReader::new(vx_result!(File::open(file_name)));
        #[cfg(debug_mode)]
        return vx_result!(gltf::Gltf::from_reader(file));
        #[cfg(not(debug_mode))]
        return vx_result!(gltf::Gltf::from_reader_without_validation(file));
    }

    pub fn add_scene(&mut self, scene: &Arc<RwLock<dyn Scene>>) {
        let id = {
            let scene = vx_result!(scene.read());
            let id = scene.get_id();
            if let Some(name) = scene.get_name() {
                self.name_to_id.insert(name, id);
            }
            id
        };
        self.scenes.insert(id, Arc::downgrade(scene));
    }

    pub fn remove_with_id(&mut self, id: &Id) {
        self.scenes.remove(id);
    }

    pub fn remove(&mut self, scene: Arc<RwLock<dyn Scene>>) {
        self.remove_with_id(&vx_result!(scene.read()).get_id());
    }

    pub(crate) fn get_scenes(&self) -> &BTreeMap<Id, Weak<RwLock<dyn Scene>>> {
        return &self.scenes;
    }
}

unsafe impl Send for Manager {}

unsafe impl Sync for Manager {}
