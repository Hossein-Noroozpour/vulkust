use super::super::system::file::File;
use super::camera::Manager as CameraManager;
use super::engine::GraphicApiEngine;
use super::mesh::{Basic as BasicMesh, Mesh};
use super::object::Object;
use super::texture::Manager as TextureManager;
use std::collections::BTreeMap;
use std::io::BufReader;
use std::sync::{Arc, RwLock};

use gltf;
use math;

pub struct Manager {
    pub gapi_engine: Arc<RwLock<GraphicApiEngine>>,
    pub scenes: Vec<Arc<RwLock<Scene>>>,
    pub scenes_names: BTreeMap<String, usize>,
    pub texture_manager: Arc<RwLock<TextureManager>>,
}

impl Manager {
    pub fn new(gapi_engine: &Arc<RwLock<GraphicApiEngine>>) -> Self {
        let texture_manager = Arc::new(RwLock::new(TextureManager::new(gapi_engine)));
        Manager {
            gapi_engine: gapi_engine.clone(),
            scenes: Vec::new(),
            scenes_names: BTreeMap::new(),
            texture_manager,
        }
    }

    pub fn render(&self) {
        for scene in &self.scenes {
            vxresult!(scene.write()).update();
        }
        for scene in &self.scenes {
            vxresult!(scene.read()).render();
            // todo depth cleaning, and other related things in here
        }
    }

    pub fn load<S: 'static>(&mut self, file_name: &str, scene_name: &str) -> Arc<RwLock<S>>
    where
        S: Loadable,
    {
        let file = Self::load_gltf(file_name);
        let scene = Self::fetch_gltf_scene(&file, scene_name);
        let scene = Arc::new(RwLock::new(S::new_with_gltf(
            self.gapi_engine.clone(),
            &scene,
            &self.texture_manager,
            vxunwrap!(file.blob),
        )));
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
    fn new_with_gltf(
        Arc<RwLock<GraphicApiEngine>>,
        &gltf::Scene,
        &Arc<RwLock<TextureManager>>,
        &Vec<u8>,
    ) -> Self {
        vxunexpected!();
    }
}

#[repr(C)]
pub struct Uniform {
    pub vp: math::Matrix4<f32>,
}

impl Uniform {
    pub fn new() -> Self {
        let vp = math::Matrix4::new(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        );
        Uniform { vp }
    }
}

pub struct Basic {
    pub name: String,
    pub uniform: Uniform,
    pub camera_manager: Arc<RwLock<CameraManager>>,
    pub meshes: Vec<Arc<RwLock<Mesh>>>,
    pub gapi_engine: Arc<RwLock<GraphicApiEngine>>,
    pub render_enabled: bool,
}

impl Basic {
    pub fn new_with_gltf(
        gapi_engine: Arc<RwLock<GraphicApiEngine>>,
        scene: &gltf::Scene,
        texture_manager: &Arc<RwLock<TextureManager>>,
        data: &Vec<u8>,
    ) -> Self {
        let camera_manager = Arc::new(RwLock::new(CameraManager::new(gapi_engine.clone())));
        let name = vxunwrap_o!(scene.name()).to_string();
        let uniform = Uniform::new();
        let mut myself = Basic {
            name,
            uniform,
            camera_manager,
            meshes: Vec::new(),
            gapi_engine,
            render_enabled: true,
        };
        for node in scene.nodes() {
            myself.import_gltf_node(&node, texture_manager, data);
        }
        myself.meshes.shrink_to_fit();
        myself
    }

    pub fn import_gltf_node(
        &mut self,
        node: &gltf::scene::Node,
        texture_manager: &Arc<RwLock<TextureManager>>,
        data: &Vec<u8>,
    ) {
        if node.camera().is_some() {
            vxresult!(self.camera_manager.write()).load(node);
        } else if let Some(gltf_mesh) = node.mesh() {
            self.meshes
                .push(Arc::new(RwLock::new(BasicMesh::new_with_gltf(
                    &self.gapi_engine,
                    gltf_mesh,
                    texture_manager,
                    data,
                ))));
        } else {
            for node in node.children() {
                self.import_gltf_node(&node, texture_manager, data);
            }
        }
    }

    pub fn render(&self) {
        // todo get directional light
        // then create light frustums
        // then rendering meshes with light
        if !self.render_enabled {
            return;
        }
        for mesh in &self.meshes {
            let mesh: &mut Mesh = &mut *vxresult!(mesh.write());
            Mesh::render(mesh, &self.uniform);
        }
    }

    pub fn update(&mut self) {
        let camera_manager = vxresult!(self.camera_manager.read());
        if camera_manager.active_camera.is_none() {
            return;
        }
        let camera = vxunwrap!(camera_manager.active_camera);
        let camera = vxresult!(camera.read());
        self.uniform.vp = *camera.get_view_projection();
    }
}

pub struct Game {
    pub basic: Basic,
}

impl Game {}

impl Object for Game {
    fn name(&self) -> &str {
        &self.basic.name
    }

    fn render(&self) {
        self.basic.render();
    }

    fn update(&mut self) {
        self.basic.update();
    }
}

impl Scene for Game {}

impl Loadable for Game {
    fn new_with_gltf(
        gapi_engine: Arc<RwLock<GraphicApiEngine>>,
        scene: &gltf::Scene,
        texture_manager: &Arc<RwLock<TextureManager>>,
        data: &Vec<u8>,
    ) -> Self {
        let basic = Basic::new_with_gltf(gapi_engine, scene, texture_manager, data);
        Game { basic }
    }
}
