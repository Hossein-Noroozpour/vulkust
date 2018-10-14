use super::super::core::constants::{MAX_DIRECTIONAL_LIGHTS_COUNT, MAX_POINT_LIGHTS_COUNT};
use super::super::core::object::Object as CoreObject;
use super::super::core::types::{Id, TypeId as CoreTypeId};
use super::super::system::file::File;
use super::buffer::DynamicBuffer;
use super::camera::{Camera, DefaultCamera, Manager as CameraManager, Uniform as CameraUniform};
use super::command::Buffer as CmdBuffer;
use super::descriptor::Set as DescriptorSet;
use super::engine::Engine;
use super::font::Manager as FontManager;
use super::gx3d::{Gx3DReader, Table as Gx3dTable};
use super::light::{
    Directional as DirectionalLight, 
    DirectionalUniform, 
    Light, 
    Manager as LightManager,
    PointUniform, 
    ShadowMakerData,
};
use super::mesh::Manager as MeshManager;
use super::model::{Base as ModelBase, Manager as ModelManager, Model};
use super::object::{Base as ObjectBase, Loadable as ObjectLoadable, Object};
use super::texture::Manager as TextureManager;
use std::collections::BTreeMap;
use std::io::BufReader;
use std::mem::size_of;
use std::sync::{Arc, RwLock, Weak};

use gltf;

#[repr(u8)]
#[cfg_attr(debug_mode, derive(Debug))]
pub enum TypeId {
    GAME = 1,
    UI = 2,
}

pub trait Scene: Object {
    fn add_camera(&mut self, Arc<RwLock<Camera>>);
    fn add_model(&mut self, Arc<RwLock<Model>>);
    fn get_active_camera(&self) -> &Option<Weak<RwLock<Camera>>>;
    fn render_deferred(&self, cmd: &mut CmdBuffer, frame_buffer: usize);
    fn get_models(&self) -> &BTreeMap<Id, Arc<RwLock<Model>>>;
    fn get_all_models(&self) -> &BTreeMap<Id, Weak<RwLock<Model>>>;
    fn get_shadow_maker_lights_data(&self) -> BTreeMap<Id, Box<ShadowMakerData>>;
    fn update_shadow_maker_lights_data(&self, &BTreeMap<Id, Box<ShadowMakerData>>);
    fn update_shadow_makers(&self);
    fn clean(&mut self);
}

pub trait Loadable: Scene + Sized {
    fn new_with_gltf(&Engine, &gltf::Scene, &[u8]) -> Self;
    fn new_with_gx3d(&Engine, &mut Gx3DReader, Id) -> Self;
}

