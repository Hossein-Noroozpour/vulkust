use std::sync::{Arc, RwLock};
use std::mem::transmute;
use std::collections::BTreeMap;
use std::io::BufReader;
use super::super::super::core::application::ApplicationTrait;
use super::super::super::system::file::File;
use super::super::engine::GraphicApiEngine;
use super::{BasicScene, Scene};

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

    pub fn render(&mut self) {
        for scene in &mut self.gapi_engine {
            scene.render();
            vxtodo!(); // do depth cleaning, and other related things in here
        }
    }

    pub fn load(&mut self, file_name: &str, scene_name: &str) -> Arc<RwLock<Scene>> {
        let mut file = BufReader::new(vxresult!(File::open(file_name)));
        #[cfg(debug_assertions)]
        let file = gltf::Gltf::from_reader(file);
        let scenes = file.scenes();
        for scene in scenes {
            if vxunwrap_o!(scene.name()) == scene_name {
                let scene = Arc::new(RwLock::new(Scene::new_with_gltf_scene(scene)));
                let index = self.scenes.len();
                self.scenes.push(scene.clone());
                self.scenes_names.insert(scene_name.to_string(), index);
                return scene;
            }
        }
        vxunexpected!();
    }
}
