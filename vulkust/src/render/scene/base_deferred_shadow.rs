use super::super::super::core::algorithms::merge_all_sorted;
use super::super::super::core::constants::{MAX_DIRECTIONAL_LIGHTS_COUNT, MAX_POINT_LIGHTS_COUNT};
use super::super::super::core::gx3d::Gx3DReader;
use super::super::super::core::object::Object as CoreObject;
use super::super::super::core::types::{Id, Real};
use super::super::buffer::Dynamic as DynamicBuffer;
use super::super::camera::{Camera, Uniform as CameraUniform};
use super::super::command::{Buffer as CmdBuffer, Pool as CmdPool};
use super::super::deferred::Deferred;
use super::super::descriptor::Set as DescriptorSet;
use super::super::engine::Engine;
use super::super::framebuffer::Framebuffer;
use super::super::g_buffer_filler::GBufferFiller;
use super::super::gapi::GraphicApiEngine;
use super::super::light::{DirectionalUniform, Light, PointUniform};
use super::super::model::{Base as ModelBase, Model};
use super::super::object::{Base as ObjectBase, Loadable as ObjectLoadable, Object};
use super::super::pipeline::{Pipeline, PipelineType};
use super::super::render_pass::RenderPass;
use super::super::shadower::Shadower;
use super::super::skybox::Skybox;
use super::super::ssao::SSAO;
use super::super::sync::Semaphore;
use super::{DefaultScene, Scene};
use std::collections::BTreeMap;
use std::mem::size_of;
use std::sync::{Arc, Mutex, RwLock, Weak};

#[repr(C)]
#[cfg_attr(debug_mode, derive(Debug))]
struct Uniform {
    camera: CameraUniform,
    directional_lights: [DirectionalUniform; MAX_DIRECTIONAL_LIGHTS_COUNT],
    point_lights: [PointUniform; MAX_POINT_LIGHTS_COUNT],
    lights_count: cgmath::Vector4<u32>, // directional, point, rezerved
    ssao_config: cgmath::Vector4<Real>, // samples-count, radius, z-tolerance, rezerved
}