pub trait DefaultScene: Scene + Sized {
    fn default(&Engine) -> Self;
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Manager {
    pub engine: Option<Weak<RwLock<Engine>>>,
    pub scenes: Arc<RwLock<BTreeMap<Id, Weak<RwLock<Scene>>>>>,
    pub name_to_id: Arc<RwLock<BTreeMap<String, Id>>>,
    pub texture_manager: Arc<RwLock<TextureManager>>,
    pub light_manager: Arc<RwLock<LightManager>>,
    pub camera_manager: Arc<RwLock<CameraManager>>,
    pub mesh_manager: Arc<RwLock<MeshManager>>,
    pub font_manager: Arc<RwLock<FontManager>>,
    pub model_manager: Arc<RwLock<ModelManager>>,
    pub gx3d_table: Option<Arc<RwLock<Gx3dTable>>>,
}

impl Manager {
    pub fn new() -> Self {
        let texture_manager = Arc::new(RwLock::new(TextureManager::new()));
        let light_manager = Arc::new(RwLock::new(LightManager::new()));
        let camera_manager = Arc::new(RwLock::new(CameraManager::new()));
        let mesh_manager = Arc::new(RwLock::new(MeshManager::new()));
        let font_manager = Arc::new(RwLock::new(FontManager::new()));
        let model_manager = Arc::new(RwLock::new(ModelManager::new()));
        let scenes = Arc::new(RwLock::new(BTreeMap::new()));
        let name_to_id = Arc::new(RwLock::new(BTreeMap::new()));
        Manager {
            engine: None,
            scenes,
            name_to_id,
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

    pub fn load_gltf<S>(&self, file_name: &str, scene_name: &str) -> Arc<RwLock<S>>
    where
        S: 'static + Loadable,
    {
        let file = Self::load_gltf_struct(file_name);
        let scene = Self::fetch_gltf_scene(&file, scene_name);
        let scene = {
            let engine = vxunwrap!(&self.engine);
            let engine = vxunwrap!(engine.upgrade());
            let engine = vxresult!(engine.read());
            Arc::new(RwLock::new(S::new_with_gltf(
                &*engine,
                &scene,
                vxunwrap!(&file.blob),
            )))
        };
        let s: Arc<RwLock<Scene>> = scene.clone();
        self.add_scene(&s);
        return scene;
    }

    pub fn load_gx3d(&self, id: Id) -> Arc<RwLock<Scene>> {
        if let Some(scene) = vxresult!(self.scenes.read()).get(&id) {
            if let Some(scene) = scene.upgrade() {
                return scene;
            }
        }
        let scene: Arc<RwLock<Scene>> = {
            let mut table = vxunwrap!(&self.gx3d_table);
            let mut table = vxresult!(table.write());
            table.goto(id);
            let mut reader: &mut Gx3DReader = &mut table.reader;
            let type_id = reader.read_type_id();
            if type_id == TypeId::GAME as CoreTypeId {
                let engine = vxunwrap!(&self.engine);
                let engine = vxunwrap!(engine.upgrade());
                let engine = vxresult!(engine.read());
                Arc::new(RwLock::new(Game::new_with_gx3d(&engine, &mut reader, id)))
            } else if type_id == TypeId::UI as CoreTypeId {
                let engine = vxunwrap!(&self.engine);
                let engine = vxunwrap!(engine.upgrade());
                let engine = vxresult!(engine.read());
                Arc::new(RwLock::new(Ui::new_with_gx3d(&engine, &mut reader, id)))
            } else {
                vxunexpected!();
            }
        };
        self.add_scene(&scene);
        return scene;
    }

    pub fn create<S>(&self) -> Arc<RwLock<S>>
    where
        S: 'static + DefaultScene,
    {
        let scene = {
            let engine = vxunwrap!(&self.engine);
            let engine = vxunwrap!(engine.upgrade());
            let engine = vxresult!(engine.read());
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
        let engine = vxunwrap!(&self.engine);
        let engine = vxunwrap!(engine.upgrade());
        let engine = vxresult!(engine.read());
        vxresult!(self.camera_manager.write()).create(&*engine)
    }

    pub fn fetch_gltf_scene<'a>(file: &'a gltf::Gltf, scene_name: &str) -> gltf::Scene<'a> {
        let scenes = file.scenes();
        for scene in scenes {
            if vxunwrap!(scene.name()) == scene_name {
                return scene;
            }
        }
        vxunexpected!();
    }

    pub fn load_gltf_struct(file_name: &str) -> gltf::Gltf {
        let file = BufReader::new(vxresult!(File::open(file_name)));
        #[cfg(debug_mode)]
        return vxresult!(gltf::Gltf::from_reader(file));
        #[cfg(not(debug_mode))]
        return vxresult!(gltf::Gltf::from_reader_without_validation(file));
    }

    pub fn add_scene(&self, scene: &Arc<RwLock<Scene>>) {
        let id = {
            let scene = vxresult!(scene.read());
            let id = scene.get_id();
            if let Some(name) = scene.get_name() {
                vxresult!(self.name_to_id.write()).insert(name, id);
            }
            id
        };
        vxresult!(self.scenes.write()).insert(id, Arc::downgrade(scene));
    }

    pub fn remove_with_id(&self, id: Id) {
        vxresult!(self.scenes.write()).remove(&id);
    }

    pub fn remove(&self, scene: Arc<RwLock<Scene>>) {
        self.remove_with_id(vxresult!(scene.read()).get_id());
    }

    pub(super) fn get_scenes(&self) -> &Arc<RwLock<BTreeMap<Id, Weak<RwLock<Scene>>>>> {
        return &self.scenes;
    }
}

unsafe impl Send for Manager {}

unsafe impl Sync for Manager {}

#[repr(C)]
#[cfg_attr(debug_mode, derive(Debug))]
pub struct Uniform {
    pub camera: CameraUniform,
    pub directional_lights: [DirectionalUniform; MAX_DIRECTIONAL_LIGHTS_COUNT],
    pub directional_lights_count: u32,
    pub point_lights: [PointUniform; MAX_POINT_LIGHTS_COUNT],
    pub point_lights_count: u32,
}

impl Uniform {
    pub fn new() -> Self {
        let camera = CameraUniform::new();
        Uniform {
            camera,
            directional_lights: [DirectionalUniform::new(); MAX_DIRECTIONAL_LIGHTS_COUNT],
            directional_lights_count: 0,
            point_lights: [PointUniform::new(); MAX_POINT_LIGHTS_COUNT],
            point_lights_count: 0,
        }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Base {
    obj_base: ObjectBase,
    uniform: Uniform,
    uniform_buffer: Arc<RwLock<DynamicBuffer>>,
    cameras: BTreeMap<Id, Arc<RwLock<Camera>>>,
    active_camera: Option<Weak<RwLock<Camera>>>,
    shadow_makers: BTreeMap<Id, Arc<RwLock<Light>>>,
    lights: BTreeMap<Id, Arc<RwLock<Light>>>,
    models: BTreeMap<Id, Arc<RwLock<Model>>>,
    all_models: BTreeMap<Id, Weak<RwLock<Model>>>,
    descriptor_set: Arc<DescriptorSet>,
    cascaded_shadow_maps_count: usize,
    // pub skybox: Option<Arc<RwLock<Skybox>>>, // todo, maybe its not gonna be needed in GI PBR
    // pub constraints: BTreeMap<Id, Arc<RwLock<Constraint>>>, // todo
}

impl Base {
    pub fn new_with_gltf(engine: &Engine, scene: &gltf::Scene, data: &[u8]) -> Self {
        let cascaded_shadow_maps_count = engine.get_config().cascaded_shadows_count as usize;
        let camera_manager = {
            let manager = vxresult!(engine.scene_manager.read());
            manager.camera_manager.clone()
        };
        let obj_base = ObjectBase::new();
        let uniform = Uniform::new();
        let mut cameras = BTreeMap::new();
        let mut active_camera = None;
        let mut models = BTreeMap::new();
        let mut all_models = BTreeMap::new();
        let lights = BTreeMap::new();
        for node in scene.nodes() {
            if node.camera().is_some() {
                let camera = vxresult!(camera_manager.write()).load_gltf(&node, engine);
                let id = vxresult!(camera.read()).get_id();
                let w = Arc::downgrade(&camera);
                cameras.insert(id, camera);
                active_camera = Some(w);
            } else if let Some(_) = node.mesh() {
                let model = ModelBase::new_with_gltf(&node, engine, data);
                let child_models = model.bring_all_child_models();
                for (id, model) in child_models {
                    all_models.insert(id, Arc::downgrade(&model));
                }
                let id = model.get_id();
                let model: Arc<RwLock<Model>> = Arc::new(RwLock::new(model));
                all_models.insert(id, Arc::downgrade(&model));
                models.insert(id, model);
            } // todo read lights
        }
        let gapi_engine = vxresult!(engine.gapi_engine.read());
        let uniform_buffer = Arc::new(RwLock::new(
            vxresult!(gapi_engine.buffer_manager.write())
                .create_dynamic_buffer(size_of::<Uniform>() as isize),
        ));
        let mut descriptor_manager = vxresult!(gapi_engine.descriptor_manager.write());
        let descriptor_set = descriptor_manager.create_buffer_only_set(uniform_buffer.clone());
        let descriptor_set = Arc::new(descriptor_set);
        Base {
            obj_base,
            uniform,
            uniform_buffer,
            descriptor_set,
            cameras,
            active_camera,
            shadow_makers: BTreeMap::new(),
            models,
            all_models,
            lights,
            cascaded_shadow_maps_count,
        }
    }

    pub fn new_with_gx3d(eng: &Engine, reader: &mut Gx3DReader, my_id: Id) -> Self {
        let cascaded_shadow_maps_count = eng.get_config().cascaded_shadows_count as usize;
        let cameras_ids = reader.read_array::<Id>();
        let _audios_ids = reader.read_array::<Id>(); // todo
        let lights_ids = reader.read_array::<Id>();
        let models_ids = reader.read_array::<Id>();
        if reader.read_bool() {
            let _skybox_id: Id = reader.read();
        }
        let _constraits_ids = reader.read_array::<Id>(); // todo
        if reader.read_bool() {
            vxunimplemented!(); // todo
        }
        let manager = vxresult!(eng.scene_manager.read());
        let (camera_manager, light_manager, model_manager) = {
            (
                &manager.camera_manager,
                &manager.light_manager,
                &manager.model_manager,
            )
        };
        let mut cameras = BTreeMap::new();
        for id in &cameras_ids {
            cameras.insert(*id, vxresult!(camera_manager.write()).load_gx3d(eng, *id));
        }
        let active_camera = if cameras_ids.len() > 0 {
            Some(Arc::downgrade(
                &vxresult!(camera_manager.write()).load_gx3d(eng, cameras_ids[0]),
            ))
        } else {
            None
        };
        let mut models = BTreeMap::new();
        let mut all_models = BTreeMap::new();
        for id in models_ids {
            let model = vxresult!(model_manager.write()).load_gx3d(eng, id);
            {
                let model = vxresult!(model.read());
                let child_models = model.bring_all_child_models();
                for (id, model) in child_models {
                    all_models.insert(id, Arc::downgrade(&model));
                }
            }
            all_models.insert(id, Arc::downgrade(&model));
            models.insert(id, model);
        }
        let mut lights = BTreeMap::new();
        for id in lights_ids {
            lights.insert(id, vxresult!(light_manager.write()).load_gx3d(eng, id));
        }
        let uniform = Uniform::new();
        let gapi_engine = vxresult!(eng.gapi_engine.read());
        let uniform_buffer = Arc::new(RwLock::new(
            vxresult!(gapi_engine.buffer_manager.write())
                .create_dynamic_buffer(size_of::<Uniform>() as isize),
        ));
        let mut descriptor_manager = vxresult!(gapi_engine.descriptor_manager.write());
        let descriptor_set = descriptor_manager.create_buffer_only_set(uniform_buffer.clone());
        let descriptor_set = Arc::new(descriptor_set);
        vxtodo!(); // pick one of the directional lights as shadow maker
        Base {
            obj_base: ObjectBase::new_with_id(my_id),
            cameras,
            active_camera,
            models,
            all_models,
            lights,
            shadow_makers: BTreeMap::new(),
            uniform,
            uniform_buffer,
            descriptor_set,
            cascaded_shadow_maps_count,
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

    fn render(&self, cmd: &mut CmdBuffer, frame_number: usize) {
        // todo get directional light
        // then create light frustums
        // then rendering meshes with light
        self.obj_base.render(cmd, frame_number);
        let camera = match &self.active_camera {
            Some(c) => c,
            None => return,
        };
        let _camera = match camera.upgrade() {
            Some(c) => c,
            None => return,
        };
        {
            let mut uniform_buffer = vxresult!(self.uniform_buffer.write());
            uniform_buffer.update(&self.uniform, frame_number);
            let buffer = uniform_buffer.get_buffer(frame_number);
            let buffer = vxresult!(buffer.read());
            cmd.bind_gbuff_scene_descriptor(&*self.descriptor_set, &*buffer);
        }
    }

    fn update(&mut self) {
        let camera = match &self.active_camera {
            Some(c) => c,
            None => return,
        };
        let camera = match camera.upgrade() {
            Some(c) => c,
            None => return,
        };
        let camera = vxresult!(camera.read());
        camera.update_uniform(&mut self.uniform.camera);

        let csmws = camera.get_cascaded_shadow_frustum_partitions(self.cascaded_shadow_maps_count);
        for (_, shadow_maker) in &self.shadow_makers {
            let mut shadow_maker = vxresult!(shadow_maker.write());
            if !shadow_maker.is_rendarable() {
                continue;
            }
            // shadow maker must be directional (maybe in far future I gonna add others)
            let shadow_maker = vxunwrap!(shadow_maker.to_mut_directional());
            shadow_maker.update_cascaded_shadow_map_cameras(&csmws);
        }
        // todo update lights
    }

    fn disable_rendering(&mut self) {
        self.obj_base.disable_rendering()
    }

    fn enable_rendering(&mut self) {
        self.obj_base.enable_rendering()
    }

    fn is_rendarable(&self) -> bool {
        if self.obj_base.is_rendarable() {
            if let Some(camera) = &self.active_camera {
                if let Some(camera) = camera.upgrade() {
                    return vxresult!(camera.read()).is_rendarable();
                }
            }
        }
        return false;
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

    fn add_model(&mut self, model: Arc<RwLock<Model>>) {
        let id = {
            let model = vxresult!(model.read());
            let child_models = model.bring_all_child_models();
            for (id, model) in child_models {
                self.all_models.insert(id, Arc::downgrade(&model));
            }
            let id = model.get_id();
            id
        };
        self.all_models.insert(id, Arc::downgrade(&model));
        self.models.insert(id, model);
    }

    fn get_active_camera(&self) -> &Option<Weak<RwLock<Camera>>> {
        return &self.active_camera;
    }

    fn render_deferred(&self, cmd: &mut CmdBuffer, frame_number: usize) {
        let uniform_buffer = vxresult!(self.uniform_buffer.read());
        let buffer = uniform_buffer.get_buffer(frame_number);
        let buffer = vxresult!(buffer.read());
        cmd.bind_deferred_scene_descriptor(&*self.descriptor_set, &*buffer);
    }

    fn get_models(&self) -> &BTreeMap<Id, Arc<RwLock<Model>>> {
        return &self.models;
    }

    fn get_all_models(&self) -> &BTreeMap<Id, Weak<RwLock<Model>>> {
        return &self.all_models;
    }

    fn get_shadow_maker_lights_data(&self) -> BTreeMap<Id, Box<ShadowMakerData>> {
        let mut result = BTreeMap::new();
        for (id, shm) in &self.shadow_makers {
            let l = vxresult!(shm.read());
            if l.is_rendarable() {
                result.insert(*id, vxunwrap!(l.get_shadow_maker_data()));
            }
        }
        return result;
    }

    fn update_shadow_maker_lights_data(&self, smds: &BTreeMap<Id, Box<ShadowMakerData>>) {
        for (id, smd) in smds {
            let sm = self.shadow_makers.get(id);
            if let Some(sm) = sm {
                let mut sm = vxresult!(sm.write());
                sm.update_shadow_maker_data(smd);
            }
        }
    }

    fn update_shadow_makers(&self) {
        for (_, sm) in &self.shadow_makers {
            let mut sm = vxresult!(sm.write());
            sm.update();
        }
    }

    fn clean(&mut self) {
        let mut ids = Vec::<Id>::new();
        for (id, model) in &self.all_models {
            if model.upgrade().is_none() {
                ids.push(*id);
            }
        }
        for id in ids {
            self.all_models.remove(&id);
        }
    }
}

impl DefaultScene for Base {
    fn default(engine: &Engine) -> Self {
        let cascaded_shadow_maps_count = engine.get_config().cascaded_shadows_count as usize;
        let gapi_engine = vxresult!(engine.gapi_engine.read());
        let uniform_buffer = Arc::new(RwLock::new(
            vxresult!(gapi_engine.buffer_manager.write())
                .create_dynamic_buffer(size_of::<Uniform>() as isize),
        ));
        let mut descriptor_manager = vxresult!(gapi_engine.descriptor_manager.write());
        let descriptor_set = descriptor_manager.create_buffer_only_set(uniform_buffer.clone());
        let descriptor_set = Arc::new(descriptor_set);
        Base {
            obj_base: ObjectBase::new(),
            uniform: Uniform::new(),
            uniform_buffer,
            descriptor_set,
            cameras: BTreeMap::new(),
            active_camera: None,
            lights: BTreeMap::new(),
            models: BTreeMap::new(),
            all_models: BTreeMap::new(),
            shadow_makers: BTreeMap::new(),
            cascaded_shadow_maps_count,
        }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
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

    fn render(&self, cmd: &mut CmdBuffer, frame_number: usize) {
        self.base.render(cmd, frame_number);
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

    fn is_rendarable(&self) -> bool {
        return self.base.is_rendarable();
    }
}

impl Scene for Game {
    fn add_camera(&mut self, camera: Arc<RwLock<Camera>>) {
        self.base.add_camera(camera);
    }

    fn add_model(&mut self, model: Arc<RwLock<Model>>) {
        self.base.add_model(model);
    }

    fn get_active_camera(&self) -> &Option<Weak<RwLock<Camera>>> {
        return self.base.get_active_camera();
    }

    fn render_deferred(&self, cmd: &mut CmdBuffer, frame_number: usize) {
        self.base.render_deferred(cmd, frame_number);
    }

    fn get_models(&self) -> &BTreeMap<Id, Arc<RwLock<Model>>> {
        return self.base.get_models();
    }

    fn get_all_models(&self) -> &BTreeMap<Id, Weak<RwLock<Model>>> {
        return self.base.get_all_models();
    }

    fn get_shadow_maker_lights_data(&self) -> BTreeMap<Id, Box<ShadowMakerData>> {
        return self.base.get_shadow_maker_lights_data();
    }

    fn update_shadow_maker_lights_data(&self, smds: &BTreeMap<Id, Box<ShadowMakerData>>) {
        self.base.update_shadow_maker_lights_data(smds);
    }

    fn update_shadow_makers(&self) {
        self.base.update_shadow_makers();
    }

    fn clean(&mut self) {
        self.base.clean();
    }
}

impl Loadable for Game {
    fn new_with_gltf(engine: &Engine, scene: &gltf::Scene, data: &[u8]) -> Self {
        let base = Base::new_with_gltf(engine, scene, data);
        Game { base }
    }

    fn new_with_gx3d(engine: &Engine, reader: &mut Gx3DReader, my_id: Id) -> Self {
        let base = Base::new_with_gx3d(engine, reader, my_id);
        Game { base }
    }
}

impl DefaultScene for Game {
    fn default(engine: &Engine) -> Self {
        let base = Base::default(engine);
        Game { base }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
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

    fn render(&self, cmd: &mut CmdBuffer, frame_number: usize) {
        self.base.render(cmd, frame_number);
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

    fn is_rendarable(&self) -> bool {
        return self.base.is_rendarable();
    }
}

impl Scene for Ui {
    fn add_camera(&mut self, camera: Arc<RwLock<Camera>>) {
        self.base.add_camera(camera)
    }

    fn add_model(&mut self, model: Arc<RwLock<Model>>) {
        self.base.add_model(model);
    }

    fn get_active_camera(&self) -> &Option<Weak<RwLock<Camera>>> {
        return self.base.get_active_camera();
    }

    fn render_deferred(&self, cmd: &mut CmdBuffer, frame_number: usize) {
        self.base.render_deferred(cmd, frame_number);
    }

    fn get_models(&self) -> &BTreeMap<Id, Arc<RwLock<Model>>> {
        return self.base.get_models();
    }

    fn get_all_models(&self) -> &BTreeMap<Id, Weak<RwLock<Model>>> {
        return self.base.get_all_models();
    }

    fn get_shadow_maker_lights_data(&self) -> BTreeMap<Id, Box<ShadowMakerData>> {
        return self.base.get_shadow_maker_lights_data();
    }

    fn update_shadow_maker_lights_data(&self, smds: &BTreeMap<Id, Box<ShadowMakerData>>) {
        self.base.update_shadow_maker_lights_data(smds);
    }

    fn update_shadow_makers(&self) {
        self.base.update_shadow_makers();
    }

    fn clean(&mut self) {
        self.base.clean();
    }
}

impl Loadable for Ui {
    fn new_with_gltf(engine: &Engine, scene: &gltf::Scene, data: &[u8]) -> Self {
        let base = Base::new_with_gltf(engine, scene, data);
        Ui { base }
    }

    fn new_with_gx3d(engine: &Engine, reader: &mut Gx3DReader, my_id: Id) -> Self {
        let base = Base::new_with_gx3d(engine, reader, my_id);
        Ui { base }
    }
}

impl DefaultScene for Ui {
    fn default(engine: &Engine) -> Self {
        let base = Base::default(engine);
        Ui { base }
    }
}
