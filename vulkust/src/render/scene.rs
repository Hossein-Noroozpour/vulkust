use super::super::core::constants::{MAX_DIRECTIONAL_LIGHTS_COUNT, MAX_POINT_LIGHTS_COUNT};
use super::super::core::gx3d::{Gx3DReader, Table as Gx3dTable};
use super::super::core::object::Object as CoreObject;
use super::super::core::types::{Id, TypeId as CoreTypeId};
use super::super::system::file::File;
use super::buffer::DynamicBuffer;
use super::camera::{Camera, Uniform as CameraUniform};
use super::command::{Buffer as CmdBuffer, Pool as CmdPool};
use super::deferred::Deferred;
use super::descriptor::Set as DescriptorSet;
use super::engine::Engine;
use super::g_buffer_filler::GBufferFiller;
use super::gapi::GraphicApiEngine;
use super::light::{DirectionalUniform, Light, PointUniform};
use super::model::{Base as ModelBase, Model};
use super::object::{Base as ObjectBase, Loadable as ObjectLoadable, Object};
use super::resolver::Resolver;
use super::shadower::Shadower;
use super::sync::Semaphore;
use std::collections::BTreeMap;
use std::io::BufReader;
use std::mem::size_of;
use std::sync::{Arc, Mutex, RwLock, Weak};

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
    fn add_light(&mut self, Arc<RwLock<Light>>);
    fn get_active_camera(&self) -> &Option<Weak<RwLock<Camera>>>;
    fn get_models(&self) -> &BTreeMap<Id, Arc<RwLock<Model>>>;
    fn get_all_models(&self) -> &BTreeMap<Id, Weak<RwLock<Model>>>;
    fn update(&mut self, frame_number: usize);
    fn render_gbuffer_shadow_maps(
        &self,
        &GraphicApiEngine,
        &Arc<CmdPool>,
        &GBufferFiller,
        &Shadower,
        usize,
    );
    fn update_shadow_makers(&self);
    fn render_shadow_maps(&self, usize, usize);
    fn clean(&mut self);
    fn submit(
        &mut self,
        &GraphicApiEngine,
        &Arc<Semaphore>,
        &Arc<CmdPool>,
        &GBufferFiller,
        &Resolver,
        &Shadower,
        &Deferred,
    ) -> Arc<Semaphore>;
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
    engine: Option<Weak<RwLock<Engine>>>,
    scenes: BTreeMap<Id, Weak<RwLock<Scene>>>,
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

    pub fn load_gx3d(&mut self, id: Id) -> Arc<RwLock<Scene>> {
        let scene: Arc<RwLock<Scene>> = {
            let mut table = vxunwrap!(&mut self.gx3d_table);
            table.goto(id);
            let reader = table.get_mut_reader();
            let type_id = reader.read_type_id();
            if type_id == TypeId::GAME as CoreTypeId {
                let engine = vxunwrap!(&self.engine);
                let engine = vxunwrap!(engine.upgrade());
                let engine = vxresult!(engine.read());
                Arc::new(RwLock::new(Game::new_with_gx3d(&engine, reader, id)))
            } else if type_id == TypeId::UI as CoreTypeId {
                let engine = vxunwrap!(&self.engine);
                let engine = vxunwrap!(engine.upgrade());
                let engine = vxresult!(engine.read());
                Arc::new(RwLock::new(Ui::new_with_gx3d(&engine, reader, id)))
            } else {
                vxunexpected!();
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
            let engine = vxunwrap!(&self.engine);
            let engine = vxunwrap!(engine.upgrade());
            let engine = vxresult!(engine.read());
            Arc::new(RwLock::new(S::default(&engine)))
        };
        let s: Arc<RwLock<Scene>> = scene.clone();
        self.add_scene(&s);
        scene
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

    pub fn remove_with_id(&mut self, id: &Id) {
        self.scenes.remove(id);
    }

    pub fn remove(&mut self, scene: Arc<RwLock<Scene>>) {
        self.remove_with_id(&vxresult!(scene.read()).get_id());
    }

    pub(super) fn get_scenes(&self) -> &BTreeMap<Id, Weak<RwLock<Scene>>> {
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
        Self {
            camera,
            directional_lights: [DirectionalUniform::new(); MAX_DIRECTIONAL_LIGHTS_COUNT],
            directional_lights_count: 0,
            point_lights: [PointUniform::new(); MAX_POINT_LIGHTS_COUNT],
            point_lights_count: 0,
        }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
struct BaseKernelFramedata {
    gbuff: CmdBuffer,
}

#[cfg_attr(debug_mode, derive(Debug))]
struct BaseKernelData {
    frames_data: Vec<BaseKernelFramedata>,
}

#[cfg_attr(debug_mode, derive(Debug))]
struct BaseFramedata {
    gbuffer: CmdBuffer,
    gbuffer_semaphore: Arc<Semaphore>,
    resolver: CmdBuffer,
    resolver_secondary: CmdBuffer,
    resolver_semaphore: Arc<Semaphore>,
    deferred: CmdBuffer,
    deferred_secondary: CmdBuffer,
    deferred_semaphore: Arc<Semaphore>,
}

impl BaseFramedata {
    fn new(engine: &GraphicApiEngine, cmd_pool: &Arc<CmdPool>) -> Self {
        let gbuffer = engine.create_primary_command_buffer(cmd_pool.clone());
        let gbuffer_semaphore = Arc::new(engine.create_semaphore());
        let resolver = engine.create_primary_command_buffer(cmd_pool.clone());
        let resolver_secondary = engine.create_secondary_command_buffer(cmd_pool.clone());
        let resolver_semaphore = Arc::new(engine.create_semaphore());
        let deferred = engine.create_primary_command_buffer(cmd_pool.clone());
        let deferred_secondary = engine.create_secondary_command_buffer(cmd_pool.clone());
        let deferred_semaphore = Arc::new(engine.create_semaphore());
        Self {
            gbuffer,
            gbuffer_semaphore,
            resolver,
            resolver_secondary,
            resolver_semaphore,
            deferred,
            deferred_secondary,
            deferred_semaphore,
        }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Base {
    obj_base: ObjectBase,
    uniform: Uniform,
    uniform_buffer: DynamicBuffer,
    cameras: BTreeMap<Id, Arc<RwLock<Camera>>>,
    active_camera: Option<Weak<RwLock<Camera>>>,
    shadow_maker_lights: BTreeMap<Id, Arc<RwLock<Light>>>,
    lights: BTreeMap<Id, Arc<RwLock<Light>>>,
    models: BTreeMap<Id, Arc<RwLock<Model>>>,
    all_models: BTreeMap<Id, Weak<RwLock<Model>>>,
    descriptor_set: Arc<DescriptorSet>,
    kernels_data: Vec<Arc<Mutex<BaseKernelData>>>,
    frames_data: Vec<BaseFramedata>,
    // pub skybox: Option<Arc<RwLock<Skybox>>>, // todo, maybe its not gonna be needed in GI PBR
    // pub constraints: BTreeMap<Id, Arc<RwLock<Constraint>>>, // todo
}

impl Base {
    pub fn new_with_gltf(engine: &Engine, scene: &gltf::Scene, data: &[u8]) -> Self {
        let camera_manager = engine.get_asset_manager().get_camera_manager();
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
        let gapi_engine = vxresult!(engine.get_gapi_engine().read());
        let uniform_buffer = vxresult!(gapi_engine.get_buffer_manager().write())
            .create_dynamic_buffer(size_of::<Uniform>() as isize);
        let mut descriptor_manager = vxresult!(gapi_engine.get_descriptor_manager().write());
        let descriptor_set = descriptor_manager.create_buffer_only_set(&uniform_buffer);
        let descriptor_set = Arc::new(descriptor_set);
        let frames_count = gapi_engine.get_frames_count();
        let kernels_count = num_cpus::get();
        let mut kernels_data = Vec::with_capacity(kernels_count);
        for _ in 0..kernels_count {
            kernels_data.push(Arc::new(Mutex::new(BaseKernelData {
                frames_data: Vec::with_capacity(frames_count),
            })));
        }
        Self {
            obj_base,
            uniform,
            uniform_buffer,
            descriptor_set,
            cameras,
            active_camera,
            shadow_maker_lights: BTreeMap::new(),
            lights,
            models,
            all_models,
            kernels_data,
            frames_data: Vec::new(),
        }
    }

    pub fn new_with_gx3d(eng: &Engine, reader: &mut Gx3DReader, my_id: Id) -> Self {
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
        let asset_manager = eng.get_asset_manager();
        let camera_manager = asset_manager.get_camera_manager();
        let light_manager = asset_manager.get_light_manager();
        let model_manager = asset_manager.get_model_manager();
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
        let gapi_engine = vxresult!(eng.get_gapi_engine().read());
        let uniform_buffer = vxresult!(gapi_engine.get_buffer_manager().write())
            .create_dynamic_buffer(size_of::<Uniform>() as isize);
        let mut descriptor_manager = vxresult!(gapi_engine.get_descriptor_manager().write());
        let descriptor_set = descriptor_manager.create_buffer_only_set(&uniform_buffer);
        let descriptor_set = Arc::new(descriptor_set);
        let frames_count = gapi_engine.get_frames_count();
        let kernels_count = num_cpus::get();
        let mut kernels_data = Vec::with_capacity(kernels_count);
        for _ in 0..kernels_count {
            kernels_data.push(Arc::new(Mutex::new(BaseKernelData {
                frames_data: Vec::with_capacity(frames_count),
            })));
        }
        vxtodo!(); // take care of shadow mapper lights
        Self {
            obj_base: ObjectBase::new_with_id(my_id),
            uniform,
            uniform_buffer,
            descriptor_set,
            cameras,
            active_camera,
            models,
            all_models,
            shadow_maker_lights: BTreeMap::new(),
            lights,
            kernels_data,
            frames_data: Vec::new(),
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

    fn add_light(&mut self, light: Arc<RwLock<Light>>) {
        let (id, is_shadow_maker) = {
            let light = vxresult!(light.read());
            let is_shadow_maker = light.to_shadow_maker().is_some();
            let id = light.get_id();
            (id, is_shadow_maker)
        };
        if is_shadow_maker {
            self.shadow_maker_lights.insert(id, light);
        } else {
            self.lights.insert(id, light);
        }
    }

    fn get_active_camera(&self) -> &Option<Weak<RwLock<Camera>>> {
        return &self.active_camera;
    }

    fn update(&mut self, frame_number: usize) {
        if !self.is_rendarable() {
            return;
        }
        let camera = vxunwrap!(&self.active_camera);
        let camera = vxunwrap!(camera.upgrade());
        let camera = vxresult!(camera.read());
        camera.update_uniform(&mut self.uniform.camera);
        let mut last_directional_light_index = 0;
        let mut last_point_light_index = 0;
        let csmws = camera.get_cascaded_shadow_frustum_partitions();
        for (_, shm) in &self.shadow_maker_lights {
            let mut shm = vxresult!(shm.write());
            if !shm.is_rendarable() {
                continue;
            }
            {
                if let Some(shm) = shm.to_mut_directional() {
                    shm.update_cascaded_shadow_map_cameras(&csmws);
                    shm.update_uniform(
                        &mut self.uniform.directional_lights[last_directional_light_index],
                    );
                    last_directional_light_index += 1;
                    continue;
                }
            }
            {
                if let Some(shm) = shm.to_point() {
                    shm.update_uniform(&mut self.uniform.point_lights[last_point_light_index]);
                    last_point_light_index += 1;
                }
                continue;
            }
        }
        for (_, l) in &self.lights {
            let mut l = vxresult!(l.read());
            if !l.is_rendarable() {
                continue;
            }
            if let Some(l) = l.to_directional() {
                l.update_uniform(
                    &mut self.uniform.directional_lights[last_directional_light_index],
                );
                last_directional_light_index += 1;
            } else if let Some(l) = l.to_point() {
                l.update_uniform(&mut self.uniform.point_lights[last_point_light_index]);
                last_point_light_index += 1;
            }
        }
        self.uniform.directional_lights_count = last_directional_light_index as u32;
        self.uniform.point_lights_count = last_point_light_index as u32;
        self.uniform_buffer.update(&self.uniform, frame_number);
    }

    fn update_shadow_makers(&self) {
        for (_, shm) in &self.shadow_maker_lights {
            let mut shm = vxresult!(shm.write());
            if !shm.is_rendarable() {
                continue;
            }
            shm.update();
        }
    }

    fn render_gbuffer_shadow_maps(
        &self,
        geng: &GraphicApiEngine,
        cmd_pool: &Arc<CmdPool>,
        g_buffer_filler: &GBufferFiller,
        shadower: &Shadower,
        kernel_index: usize,
    ) {
        if !self.is_rendarable() {
            return;
        }
        let frame_number = geng.get_frame_number();
        for (_, shm) in &self.shadow_maker_lights {
            vxunwrap!(vxresult!(shm.read()).to_shadow_maker()).begin_secondary_commands(
                geng,
                cmd_pool,
                shadower,
                kernel_index,
                frame_number,
            );
        }
        let kernels_count = self.kernels_data.len();
        let mut kernel_data = vxresult!(self.kernels_data[kernel_index].lock());
        if kernel_data.frames_data.len() < 1 {
            let frames_count = geng.get_frames_count();
            for _ in 0..frames_count {
                kernel_data.frames_data.push(BaseKernelFramedata {
                    gbuff: geng.create_secondary_command_buffer(cmd_pool.clone()),
                });
            }
        }
        let cmd = &mut kernel_data.frames_data[frame_number].gbuff;
        g_buffer_filler.begin_secondary(cmd);
        {
            let buffer = self.uniform_buffer.get_buffer(frame_number);
            let buffer = vxresult!(buffer.read());
            cmd.bind_gbuff_scene_descriptor(&*self.descriptor_set, &*buffer);
        }
        let mut task_index = 0;
        for (_, model) in &self.all_models {
            let camera = vxunwrap!(&self.active_camera).upgrade();
            let camera = vxunwrap!(camera);
            let camera = vxresult!(camera.read());
            task_index += 1;
            if task_index % kernels_count != kernel_index {
                continue;
            }
            let model = model.upgrade();
            if model.is_none() {
                continue;
            }
            let m = vxunwrap!(model);
            let mut model = vxresult!(m.write());
            if !model.is_rendarable() {
                continue;
            }
            model.update(self, &*camera, frame_number);
            model.render_gbuffer(cmd, frame_number);
            if model.has_shadow() {
                for (_, shm) in &self.shadow_maker_lights {
                    vxunwrap!(vxresult!(shm.read()).to_shadow_maker()).shadow(
                        &mut *model,
                        &m,
                        kernel_index,
                    );
                }
            }
        }
        cmd.end();
    }

    fn render_shadow_maps(&self, kernel_index: usize, frame_number: usize) {
        for (_, shm) in &self.shadow_maker_lights {
            vxunwrap!(vxresult!(shm.read()).to_shadow_maker())
                .render_shadow_mapper(kernel_index, frame_number);
        }
    }

    fn get_models(&self) -> &BTreeMap<Id, Arc<RwLock<Model>>> {
        return &self.models;
    }

    fn get_all_models(&self) -> &BTreeMap<Id, Weak<RwLock<Model>>> {
        return &self.all_models;
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

    fn submit(
        &mut self,
        geng: &GraphicApiEngine,
        sem: &Arc<Semaphore>,
        cmd_pool: &Arc<CmdPool>,
        g_buffer_filler: &GBufferFiller,
        resolver: &Resolver,
        shadower: &Shadower,
        deferred: &Deferred,
    ) -> Arc<Semaphore> {
        if !self.is_rendarable() {
            return sem.clone();
        }
        let frame_number = geng.get_frame_number();
        let frames_count = geng.get_frames_count();
        let frames_data_len = self.frames_data.len();
        for _ in frames_data_len..frames_count {
            self.frames_data.push(BaseFramedata::new(geng, cmd_pool));
        }
        let frame_data = &mut self.frames_data[frame_number];
        // g-buffer
        {
            let mut kernels_gbuffer_commands = Vec::with_capacity(self.kernels_data.len());
            for k in &self.kernels_data {
                kernels_gbuffer_commands.push(
                    vxresult!(k.lock()).frames_data[frame_number]
                        .gbuff
                        .get_data(),
                );
            }
            let cmd = &mut frame_data.gbuffer;
            cmd.begin();
            g_buffer_filler.begin_primary(cmd);
            cmd.exe_cmds_with_data(&kernels_gbuffer_commands);
            cmd.end_render_pass();
            cmd.end();
        }
        geng.submit(&sem, &frame_data.gbuffer, &frame_data.gbuffer_semaphore);
        // resolver
        resolver.begin_primary(&mut frame_data.resolver);
        resolver.begin_secondary(&mut frame_data.resolver_secondary, frame_number);
        frame_data.resolver.exe_cmd(&frame_data.resolver_secondary);
        frame_data.resolver.end_render_pass();
        frame_data.resolver.end();
        geng.submit(
            &frame_data.gbuffer_semaphore,
            &frame_data.resolver,
            &frame_data.resolver_semaphore,
        );
        let mut last_sem = frame_data.resolver_semaphore.clone();
        for (_, sml) in &self.shadow_maker_lights {
            let mut sml = vxresult!(sml.write());
            let sml = vxunwrap!(sml.to_mut_shadow_maker());
            last_sem = sml.submit_shadow_mapper(&last_sem, geng, shadower, frame_number);
        }
        // deferred
        frame_data
            .deferred_secondary
            .begin_secondary(geng.get_current_framebuffer());
        deferred.render(&mut frame_data.deferred_secondary, frame_number);
        frame_data
            .deferred_secondary
            .bind_deferred_scene_descriptor(
                &*self.descriptor_set,
                &*vxresult!(self.uniform_buffer.get_buffer(frame_number).read()),
            );
        frame_data.deferred_secondary.render_deferred();
        frame_data.deferred_secondary.end();
        frame_data.deferred.begin();
        geng.get_current_framebuffer()
            .begin(&mut frame_data.deferred);
        frame_data.deferred.exe_cmd(&frame_data.deferred_secondary);
        frame_data.deferred.end_render_pass();
        frame_data.deferred.end();
        geng.submit(
            &last_sem,
            &frame_data.deferred,
            &frame_data.deferred_semaphore,
        );
        return frame_data.deferred_semaphore.clone();
    }
}

impl DefaultScene for Base {
    fn default(engine: &Engine) -> Self {
        let gapi_engine = vxresult!(engine.get_gapi_engine().read());
        let uniform_buffer = vxresult!(gapi_engine.get_buffer_manager().write())
            .create_dynamic_buffer(size_of::<Uniform>() as isize);
        let mut descriptor_manager = vxresult!(gapi_engine.get_descriptor_manager().write());
        let descriptor_set = descriptor_manager.create_buffer_only_set(&uniform_buffer);
        let descriptor_set = Arc::new(descriptor_set);
        let frames_count = gapi_engine.get_frames_count();
        let kernels_count = num_cpus::get();
        let mut kernels_data = Vec::with_capacity(kernels_count);
        for _ in 0..kernels_count {
            kernels_data.push(Arc::new(Mutex::new(BaseKernelData {
                frames_data: Vec::with_capacity(frames_count),
            })));
        }
        Self {
            obj_base: ObjectBase::new(),
            uniform: Uniform::new(),
            uniform_buffer,
            descriptor_set,
            cameras: BTreeMap::new(),
            active_camera: None,
            models: BTreeMap::new(),
            all_models: BTreeMap::new(),
            lights: BTreeMap::new(),
            shadow_maker_lights: BTreeMap::new(),
            kernels_data,
            frames_data: Vec::new(),
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

    fn add_light(&mut self, light: Arc<RwLock<Light>>) {
        self.base.add_light(light);
    }

    fn get_active_camera(&self) -> &Option<Weak<RwLock<Camera>>> {
        return self.base.get_active_camera();
    }

    fn update(&mut self, frame_number: usize) {
        self.base.update(frame_number);
    }

    fn render_gbuffer_shadow_maps(
        &self,
        geng: &GraphicApiEngine,
        cmd_pool: &Arc<CmdPool>,
        g_buffer_filler: &GBufferFiller,
        shadower: &Shadower,
        kernel_index: usize,
    ) {
        self.base.render_gbuffer_shadow_maps(
            geng,
            cmd_pool,
            g_buffer_filler,
            shadower,
            kernel_index,
        );
    }

    fn update_shadow_makers(&self) {
        self.base.update_shadow_makers();
    }

    fn render_shadow_maps(&self, kernel_index: usize, frame_number: usize) {
        self.base.render_shadow_maps(kernel_index, frame_number);
    }

    fn get_models(&self) -> &BTreeMap<Id, Arc<RwLock<Model>>> {
        return self.base.get_models();
    }

    fn get_all_models(&self) -> &BTreeMap<Id, Weak<RwLock<Model>>> {
        return self.base.get_all_models();
    }

    fn clean(&mut self) {
        self.base.clean();
    }

    fn submit(
        &mut self,
        geng: &GraphicApiEngine,
        sem: &Arc<Semaphore>,
        cmd_pool: &Arc<CmdPool>,
        g_buffer_filler: &GBufferFiller,
        resolver: &Resolver,
        shadower: &Shadower,
        deferred: &Deferred,
    ) -> Arc<Semaphore> {
        return self.base.submit(
            geng,
            sem,
            cmd_pool,
            g_buffer_filler,
            resolver,
            shadower,
            deferred,
        );
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

    fn add_light(&mut self, light: Arc<RwLock<Light>>) {
        self.base.add_light(light);
    }

    fn get_active_camera(&self) -> &Option<Weak<RwLock<Camera>>> {
        return self.base.get_active_camera();
    }

    fn update(&mut self, frame_number: usize) {
        self.base.update(frame_number);
    }

    fn render_gbuffer_shadow_maps(
        &self,
        geng: &GraphicApiEngine,
        cmd_pool: &Arc<CmdPool>,
        g_buffer_filler: &GBufferFiller,
        shadower: &Shadower,
        kernel_index: usize,
    ) {
        self.base.render_gbuffer_shadow_maps(
            geng,
            cmd_pool,
            g_buffer_filler,
            shadower,
            kernel_index,
        );
    }

    fn update_shadow_makers(&self) {
        self.base.update_shadow_makers();
    }

    fn render_shadow_maps(&self, kernel_index: usize, frame_number: usize) {
        self.base.render_shadow_maps(kernel_index, frame_number);
    }

    fn get_models(&self) -> &BTreeMap<Id, Arc<RwLock<Model>>> {
        return self.base.get_models();
    }

    fn get_all_models(&self) -> &BTreeMap<Id, Weak<RwLock<Model>>> {
        return self.base.get_all_models();
    }

    fn clean(&mut self) {
        self.base.clean();
    }

    fn submit(
        &mut self,
        geng: &GraphicApiEngine,
        sem: &Arc<Semaphore>,
        cmd_pool: &Arc<CmdPool>,
        g_buffer_filler: &GBufferFiller,
        resolver: &Resolver,
        shadower: &Shadower,
        deferred: &Deferred,
    ) -> Arc<Semaphore> {
        return self.base.submit(
            geng,
            sem,
            cmd_pool,
            g_buffer_filler,
            resolver,
            shadower,
            deferred,
        );
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
