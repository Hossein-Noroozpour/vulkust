use super::super::super::core::gx3d::Gx3DReader;
use super::super::super::core::object::Object as CoreObject;
use super::super::super::core::types::Id;
use super::super::camera::Camera;
use super::super::command::{Buffer as CmdBuffer, Pool as CmdPool};
use super::super::deferred::Deferred;
use super::super::engine::Engine;
use super::super::framebuffer::Framebuffer;
use super::super::g_buffer_filler::GBufferFiller;
use super::super::gapi::GraphicApiEngine;
use super::super::light::Light;
use super::super::model::{Base as ModelBase, Model};
use super::super::object::{Base as ObjectBase, Loadable as ObjectLoadable, Object};
use super::super::pipeline::{Pipeline, PipelineType};
use super::super::render_pass::RenderPass;
use super::super::shadower::Shadower;
use super::super::ssao::SSAO;
use super::super::sync::Semaphore;
use super::{DefaultScene, Scene};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, RwLock, Weak};

#[cfg_attr(debug_mode, derive(Debug))]
struct BaseKernelFramedata {
    cmd_buff: CmdBuffer,
}

#[cfg_attr(debug_mode, derive(Debug))]
struct BaseKernelData {
    frames_data: Vec<BaseKernelFramedata>,
}

#[cfg_attr(debug_mode, derive(Debug))]
struct BaseFramedata {
    cmd_buff: CmdBuffer,
    cmd_buff_semaphore: Arc<Semaphore>,
}

