use super::super::system::file::File;
use super::engine::GraphicApiEngine;
use super::object::Object;
use std::collections::BTreeMap;
use std::io::BufReader;
use std::sync::{Arc, RwLock};

use gltf;

pub struct Manager {
    pub gapi_engine: Arc<RwLock<GraphicApiEngine>>,
    pub scenes: Vec<Arc<RwLock<Scene>>>,
    pub scenes_names: BTreeMap<String, usize>,
}

impl Manager {
    pub fn new(gapi_engine: &Arc<RwLock<GraphicApiEngine>>) -> Self {
        Manager {
            gapi_engine: gapi_engine.clone(),
            scenes: Vec::new(),
            scenes_names: BTreeMap::new(),
        }
    }

    pub fn render(&self) {
        for scene in &self.scenes {
            vxresult!(scene.read()).render();
            vxtodo!(); // do depth cleaning, and other related things in here
        }
    }

    pub fn load<S: 'static>(&mut self, file_name: &str, scene_name: &str) -> Arc<RwLock<S>>
    where
        S: Loadable,
    {
        let file = Self::load_gltf(file_name);
        let scene = Self::fetch_gltf_scene(&file, scene_name);
        let scene = Arc::new(RwLock::new(S::new_with_gltf_scene(scene)));
        let s: Arc<RwLock<Scene>> = scene.clone();
        self.add(s);
        return scene;
    }

    pub fn fetch_gltf_scene<'a>(file: &'a gltf::Gltf, scene_name: &str) -> gltf::Scene<'a> {
        let scenes = file.scenes();
        for scene in scenes {
            if vxunwrap_o!(scene.name()) == scene_name {
                return scene;
            }
        }
        vxunexpected!();
    }

    pub fn load_gltf(file_name: &str) -> gltf::Gltf {
        let file = BufReader::new(vxresult!(File::open(file_name)));
        #[cfg(debug_assertions)]
        return vxresult!(gltf::Gltf::from_reader(file));
        #[cfg(not(debug_assertions))]
        return vxresult!(gltf::Gltf::from_reader_without_validation(file));
    }

    pub fn add(&mut self, scene: Arc<RwLock<Scene>>) {
        let index = self.scenes.len();
        self.scenes_names
            .insert(vxresult!(scene.read()).name().to_string(), index);
        self.scenes.push(scene);
    }

    pub fn remove_with_name(&mut self, scene_name: &str) {
        let index = *vxunwrap_o!(self.scenes_names.get(&scene_name.to_string()));
        self.scenes.remove(index);
    }

    pub fn remove(&mut self, scene: Arc<RwLock<Scene>>) {
        self.remove_with_name(&vxresult!(scene.read()).name());
    }

    pub fn get_scene_layer_no(&self, scene: Arc<RwLock<Scene>>) -> usize {
        self.get_scene_layer_no_with_name(vxresult!(scene.read()).name())
    }

    pub fn get_scene_layer_no_with_name(&self, scene_name: &str) -> usize {
        *vxunwrap_o!(self.scenes_names.get(&scene_name.to_string()))
    }
}

pub trait Scene: Object {}

pub trait Loadable: Scene + Sized {
    fn new_with_gltf_scene(gltf::Scene) -> Self {
        vxunexpected!();
    }
}

pub struct Game {
    pub name: String,
}

impl Object for Game {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Scene for Game {}

impl Loadable for Game {
    fn new_with_gltf_scene(scene: gltf::Scene) -> Self {
        let name = vxunwrap_o!(scene.name()).to_string();
        Game { name }
    }
}
