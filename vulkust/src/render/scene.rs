use super::super::core::object::Object as CoreObject;
use super::super::core::types::Id;
use super::super::system::file::File;
use super::camera::{Camera, DefaultCamera, Manager as CameraManager};
use super::engine::GraphicApiEngine;
use super::light::Manager as LightManager;
use super::mesh::{Base as MeshBase, DefaultMesh, Manager as MeshManager, Mesh};
use super::object::{Base as BaseObject, Object};
use super::texture::Manager as TextureManager;
use std::collections::BTreeMap;
use std::io::BufReader;
use std::sync::{Arc, RwLock, Weak};

use gltf;
use math;

pub trait Scene: Object {
    fn add_camera(&mut self, Arc<RwLock<Camera>>);
    fn add_mesh(&mut self, Arc<RwLock<Mesh>>);
}

pub trait Loadable: Scene + Sized {
    fn new_with_gltf(
        &Arc<RwLock<GraphicApiEngine>>,
        &Arc<RwLock<TextureManager>>,
        &Arc<RwLock<LightManager>>,
        &Arc<RwLock<CameraManager>>,
        &gltf::Scene,
        &Vec<u8>,
    ) -> Self;
}

pub trait DefaultScene: Scene + Sized {
    fn default() -> Self;
}

pub struct Manager {
    pub gapi_engine: Arc<RwLock<GraphicApiEngine>>,
    pub scenes: BTreeMap<Id, Weak<RwLock<Scene>>>,
    pub texture_manager: Arc<RwLock<TextureManager>>,
    pub light_manager: Arc<RwLock<LightManager>>,
    pub camera_manager: Arc<RwLock<CameraManager>>,
    pub mesh_manager: Arc<RwLock<MeshManager>>,
}

impl Manager {
    pub fn new(gapi_engine: &Arc<RwLock<GraphicApiEngine>>) -> Self {
        let texture_manager = Arc::new(RwLock::new(TextureManager::new(gapi_engine)));
        let light_manager = Arc::new(RwLock::new(LightManager::new()));
        let camera_manager = Arc::new(RwLock::new(CameraManager::new(gapi_engine)));
        let mesh_manager = Arc::new(RwLock::new(MeshManager::new(gapi_engine)));
        Manager {
            gapi_engine: gapi_engine.clone(),
            scenes: BTreeMap::new(),
            texture_manager,
            light_manager,
            camera_manager,
            mesh_manager,
        }
    }

