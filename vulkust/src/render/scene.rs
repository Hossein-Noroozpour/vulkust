use super::super::core::object::Object as CoreObject;
use super::super::core::types::{Id, TypeId as CoreTypeId};
use super::super::system::file::File;
use super::camera::{Camera, DefaultCamera, Manager as CameraManager};
use super::engine::Engine;
use super::font::Manager as FontManager;
use super::gx3d::{Gx3DReader, Table as Gx3dTable};
use super::light::Manager as LightManager;
use super::mesh::{Base as MeshBase, DefaultMesh, Manager as MeshManager, Mesh};
use super::model::Manager as ModelManager;
use super::object::{Base as BaseObject, Object};
use super::texture::Manager as TextureManager;
use std::collections::BTreeMap;
use std::io::BufReader;
use std::sync::{Arc, RwLock, Weak};

use gltf;
use math;

#[repr(u8)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum TypeId {
    GAME = 1,
    UI = 2,
}

pub trait Scene: Object {
    fn add_camera(&mut self, Arc<RwLock<Camera>>);
    fn add_mesh(&mut self, Arc<RwLock<Mesh>>);
}

pub trait Loadable: Scene + Sized {
    fn new_with_gltf(&Arc<RwLock<Engine>>, &gltf::Scene, &[u8]) -> Self;
    fn new_with_gx3d(&Arc<RwLock<Engine>>, &mut Gx3DReader) -> Self;
}

pub trait DefaultScene: Scene + Sized {
    fn default(&Arc<RwLock<Engine>>) -> Self;
}

pub struct Manager {
    pub engine: Option<Weak<RwLock<Engine>>>,
    pub scenes: BTreeMap<Id, Weak<RwLock<Scene>>>,
    pub name_to_id: BTreeMap<String, Id>,
    pub texture_manager: Arc<RwLock<TextureManager>>,
    pub light_manager: Arc<RwLock<LightManager>>,
    pub camera_manager: Arc<RwLock<CameraManager>>,
    pub mesh_manager: Arc<RwLock<MeshManager>>,
    pub font_manager: Arc<RwLock<FontManager>>,
    pub model_manager: Arc<RwLock<ModelManager>>,
    pub gx3d_table: Option<Gx3dTable>,
}

impl Manager {
    pub fn new() -> Self {
        let texture_manager = Arc::new(RwLock::new(TextureManager::new()));
        let light_manager = Arc::new(RwLock::new(LightManager::new()));
        let camera_manager = Arc::new(RwLock::new(CameraManager::new()));
        let mesh_manager = Arc::new(RwLock::new(MeshManager::new()));
        let font_manager = Arc::new(RwLock::new(FontManager::new()));
        let model_manager = Arc::new(RwLock::new(ModelManager::new()));
        Manager {
            engine: None,
            scenes: BTreeMap::new(),
            name_to_id: BTreeMap::new(),
            texture_manager,
            light_manager,
            camera_manager,
            mesh_manager,
            font_manager,
            model_manager,
            gx3d_table: None,
        }
    }