impl Uniform {
    pub fn new() -> Self {
        let camera = CameraUniform::new();
        Self {
            camera,
            directional_lights: [DirectionalUniform::new(); MAX_DIRECTIONAL_LIGHTS_COUNT],
            point_lights: [PointUniform::new(); MAX_POINT_LIGHTS_COUNT],
            lights_count: cgmath::Vector4::new(0, 0, 0, 0),
            ssao_config: cgmath::Vector4::new(64.1, 0.2, 0.6, 0.0),
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
    distance_transparent_models: Vec<(Real, Weak<RwLock<dyn Model>>)>,
}

#[cfg_attr(debug_mode, derive(Debug))]
struct BaseFramedata {
    gbuffer: CmdBuffer,
    gbuffer_semaphore: Arc<Semaphore>,
    gbuffer_semaphore_ssao: Arc<Semaphore>,
    deferred: CmdBuffer,
    deferred_secondary: CmdBuffer,
    deferred_semaphore: Arc<Semaphore>,
    ssao: CmdBuffer,
    ssao_secondary: CmdBuffer,
    ssao_semaphore: Arc<Semaphore>,
    preparation_cmd: CmdBuffer,
    preparation_semaphore: Arc<Semaphore>,
}

impl BaseFramedata {
    fn new(engine: &GraphicApiEngine, cmd_pool: &Arc<CmdPool>) -> Self {
        let gbuffer = engine.create_primary_command_buffer(cmd_pool.clone());
        let gbuffer_semaphore = Arc::new(engine.create_semaphore());
        let gbuffer_semaphore_ssao = Arc::new(engine.create_semaphore());
        let deferred = engine.create_primary_command_buffer(cmd_pool.clone());
        let deferred_secondary = engine.create_secondary_command_buffer(cmd_pool.clone());
        let deferred_semaphore = Arc::new(engine.create_semaphore());
        let ssao = engine.create_primary_command_buffer(cmd_pool.clone());
        let ssao_secondary = engine.create_secondary_command_buffer(cmd_pool.clone());
        let ssao_semaphore = Arc::new(engine.create_semaphore());
        let preparation_cmd = engine.create_primary_command_buffer(cmd_pool.clone());
        let preparation_semaphore = Arc::new(engine.create_semaphore());
        Self {
            gbuffer,
            gbuffer_semaphore,
            gbuffer_semaphore_ssao,
            deferred,
            deferred_secondary,
            deferred_semaphore,
            ssao,
            ssao_secondary,
            ssao_semaphore,
            preparation_cmd,
            preparation_semaphore,
        }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub(super) struct Base {
    obj_base: ObjectBase,
    uniform: Uniform,
    uniform_buffer: DynamicBuffer,
    cameras: BTreeMap<Id, Arc<RwLock<dyn Camera>>>,
    active_camera: Option<Weak<RwLock<dyn Camera>>>,
    shadow_maker_lights: BTreeMap<Id, Arc<RwLock<dyn Light>>>,
    lights: BTreeMap<Id, Arc<RwLock<dyn Light>>>,
    models: BTreeMap<Id, Arc<RwLock<dyn Model>>>,
    all_models: BTreeMap<Id, Weak<RwLock<dyn Model>>>,
    descriptor_set: Arc<DescriptorSet>,
    kernels_data: Vec<Arc<Mutex<BaseKernelData>>>,
    distance_transparent_models: Vec<(Real, Weak<RwLock<dyn Model>>)>,
    frames_data: Vec<BaseFramedata>,
    skybox: Option<Arc<RwLock<dyn Skybox>>>,
    render_pass: Arc<RenderPass>,
    framebuffers: Vec<Arc<Framebuffer>>,
    unlit_pipeline: Arc<Pipeline>,
    // constraints: BTreeMap<Id, Arc<RwLock<Constraint>>>, // todo
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
                let model: Arc<RwLock<dyn Model>> = Arc::new(RwLock::new(model));
                all_models.insert(id, Arc::downgrade(&model));
                models.insert(id, model);
            } // todo read lights
        }
        let gapi_engine = vxresult!(engine.get_gapi_engine().read());
        let uniform_buffer = vxresult!(gapi_engine.get_buffer_manager().write())
            .create_dynamic_buffer(size_of::<Uniform>() as isize);
        let render_pass = gapi_engine.get_render_pass().clone();
        let framebuffers = gapi_engine.get_framebuffers().clone();
        let unlit_pipeline = vxresult!(gapi_engine.get_pipeline_manager().write()).create(
            render_pass.clone(),
            PipelineType::Unlit,
            engine.get_config(),
        );
        let mut descriptor_manager = vxresult!(gapi_engine.get_descriptor_manager().write());
        let descriptor_set = descriptor_manager.create_buffer_only_set(&uniform_buffer);
        let frames_count = gapi_engine.get_frames_count();
        let kernels_count = num_cpus::get();
        let mut kernels_data = Vec::with_capacity(kernels_count);
        for _ in 0..kernels_count {
            kernels_data.push(Arc::new(Mutex::new(BaseKernelData {
                frames_data: Vec::with_capacity(frames_count),
                distance_transparent_models: Vec::new(),
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
            distance_transparent_models: Vec::new(),
            frames_data: Vec::new(),
            skybox: None,
            unlit_pipeline,
            framebuffers,
            render_pass,
        }
    }

    pub fn new_with_gx3d(eng: &Engine, reader: &mut Gx3DReader, my_id: Id) -> Self {
        let asset_manager = eng.get_asset_manager();
        let cameras_ids = reader.read_array::<Id>();
        let _audios_ids = reader.read_array::<Id>(); // todo
        let lights_ids = reader.read_array::<Id>();
        let models_ids = reader.read_array::<Id>();
        let skybox = if reader.read_bool() {
            let skybox_id: Id = reader.read();
            let skyboxmgr = asset_manager.get_skybox_manager();
            let mut skyboxmgr = vxresult!(skyboxmgr.write());
            Some(skyboxmgr.load_gx3d(eng, skybox_id))
        } else {
            None
        };
        let _constraits_ids = reader.read_array::<Id>(); // todo
        if reader.read_bool() {
            vxunimplemented!(); // todo
        }
        let camera_manager = asset_manager.get_camera_manager();
        let light_manager = asset_manager.get_light_manager();
        let model_manager = asset_manager.get_model_manager();
        let mut cameras = BTreeMap::new();
        let active_camera = {
            let mut mgr = vxresult!(camera_manager.write());
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
            let mut mgr = vxresult!(model_manager.write());
            for id in models_ids {
                let model = mgr.load_gx3d(eng, id);
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
        }
        let mut lights = BTreeMap::new();
        let mut shadow_maker_lights = BTreeMap::new();
        {
            let mut mgr = vxresult!(light_manager.write());
            for id in lights_ids {
                let light = mgr.load_gx3d(eng, id);
                let is_shadow_maker = vxresult!(light.read()).to_shadow_maker().is_some();
                if is_shadow_maker {
                    shadow_maker_lights.insert(id, light);
                } else {
                    lights.insert(id, light);
                }
            }
        }
        let uniform = Uniform::new();
        let gapi_engine = vxresult!(eng.get_gapi_engine().read());
        let uniform_buffer = vxresult!(gapi_engine.get_buffer_manager().write())
            .create_dynamic_buffer(size_of::<Uniform>() as isize);
        let render_pass = gapi_engine.get_render_pass().clone();
        let framebuffers = gapi_engine.get_framebuffers().clone();
        let unlit_pipeline = vxresult!(gapi_engine.get_pipeline_manager().write()).create(
            render_pass.clone(),
            PipelineType::Unlit,
            eng.get_config(),
        );
        let mut descriptor_manager = vxresult!(gapi_engine.get_descriptor_manager().write());
        let descriptor_set = descriptor_manager.create_buffer_only_set(&uniform_buffer);
        let frames_count = gapi_engine.get_frames_count();
        let kernels_count = num_cpus::get();
        let mut kernels_data = Vec::with_capacity(kernels_count);
        for _ in 0..kernels_count {
            kernels_data.push(Arc::new(Mutex::new(BaseKernelData {
                frames_data: Vec::with_capacity(frames_count),
                distance_transparent_models: Vec::new(),
            })));
        }
        Self {
            obj_base: ObjectBase::new_with_id(my_id),
            uniform,
            uniform_buffer,
            descriptor_set,
            cameras,
            active_camera,
            models,
            all_models,
            shadow_maker_lights,
            lights,
            kernels_data,
            distance_transparent_models: Vec::new(),
            frames_data: Vec::new(),
            skybox,
            render_pass,
            framebuffers,
            unlit_pipeline,
        }
    }

    fn gather_all_transparent_models_sorted(&self) -> Vec<Weak<RwLock<dyn Model>>> {
        let kernels_count = self.kernels_data.len();
        let mut kernels_data = Vec::with_capacity(kernels_count);
        for kd in &self.kernels_data {
            kernels_data.push(vxresult!(kd.lock()));
        }
        let mut ds: Vec<&[(Real, Weak<RwLock<dyn Model>>)]> = Vec::with_capacity(kernels_count);
        for kd in &kernels_data {
            ds.push(&kd.distance_transparent_models);
        }
        let sorted = merge_all_sorted(
            &ds,
            &mut |b: &(Real, Weak<RwLock<dyn Model>>), a: &(Real, Weak<RwLock<dyn Model>>)| {
                a.0.partial_cmp(&b.0).unwrap()
            },
        );
        let mut result = Vec::with_capacity(sorted.len());
        for e in sorted {
            result.push(e.1);
        }
        return result;
    }

    // fn render_transparent_models(&self) {
    //     let models = self.gather_all_transparent_models_sorted();
    //     for
    // }
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

    fn is_renderable(&self) -> bool {
        if self.obj_base.is_renderable() {
            if let Some(camera) = &self.active_camera {
                if let Some(camera) = camera.upgrade() {
                    return vxresult!(camera.read()).is_renderable();
                }
            }
        }
        return false;
    }
}

impl Scene for Base {
    fn add_camera(&mut self, camera: Arc<RwLock<dyn Camera>>) {
        let id = vxresult!(camera.read()).get_id();
        if self.active_camera.is_none() {
            self.active_camera = Some(Arc::downgrade(&camera));
        }
        self.cameras.insert(id, camera);
    }

    fn add_model(&mut self, model: Arc<RwLock<dyn Model>>) {
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

    fn add_light(&mut self, light: Arc<RwLock<dyn Light>>) {
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

    fn get_active_camera(&self) -> &Option<Weak<RwLock<dyn Camera>>> {
        return &self.active_camera;
    }

    fn update(&mut self, frame_number: usize) {
        if !self.is_renderable() {
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
            if !shm.is_renderable() {
                continue;
            }
            {
                if let Some(shm) = shm.to_mut_directional() {
                    if let Some(sun) = shm.to_mut_sun() {
                        sun.update_cascaded_shadow_map_cameras(
                            &csmws,
                            last_directional_light_index,
                        );
                    }
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
            let l = vxresult!(l.read());
            if !l.is_renderable() {
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
        self.uniform.lights_count.x = last_directional_light_index as u32;
        self.uniform.lights_count.y = last_point_light_index as u32;
        self.uniform_buffer.update(&self.uniform, frame_number);
        if let Some(skybox) = &self.skybox {
            vxresult!(skybox.write()).update(&*camera, frame_number);
        }
    }

    fn update_shadow_makers(&self) {
        for (_, shm) in &self.shadow_maker_lights {
            let mut shm = vxresult!(shm.write());
            if !shm.is_renderable() {
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
        if !self.is_renderable() {
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
        kernel_data.distance_transparent_models.clear();
        {
            let cmd = &mut kernel_data.frames_data[frame_number].gbuff;
            g_buffer_filler.begin_secondary(cmd);
            let buffer = self.uniform_buffer.get_buffer(frame_number);
            let buffer = vxresult!(buffer.read());
            cmd.bind_gbuff_scene_descriptor(&*self.descriptor_set, &*buffer);
        }
        let camera = vxunwrap!(&self.active_camera).upgrade();
        let camera = vxunwrap!(camera);
        let camera = vxresult!(camera.read());
        let mut task_index = 0;
        for (_, mw) in &self.all_models {
            task_index += 1;
            task_index %= kernels_count;
            if task_index != kernel_index {
                continue;
            }
            let model = mw.upgrade();
            let m = if let Some(model) = model {
                model
            } else {
                continue;
            };
            let mut model = vxresult!(m.write());
            if !model.is_renderable() {
                continue;
            }
            model.update(self, &*camera, frame_number);
            model.render_gbuffer(
                &mut kernel_data.frames_data[frame_number].gbuff,
                frame_number,
            );
            if model.has_shadow() {
                for (_, shm) in &self.shadow_maker_lights {
                    vxunwrap!(vxresult!(shm.read()).to_shadow_maker()).shadow(
                        &mut *model,
                        &m,
                        kernel_index,
                    );
                }
            }
            if model.has_transparent() {
                let d = model.get_distance_from_camera(&*camera);
                kernel_data
                    .distance_transparent_models
                    .push((d, mw.clone()));
            }
        }
        kernel_data.frames_data[frame_number].gbuff.end();
        kernel_data
            .distance_transparent_models
            .sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    }

    fn render_shadow_maps(&self, shadower: &Shadower, kernel_index: usize, frame_number: usize) {
        for (_, shm) in &self.shadow_maker_lights {
            vxunwrap!(vxresult!(shm.read()).to_shadow_maker()).render_shadow_mapper(
                shadower,
                kernel_index,
                frame_number,
            );
        }
    }

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
        g_buffer_filler: &GBufferFiller,
        shadower: &mut Shadower,
        deferred: &Deferred,
        ssao: Option<&SSAO>,
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
        // g-buffer
        {
            let cmd = &mut frame_data.gbuffer;
            cmd.begin();
            g_buffer_filler.begin_primary(cmd);
            for k in &self.kernels_data {
                cmd.exe_cmd(&vxresult!(k.lock()).frames_data[frame_number].gbuff);
            }
            cmd.end_render_pass();
            cmd.end();
        }
        if ssao.is_some() {
            geng.submit_multiple(
                &[&sem],
                &[&frame_data.gbuffer],
                &[
                    &frame_data.gbuffer_semaphore,
                    &frame_data.gbuffer_semaphore_ssao,
                ],
            );
        } else {
            geng.submit(&sem, &frame_data.gbuffer, &frame_data.gbuffer_semaphore);
        }
        // shadow
        shadower.clear_shadow_accumulator(&mut frame_data.preparation_cmd);
        geng.submit(
            &frame_data.gbuffer_semaphore,
            &frame_data.preparation_cmd,
            &frame_data.preparation_semaphore,
        );
        let mut last_sem = frame_data.preparation_semaphore.clone();
        for (_, sml) in &self.shadow_maker_lights {
            let mut sml = vxresult!(sml.write());
            let sml = vxunwrap!(sml.to_mut_shadow_maker());
            last_sem = sml.submit_shadow_mapper(&last_sem, geng, shadower, frame_number);
        }
        let uniform_buffer = vxresult!(self.uniform_buffer.get_buffer(frame_number).read());
        // SSAO
        if let Some(ssao) = &ssao {
            ssao.begin_secondary(&mut frame_data.ssao_secondary);
            frame_data
                .ssao_secondary
                .bind_ssao_scene_descriptor(&*self.descriptor_set, &*uniform_buffer);
            ssao.end_secondary(&mut frame_data.ssao_secondary, frame_number);
            ssao.record_primary(&mut frame_data.ssao, &frame_data.ssao_secondary);
            geng.submit(
                &frame_data.gbuffer_semaphore_ssao,
                &frame_data.ssao,
                &frame_data.ssao_semaphore,
            );
        }
        // deferred
        frame_data
            .deferred_secondary
            .begin_secondary(geng.get_current_framebuffer());
        // deferred -> skybox

        // deferred -> final
        deferred.render(&mut frame_data.deferred_secondary, frame_number);
        frame_data
            .deferred_secondary
            .bind_deferred_scene_descriptor(&*self.descriptor_set, &*uniform_buffer);
        frame_data.deferred_secondary.render_deferred();
        frame_data.deferred_secondary.end();
        frame_data.deferred.begin();
        geng.get_current_framebuffer()
            .begin(&mut frame_data.deferred);
        frame_data.deferred.exe_cmd(&frame_data.deferred_secondary);
        frame_data.deferred.end_render_pass();
        frame_data.deferred.end();
        if ssao.is_some() {
            geng.submit_multiple(
                &[&last_sem, &frame_data.ssao_semaphore],
                &[&frame_data.deferred],
                &[&frame_data.deferred_semaphore],
            );
        } else {
            geng.submit(
                &last_sem,
                &frame_data.deferred,
                &frame_data.deferred_semaphore,
            );
        }
        return frame_data.deferred_semaphore.clone();
    }
}

impl DefaultScene for Base {
    fn default(engine: &Engine) -> Self {
        let gapi_engine = vxresult!(engine.get_gapi_engine().read());
        let uniform_buffer = vxresult!(gapi_engine.get_buffer_manager().write())
            .create_dynamic_buffer(size_of::<Uniform>() as isize);
        let render_pass = gapi_engine.get_render_pass().clone();
        let framebuffers = gapi_engine.get_framebuffers().clone();
        let unlit_pipeline = vxresult!(gapi_engine.get_pipeline_manager().write()).create(
            render_pass.clone(),
            PipelineType::Unlit,
            engine.get_config(),
        );
        let mut descriptor_manager = vxresult!(gapi_engine.get_descriptor_manager().write());
        let descriptor_set = descriptor_manager.create_buffer_only_set(&uniform_buffer);
        let frames_count = gapi_engine.get_frames_count();
        let kernels_count = num_cpus::get();
        let mut kernels_data = Vec::with_capacity(kernels_count);
        for _ in 0..kernels_count {
            kernels_data.push(Arc::new(Mutex::new(BaseKernelData {
                frames_data: Vec::with_capacity(frames_count),
                distance_transparent_models: Vec::new(),
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
            distance_transparent_models: Vec::new(),
            frames_data: Vec::new(),
            skybox: None,
            render_pass,
            framebuffers,
            unlit_pipeline,
        }
    }
}