    pub fn render(&self) {
        for (_, scene) in &self.scenes {
            let scene = vxunwrap_o!(scene.upgrade());
            vxresult!(scene.write()).update();
        }
        for (_, scene) in &self.scenes {
            let scene = vxunwrap_o!(scene.upgrade());
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
        self.add(&s);
        return scene;
    }

    pub fn create<S>(&mut self) -> Arc<RwLock<S>>
    where
        S: 'static + DefaultScene,
    {
        let scene = Arc::new(RwLock::new(S::default()));
        let s: Arc<RwLock<Scene>> = scene.clone();
        self.add(&s);
        scene
    }

    pub fn create_camera<C>(&self) -> Arc<RwLock<C>>
    where
        C: 'static + DefaultCamera,
    {
        vxresult!(self.camera_manager.write()).create()
    }

    pub fn create_mesh<M>(&self) -> Arc<RwLock<M>>
    where
        M: 'static + DefaultMesh,
    {
        vxresult!(self.mesh_manager.write()).create()
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

    pub fn add(&mut self, scene: &Arc<RwLock<Scene>>) {
        let id = vxresult!(scene.read()).get_id();
        self.scenes.insert(id, Arc::downgrade(scene));
    }

    pub fn remove_with_id(&mut self, id: Id) {
        self.scenes.remove(&id);
    }

    pub fn remove(&mut self, scene: Arc<RwLock<Scene>>) {
        self.remove_with_id(vxresult!(scene.read()).get_id());
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
    pub meshes: BTreeMap<Id, Arc<RwLock<Mesh>>>,
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
        let obj_base = BaseObject::new();
        let uniform = Uniform::new();
        let cameras = BTreeMap::new();
        let active_camera = None;
        let meshes = BTreeMap::new();
        let mut myself = Base {
            obj_base,
            uniform,
            cameras,
            active_camera,
            meshes,
        };
        for node in scene.nodes() {
            myself.import_gltf_node(
                &node,
                gapi_engine,
                texture_manager,
                camera_manager,
                light_manager,
                data,
            );
        }
        return myself;
    }

    pub fn import_gltf_node(
        &mut self,
        node: &gltf::scene::Node,
        gapi_engine: &Arc<RwLock<GraphicApiEngine>>,
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
            let mesh = MeshBase::new_with_gltf(gapi_engine, gltf_mesh, texture_manager, data);
            let id = mesh.get_id();
            let mesh = Arc::new(RwLock::new(mesh));
            self.meshes.insert(id, mesh);
        } else {
            for node in node.children() {
                self.import_gltf_node(
                    &node,
                    gapi_engine,
                    texture_manager,
                    camera_manager,
                    light_manager,
                    data,
                );
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
    fn render(&self) {
        // todo get directional light
        // then create light frustums
        // then rendering meshes with light
        if !self.obj_base.renderable {
            return;
        }
        for (_, mesh) in &self.meshes {
            let mesh: &mut Mesh = &mut *vxresult!(mesh.write());
            Mesh::render(mesh, &self.uniform);
        }
    }

    fn update(&mut self) {
        let camera = match &self.active_camera {
            Some(c) => c,
            None => return,
        };
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

impl Scene for Base {
    fn add_camera(&mut self, camera: Arc<RwLock<Camera>>) {
        let id = vxresult!(camera.read()).get_id();
        if self.active_camera.is_none() {
            self.active_camera = Some(Arc::downgrade(&camera));
        }
        self.cameras.insert(id, camera);
    }

    fn add_mesh(&mut self, mesh: Arc<RwLock<Mesh>>) {
        let id = vxresult!(mesh.read()).get_id();
        #[cfg(debug_assertions)]
        {
            if self.meshes.get(&id).is_some() {
                vxlogf!("Mesh is already added.\nDo not import same mesh twise");
            }
        }
        self.meshes.insert(id, mesh);
    }
}

impl DefaultScene for Base {
    fn default() -> Self {
        Base {
            obj_base: BaseObject::new(),
            uniform: Uniform::new(),
            cameras: BTreeMap::new(),
            active_camera: None,
            meshes: BTreeMap::new(),
        }
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

impl Scene for Game {
    fn add_camera(&mut self, camera: Arc<RwLock<Camera>>) {
        self.base.add_camera(camera);
    }

    fn add_mesh(&mut self, mesh: Arc<RwLock<Mesh>>) {
        self.base.add_mesh(mesh);
    }
}

impl Loadable for Game {
    fn new_with_gltf(
        gapi_engine: &Arc<RwLock<GraphicApiEngine>>,
        texture_manager: &Arc<RwLock<TextureManager>>,
        light_manager: &Arc<RwLock<LightManager>>,
        camera_manager: &Arc<RwLock<CameraManager>>,
        scene: &gltf::Scene,
        data: &Vec<u8>,
    ) -> Self {
        let base = Base::new_with_gltf(
            gapi_engine,
            scene,
            texture_manager,
            camera_manager,
            light_manager,
            data,
        );
        Game { base }
    }
}

pub struct Ui {
    pub base: Base,
}

impl Ui {}

impl CoreObject for Ui {
    fn get_id(&self) -> Id {
        self.base.get_id()
    }
}

impl Object for Ui {
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

impl Scene for Ui {
    fn add_camera(&mut self, camera: Arc<RwLock<Camera>>) {
        self.base.add_camera(camera)
    }

    fn add_mesh(&mut self, mesh: Arc<RwLock<Mesh>>) {
        self.base.add_mesh(mesh);
    }
}

impl Loadable for Ui {
    fn new_with_gltf(
        gapi_engine: &Arc<RwLock<GraphicApiEngine>>,
        texture_manager: &Arc<RwLock<TextureManager>>,
        light_manager: &Arc<RwLock<LightManager>>,
        camera_manager: &Arc<RwLock<CameraManager>>,
        scene: &gltf::Scene,
        data: &Vec<u8>,
    ) -> Self {
        let base = Base::new_with_gltf(
            gapi_engine,
            scene,
            texture_manager,
            camera_manager,
            light_manager,
            data,
        );
        Ui { base }
    }
}

impl DefaultScene for Ui {
    fn default() -> Self {
        let base = Base::default();
        Ui { base }
    }
}