    pub fn set_engine(&mut self, engine: Weak<RwLock<Engine>>) {
        self.engine = Some(engine);
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

    pub fn load_gltf<S>(&mut self, file_name: &str, scene_name: &str) -> Arc<RwLock<S>>
    where
        S: 'static + Loadable,
    {
        let file = Self::load_gltf_struct(file_name);
        let scene = Self::fetch_gltf_scene(&file, scene_name);
        let scene = {
            let engine = vxunwrap!(self.engine);
            let engine = vxunwrap_o!(engine.upgrade());
            Arc::new(RwLock::new(S::new_with_gltf(
                &engine,
                &scene,
                vxunwrap!(file.blob),
            )))
        };
        let s: Arc<RwLock<Scene>> = scene.clone();
        self.add_scene(&s);
        return scene;
    }

    pub fn load_gx3d(&mut self, id: Id) -> Arc<RwLock<Scene>> {
        let mut table = vxunwrap!(self.gx3d_table);
        table.goto(id);
        let mut reader: &mut Gx3DReader = &mut table.reader;
        let type_id = reader.read_type_id();
        let scene: Arc<RwLock<Scene>> = if type_id == TypeId::GAME as CoreTypeId {
            let engine = vxunwrap!(self.engine);
            let engine = vxunwrap_o!(engine.upgrade());
            Arc::new(RwLock::new(Game::new_with_gx3d(&engine, &mut reader)))
        } else if type_id == TypeId::UI as CoreTypeId {
            let engine = vxunwrap!(self.engine);
            let engine = vxunwrap_o!(engine.upgrade());
            Arc::new(RwLock::new(Ui::new_with_gx3d(&engine, &mut reader)))
        } else {
            vxunexpected!();
        };
        self.add_scene(&scene);
        return scene;
    }


    pub fn create<S>(&mut self) -> Arc<RwLock<S>>
    where
        S: 'static + DefaultScene,
    {
        let scene = {
            let engine = vxunwrap!(self.engine);
            let engine = vxunwrap_o!(engine.upgrade());
            Arc::new(RwLock::new(S::default(&engine)))
        };
        let s: Arc<RwLock<Scene>> = scene.clone();
        self.add_scene(&s);
        scene
    }

    pub fn create_camera<C>(&self) -> Arc<RwLock<C>>
    where
        C: 'static + DefaultCamera,
    {
        let engine = vxunwrap!(self.engine);
        let engine = vxunwrap_o!(engine.upgrade());
        vxresult!(self.camera_manager.write()).create(&engine)
    }

    pub fn create_mesh<M>(&self) -> Arc<RwLock<M>>
    where
        M: 'static + DefaultMesh,
    {
        let engine = vxunwrap!(self.engine);
        let engine = vxunwrap_o!(engine.upgrade());
        vxresult!(self.mesh_manager.write()).create(&engine)
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

    pub fn load_gltf_struct(file_name: &str) -> gltf::Gltf {
        let file = BufReader::new(vxresult!(File::open(file_name)));
        #[cfg(debug_assertions)]
        return vxresult!(gltf::Gltf::from_reader(file));
        #[cfg(not(debug_assertions))]
        return vxresult!(gltf::Gltf::from_reader_without_validation(file));
    }

    pub fn add_scene(&mut self, scene: &Arc<RwLock<Scene>>) {
        let id = {
            let scene = vxresult!(scene.read());
            let id = scene.get_id();
            if let Some(name) = scene.get_name() {
                self.name_to_id.insert(name, id);
            }
            id
        };
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
    pub fn new_with_gltf(engine: &Arc<RwLock<Engine>>, scene: &gltf::Scene, data: &[u8]) -> Self {
        let (texture_manager, camera_manager, light_manager) = {
            let engine = vxresult!(engine.read());
            let manager = vxresult!(engine.scene_manager.read());
            (
                manager.texture_manager.clone(),
                manager.camera_manager.clone(),
                manager.light_manager.clone(),
            )
        };
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
                engine,
                &texture_manager,
                &camera_manager,
                &light_manager,
                data,
            );
        }
        return myself;
    }

    pub fn import_gltf_node(
        &mut self,
        node: &gltf::scene::Node,
        engine: &Arc<RwLock<Engine>>,
        texture_manager: &Arc<RwLock<TextureManager>>,
        camera_manager: &Arc<RwLock<CameraManager>>,
        light_manager: &Arc<RwLock<LightManager>>,
        data: &[u8],
    ) {
        if node.camera().is_some() {
            let camera = vxresult!(camera_manager.write()).load_gltf(node, engine);
            let id = vxresult!(camera.read()).get_id();
            let w = Arc::downgrade(&camera);
            self.cameras.insert(id, camera);
            self.active_camera = Some(w);
        } else if let Some(gltf_mesh) = node.mesh() {
            let mesh = MeshBase::new_with_gltf(engine, gltf_mesh, texture_manager, data);
            let id = mesh.get_id();
            let mesh = Arc::new(RwLock::new(mesh));
            self.meshes.insert(id, mesh);
        } else {
            for node in node.children() {
                self.import_gltf_node(
                    &node,
                    engine,
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
    fn get_name(&self) -> Option<String> {
        self.obj_base.get_name()
    }

    fn set_name(&mut self, name: &str) {
        self.obj_base.set_name(name);
        vxunimplemented!(); //it must update corresponding manager
    }

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
    fn default(_: &Arc<RwLock<Engine>>) -> Self {
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
    fn get_name(&self) -> Option<String> {
        self.base.get_name()
    }

    fn set_name(&mut self, name: &str) {
        self.base.set_name(name);
        vxunimplemented!(); //it must update corresponding manager
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

impl Scene for Game {
    fn add_camera(&mut self, camera: Arc<RwLock<Camera>>) {
        self.base.add_camera(camera);
    }

    fn add_mesh(&mut self, mesh: Arc<RwLock<Mesh>>) {
        self.base.add_mesh(mesh);
    }
}

impl Loadable for Game {
    fn new_with_gltf(engine: &Arc<RwLock<Engine>>, scene: &gltf::Scene, data: &[u8]) -> Self {
        let base = Base::new_with_gltf(engine, scene, data);
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
    fn get_name(&self) -> Option<String> {
        self.base.get_name()
    }

    fn set_name(&mut self, name: &str) {
        self.base.set_name(name);
        vxunimplemented!(); //it must update corresponding manager
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

impl Scene for Ui {
    fn add_camera(&mut self, camera: Arc<RwLock<Camera>>) {
        self.base.add_camera(camera)
    }

    fn add_mesh(&mut self, mesh: Arc<RwLock<Mesh>>) {
        self.base.add_mesh(mesh);
    }
}

impl Loadable for Ui {
    fn new_with_gltf(engine: &Arc<RwLock<Engine>>, scene: &gltf::Scene, data: &[u8]) -> Self {
        let base = Base::new_with_gltf(engine, scene, data);
        Ui { base }
    }
}

impl DefaultScene for Ui {
    fn default(engine: &Arc<RwLock<Engine>>) -> Self {
        let base = Base::default(engine);
        Ui { base }
    }
}