impl BaseFramedata {
    fn new(engine: &GraphicApiEngine, cmd_pool: &Arc<CmdPool>) -> Self {
        let cmd_buff = engine.create_primary_command_buffer(cmd_pool.clone());
        let cmd_buff_semaphore = Arc::new(engine.create_semaphore());
        Self {
            cmd_buff,
            cmd_buff_semaphore,
        }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub(super) struct Base {
    obj_base: ObjectBase,
    cameras: BTreeMap<Id, Arc<RwLock<dyn Camera>>>,
    active_camera: Option<Weak<RwLock<dyn Camera>>>,
    models: BTreeMap<Id, Arc<RwLock<dyn Model>>>,
    all_models: BTreeMap<Id, Weak<RwLock<dyn Model>>>,
    kernels_data: Vec<Arc<Mutex<BaseKernelData>>>,
    frames_data: Vec<BaseFramedata>,
    render_pass: Arc<RenderPass>,
    framebuffers: Vec<Arc<Framebuffer>>,
    unlit_pipeline: Arc<Pipeline>,
}

impl Base {
    pub fn new_with_gltf(engine: &Engine, scene: &gltf::Scene, data: &[u8]) -> Self {
        let camera_manager = engine.get_asset_manager().get_camera_manager();
        let obj_base = ObjectBase::new();
        let mut cameras = BTreeMap::new();
        let mut active_camera = None;
        let mut models = BTreeMap::new();
        let mut all_models = BTreeMap::new();
        for node in scene.nodes() {
            if node.camera().is_some() {
                let camera = vx_result!(camera_manager.write()).load_gltf(&node, engine);
                let id = vx_result!(camera.read()).get_id();
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
                let model: Arc<RwLock<dyn Model>> = Arc::new(RwLock::new(model));
                all_models.insert(id, Arc::downgrade(&model));
                models.insert(id, model);
            } // todo read lights
        }
        let gapi_engine = vx_result!(engine.get_gapi_engine().read());
        let frames_count = gapi_engine.get_frames_count();
        let kernels_count = num_cpus::get();
        let mut kernels_data = Vec::with_capacity(kernels_count);
        for _ in 0..kernels_count {
            kernels_data.push(Arc::new(Mutex::new(BaseKernelData {
                frames_data: Vec::with_capacity(frames_count),
            })));
        }
        let render_pass = gapi_engine.get_render_pass().clone();
        let framebuffers = gapi_engine.get_framebuffers().clone();
        let unlit_pipeline = vx_result!(gapi_engine.get_pipeline_manager().write()).create(
            render_pass.clone(),
            PipelineType::Unlit,
            engine.get_config(),
        );
        Self {
            obj_base,
            cameras,
            active_camera,
            models,
            all_models,
            kernels_data,
            frames_data: Vec::new(),
            render_pass,
            framebuffers,
            unlit_pipeline,
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
            vx_unimplemented!(); // todo
        }
        let asset_manager = eng.get_asset_manager();
        let camera_manager = asset_manager.get_camera_manager();
        let light_manager = asset_manager.get_light_manager();
        let model_manager = asset_manager.get_model_manager();
        let mut cameras = BTreeMap::new();
        let active_camera = {
            let mut mgr = vx_result!(camera_manager.write());
            for id in &cameras_ids {
                cameras.insert(*id, mgr.load_gx3d(eng, *id));
            }
            if cameras_ids.len() > 0 {
                Some(Arc::downgrade(&mgr.load_gx3d(eng, cameras_ids[0])))
            } else {
                None
            }
        };
        let mut models = BTreeMap::new();
        let mut all_models = BTreeMap::new();
        {
            let mut mgr = vx_result!(model_manager.write());
            for id in models_ids {
                let model = mgr.load_gx3d(eng, id);
                {
                    let model = vx_result!(model.read());
                    let child_models = model.bring_all_child_models();
                    for (id, model) in child_models {
                        all_models.insert(id, Arc::downgrade(&model));
                    }
                }
                all_models.insert(id, Arc::downgrade(&model));
                models.insert(id, model);
            }
        }
        let mut lights = BTreeMap::new();
        let mut shadow_maker_lights = BTreeMap::new();
        {
            let mut mgr = vx_result!(light_manager.write());
            for id in lights_ids {
                let light = mgr.load_gx3d(eng, id);
                let is_shadow_maker = vx_result!(light.read()).to_shadow_maker().is_some();
                if is_shadow_maker {
                    shadow_maker_lights.insert(id, light);
                } else {
                    lights.insert(id, light);
                }
            }
        }
        let gapi_engine = vx_result!(eng.get_gapi_engine().read());
        let frames_count = gapi_engine.get_frames_count();
        let kernels_count = num_cpus::get();
        let mut kernels_data = Vec::with_capacity(kernels_count);
        for _ in 0..kernels_count {
            kernels_data.push(Arc::new(Mutex::new(BaseKernelData {
                frames_data: Vec::with_capacity(frames_count),
            })));
        }
        let render_pass = gapi_engine.get_render_pass().clone();
        let framebuffers = gapi_engine.get_framebuffers().clone();
        let unlit_pipeline = vx_result!(gapi_engine.get_pipeline_manager().write()).create(
            render_pass.clone(),
            PipelineType::Unlit,
            eng.get_config(),
        );
        Self {
            obj_base: ObjectBase::new_with_id(my_id),
            cameras,
            active_camera,
            models,
            all_models,
            kernels_data,
            frames_data: Vec::new(),
            render_pass,
            framebuffers,
            unlit_pipeline,
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
        vx_unimplemented!(); //it must update corresponding manager
    }

    fn disable_rendering(&mut self) {
        self.obj_base.disable_rendering()
    }

    fn enable_rendering(&mut self) {
        self.obj_base.enable_rendering()
    }

    fn is_renderable(&self) -> bool {
        if self.obj_base.is_renderable() {
            if let Some(camera) = &self.active_camera {
                if let Some(camera) = camera.upgrade() {
                    return vx_result!(camera.read()).is_renderable();
                }
            }
        }
        return false;
    }
}

impl Scene for Base {
    fn add_camera(&mut self, camera: Arc<RwLock<dyn Camera>>) {
        let id = vx_result!(camera.read()).get_id();
        if self.active_camera.is_none() {
            self.active_camera = Some(Arc::downgrade(&camera));
        }
        self.cameras.insert(id, camera);
    }

    fn add_model(&mut self, model: Arc<RwLock<dyn Model>>) {
        let id = {
            let model = vx_result!(model.read());
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

    fn add_light(&mut self, _: Arc<RwLock<dyn Light>>) {
        vx_unexpected!();
    }

    fn get_active_camera(&self) -> &Option<Weak<RwLock<dyn Camera>>> {
        return &self.active_camera;
    }

    fn update(&mut self, _: usize) {}

    fn update_shadow_makers(&self) {}

    fn render_gbuffer_shadow_maps(
        &self,
        geng: &GraphicApiEngine,
        cmd_pool: &Arc<CmdPool>,
        _: &GBufferFiller,
        _: &Shadower,
        kernel_index: usize,
    ) {
        if !self.is_renderable() {
            return;
        }
        let frame_number = geng.get_frame_number();
        let kernels_count = self.kernels_data.len();
        let mut kernel_data = vx_result!(self.kernels_data[kernel_index].lock());
        if kernel_data.frames_data.len() < 1 {
            let frames_count = geng.get_frames_count();
            for _ in 0..frames_count {
                kernel_data.frames_data.push(BaseKernelFramedata {
                    cmd_buff: geng.create_secondary_command_buffer(cmd_pool.clone()),
                });
            }
        }
        let cmd = &mut kernel_data.frames_data[frame_number].cmd_buff;
        cmd.begin_secondary(&self.framebuffers[frame_number]);
        cmd.bind_pipeline(&self.unlit_pipeline);
        let mut task_index = 0;
        let camera = vx_unwrap!(&self.active_camera).upgrade();
        let camera = vx_unwrap!(camera);
        let camera = vx_result!(camera.read());
        for (_, model) in &self.all_models {
            task_index += 1;
            if task_index % kernels_count != kernel_index {
                continue;
            }
            let model = model.upgrade();
            if model.is_none() {
                continue;
            }
            let model = vx_unwrap!(model);
            let mut model = vx_result!(model.write());
            if !model.is_renderable() {
                continue;
            }
            model.update(self, &*camera, frame_number);
            model.render_unlit(cmd, &*camera, frame_number);
        }
        cmd.end();
    }

    fn render_shadow_maps(&self, _: &Shadower, _: usize, _: usize) {}

    fn get_models(&self) -> &BTreeMap<Id, Arc<RwLock<dyn Model>>> {
        return &self.models;
    }

    fn get_all_models(&self) -> &BTreeMap<Id, Weak<RwLock<dyn Model>>> {
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
        _: &GBufferFiller,
        _: &mut Shadower,
        _: &Deferred,
        _: Option<&SSAO>,
    ) -> Arc<Semaphore> {
        if !self.is_renderable() {
            return sem.clone();
        }
        let frame_number = geng.get_frame_number();
        let frames_count = geng.get_frames_count();
        let frames_data_len = self.frames_data.len();
        for _ in frames_data_len..frames_count {
            self.frames_data.push(BaseFramedata::new(geng, cmd_pool));
        }
        let frame_data = &mut self.frames_data[frame_number];
        {
            let cmd = &mut frame_data.cmd_buff;
            cmd.begin();
            self.framebuffers[frame_number].begin(cmd);
            for k in &self.kernels_data {
                cmd.exe_cmd(&vx_result!(k.lock()).frames_data[frame_number].cmd_buff);
            }
            cmd.end_render_pass();
            cmd.end();
        }
        geng.submit(&sem, &frame_data.cmd_buff, &frame_data.cmd_buff_semaphore);
        return frame_data.cmd_buff_semaphore.clone();
    }
}

impl DefaultScene for Base {
    fn default(engine: &Engine) -> Self {
        let gapi_engine = vx_result!(engine.get_gapi_engine().read());
        let frames_count = gapi_engine.get_frames_count();
        let kernels_count = num_cpus::get();
        let mut kernels_data = Vec::with_capacity(kernels_count);
        for _ in 0..kernels_count {
            kernels_data.push(Arc::new(Mutex::new(BaseKernelData {
                frames_data: Vec::with_capacity(frames_count),
            })));
        }
        let render_pass = gapi_engine.get_render_pass().clone();
        let framebuffers = gapi_engine.get_framebuffers().clone();
        let unlit_pipeline = vx_result!(gapi_engine.get_pipeline_manager().write()).create(
            render_pass.clone(),
            PipelineType::Unlit,
            engine.get_config(),
        );
        Self {
            obj_base: ObjectBase::new(),
            cameras: BTreeMap::new(),
            active_camera: None,
            models: BTreeMap::new(),
            all_models: BTreeMap::new(),
            kernels_data,
            frames_data: Vec::new(),
            render_pass,
            framebuffers,
            unlit_pipeline,
        }
    }
}
