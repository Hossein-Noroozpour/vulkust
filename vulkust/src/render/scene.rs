use super::super::core::types::Id;
use super::super::core::object::Object as CoreObject;
use super::super::system::file::File;
use super::camera::{Camera, Manager as CameraManager};
use super::engine::GraphicApiEngine;
use super::light::Manager as LightManager;
use super::mesh::{Base as MeshBase, Mesh};
use super::object::{Base as BaseObject, Object};
use super::texture::Manager as TextureManager;
use std::collections::BTreeMap;
use std::io::BufReader;
use std::sync::{Arc, RwLock, Weak};

use gltf;
use math;

pub struct Manager {
    pub gapi_engine: Arc<RwLock<GraphicApiEngine>>,
    pub scenes: Vec<Arc<RwLock<Scene>>>,
    pub scenes_names: BTreeMap<String, usize>,
    pub texture_manager: Arc<RwLock<TextureManager>>,
    pub light_manager: Arc<RwLock<LightManager>>,
    pub camera_manager: Arc<RwLock<CameraManager>>,
}

impl Manager {
    pub fn new(gapi_engine: &Arc<RwLock<GraphicApiEngine>>) -> Self {
        let texture_manager = Arc::new(RwLock::new(TextureManager::new(gapi_engine)));
        let light_manager = Arc::new(RwLock::new(LightManager::new()));
        let camera_manager = Arc::new(RwLock::new(CameraManager::new(&gapi_engine)));
        Manager {
            gapi_engine: gapi_engine.clone(),
            scenes: Vec::new(),
            scenes_names: BTreeMap::new(),
            texture_manager,
            light_manager,
            camera_manager,
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

    pub fn load<S>(&mut self, file_name: &str, scene_name: &str) -> Arc<RwLock<S>>
    where
        S: 'static + Loadable,
    {
        let file = Self::load_gltf(file_name);
        let scene = Self::fetch_gltf_scene(&file, scene_name);
        let scene = Arc::new(RwLock::new(S::new_with_gltf(
            &self.gapi_engine,
            &self.texture_manager,
            &self.light_manager,
            &self.camera_manager,
            &scene,
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
        &Arc<RwLock<GraphicApiEngine>>,
        &Arc<RwLock<TextureManager>>,
        &Arc<RwLock<LightManager>>,
        &Arc<RwLock<CameraManager>>,
        &gltf::Scene,
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

pub struct Base {
    pub obj_base: BaseObject,
    pub uniform: Uniform,
    pub cameras: BTreeMap<Id, Arc<RwLock<Camera>>>,
    pub active_camera: Option<Weak<RwLock<Camera>>>,
    pub meshes: Vec<Arc<RwLock<Mesh>>>,
    pub gapi_engine: Weak<RwLock<GraphicApiEngine>>,
}

impl Base {
    pub fn new_with_gltf(
        gapi_engine: &Arc<RwLock<GraphicApiEngine>>,
        scene: &gltf::Scene,
        texture_manager: &Arc<RwLock<TextureManager>>,
        camera_manager: &Arc<RwLock<CameraManager>>,
        light_manager: &Arc<RwLock<LightManager>>,
        data: &Vec<u8>,
    ) -> Self {
        let obj_base = BaseObject::new(vxunwrap_o!(scene.name()));
        let uniform = Uniform::new();
        let gapi_engine = Arc::downgrade(gapi_engine);
        let cameras = BTreeMap::new();
        let active_camera = None;
        let meshes = Vec::new();
        let mut myself = Base {
            obj_base,
            uniform,
            cameras,
            active_camera,
            meshes,
            gapi_engine,
        };
        for node in scene.nodes() {
            myself.import_gltf_node(&node, texture_manager, camera_manager, light_manager, data);
        }
        myself.meshes.shrink_to_fit();
        return myself;
    }

    pub fn import_gltf_node(
        &mut self,
        node: &gltf::scene::Node,
        texture_manager: &Arc<RwLock<TextureManager>>,
        camera_manager: &Arc<RwLock<CameraManager>>,
        light_manager: &Arc<RwLock<LightManager>>,
        data: &Vec<u8>,
    ) {
        if node.camera().is_some() {
            let camera = vxresult!(camera_manager.write()).load(node);
            let id = vxresult!(camera.read()).get_id();
            let w = Arc::downgrade(&camera);
            self.cameras.insert(id, camera);
            self.active_camera = Some(w);
        } else if let Some(gltf_mesh) = node.mesh() {
            self.meshes
                .push(Arc::new(RwLock::new(MeshBase::new_with_gltf(
                    vxunwrap!(self.gapi_engine.upgrade()),
                    gltf_mesh,
                    texture_manager,
                    data,
                ))));
        } else {
            for node in node.children() {
                self.import_gltf_node(&node, texture_manager, camera_manager, light_manager, data);
            }
        }
    }
}

impl CoreObject for Base {
    fn get_id(&self) -> Id {
        self.obj_base.get_id()
    }
}

impl Object for Base {
    fn name(&self) -> &str {
        &self.obj_base.name()
    }

    fn render(&self) {
        // todo get directional light
        // then create light frustums
        // then rendering meshes with light
        if !self.obj_base.renderable {
            return;
        }
        for mesh in &self.meshes {
            let mesh: &mut Mesh = &mut *vxresult!(mesh.write());
            Mesh::render(mesh, &self.uniform);
        }
    }

    fn update(&mut self) {
        let camera = vxunwrap!(self.active_camera);
        let camera = vxunwrap_o!(camera.upgrade());
        let camera = vxresult!(camera.read());
        self.uniform.vp = *camera.get_view_projection();
    }

    fn disable_rendering(&mut self) {
        self.obj_base.disable_rendering()
    }

    fn enable_rendering(&mut self) {
        self.obj_base.enable_rendering()
    }
}

pub struct Game {
    pub base: Base,
}

impl Game {}

impl CoreObject for Game {
    fn get_id(&self) -> Id {
        self.base.get_id()
    }
}

impl Object for Game {
    fn name(&self) -> &str {
        &self.base.name()
    }

    fn render(&self) {
        self.base.render();
    }

    fn update(&mut self) {
        self.base.update();
    }

    fn disable_rendering(&mut self) {
        self.base.disable_rendering()
    }

    fn enable_rendering(&mut self) {
        self.base.enable_rendering()
    }
}

impl Scene for Game {}

impl Loadable for Game {
    fn new_with_gltf(
        gapi_engine: &Arc<RwLock<GraphicApiEngine>>,
        texture_manager: &Arc<RwLock<TextureManager>>,
        light_manager: &Arc<RwLock<LightManager>>,
        camera_manager: &Arc<RwLock<CameraManager>>,
        scene: &gltf::Scene,
        data: &Vec<u8>,
    ) -> Self {
        let base = Base::new_with_gltf(gapi_engine, scene, texture_manager, camera_manager, light_manager, data);
        Game { base }
    }
}
