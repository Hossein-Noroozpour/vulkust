pub mod manager;

use std::cell::RefCell;
use std::default::Default;
use std::sync::Arc;
use super::super::audio::Audio;
use super::super::core::application::ApplicationTrait;
use super::super::math::matrix::Mat4x4;
use super::super::math::vector::Vec3;
use super::super::system::file::File;
use super::super::system::os::ApplicationTrait as OsApp;
use super::buffer::Manager as BufferManager;
use super::command::buffer::Buffer as CmdBuff;
use super::camera::Camera;
use super::descriptor::{Manager as DescriptorManager, Set as DescriptorSet};
use super::engine::RenderEngine;
use super::light::Light;
use super::material::{Material, White};
use super::model::Model;
use super::pipeline::Pipeline;
use super::shader;

#[derive(Default)]
pub struct UniformData {
    pub sun_dir: Vec3<f32>,
    pub eye_loc: Vec3<f32>,
    pub vp: Mat4x4<f32>,
}

pub trait Scene {
    fn get_current_camera(&self) -> &Arc<RefCell<Camera<f32>>>;
    fn update(&mut self, frame_index: usize);
    fn record(&mut self, cmd_buff: &mut CmdBuff, frame_index: usize);
}

pub struct BasicScene {
    pub uniform_data: UniformData,
    pub buffer_manager: Arc<RefCell<BufferManager>>,
    pub descriptor_manager: DescriptorManager,
    pub current_camera: usize,
    pub cameras: Vec<Arc<RefCell<Camera<f32>>>>,
    pub audios: Vec<Arc<RefCell<Audio>>>,
    pub lights: Vec<Arc<RefCell<Light>>>,
    pub models: Vec<Arc<RefCell<Model>>>,
    pub occ_material: Arc<RefCell<Material>>,
    pub occ_pipeline: Pipeline,
    pub occ_descriptor: Arc<DescriptorSet>,
}

impl BasicScene {
    pub fn new<CoreApp>(file: &mut File, engine: &mut RenderEngine<CoreApp>) -> Self
    where
        CoreApp: ApplicationTrait,
    {
        let device = engine.logical_device.as_ref().unwrap();
        let window_ratio = engine.os_app.get_window_ratio() as f32;
        let asset_manager = &mut engine.os_app.asset_manager;
        let vi_size = file.read_type::<u64>() * 1024;
        let u_size = file.read_type::<u64>() * 1024;
        let mut buffer_manager =
            BufferManager::new(device.clone(), vi_size as usize, u_size as usize, engine.framebuffers.len());
        let cameras_count = file.read_count() as usize;
        let mut cameras_ids = vec![0; cameras_count];
        for i in 0..cameras_count {
            cameras_ids[i] = file.read_id();
        }
        let audios_count = file.read_count() as usize;
        let mut audios_ids = vec![0; audios_count];
        for i in 0..audios_count {
            audios_ids[i] = file.read_id();
        }
        let lights_count = file.read_count() as usize;
        let mut lights_ids = vec![0; lights_count];
        for i in 0..lights_count {
            lights_ids[i] = file.read_id();
        }
        let models_count = file.read_count() as usize;
        let mut models_ids = vec![0; models_count];
        for i in 0..models_count {
            models_ids[i] = file.read_id();
        }
        let occ_material: Arc<RefCell<Material>> = Arc::new(RefCell::new(White::new(
            file, device.clone(), &mut asset_manager.shader_manager)));
        let mut cameras = Vec::new();
        for i in cameras_ids {
            cameras.push(asset_manager.get_camera(i, window_ratio));
        }
        let mut audios = Vec::new();
        for i in audios_ids {
            audios.push(asset_manager.get_audio(i));
        }
        let mut lights = Vec::new();
        for i in lights_ids {
            lights.push(asset_manager.get_light(i));
        }
        let mut models = Vec::new();
        for i in models_ids {
            models.push(asset_manager.get_model(i, &mut buffer_manager));
        }
        let _ = device;
        let occ_pipeline = Pipeline::new(&engine.os_app.render_engine, &occ_material);
        let des_pool = engine.descriptor_pool.as_ref().unwrap().clone();
        let mut descriptor_manager = DescriptorManager::new(des_pool, occ_pipeline.layout.clone(), &buffer_manager);
        let occ_descriptor = descriptor_manager.get(shader::WHITE_ID);
        BasicScene {
            uniform_data: UniformData::default(),
            buffer_manager: buffer_manager,
            descriptor_manager: descriptor_manager,
            current_camera: 0,
            cameras: cameras,
            audios: audios,
            lights: lights,
            models: models,
            occ_material: occ_material,
            occ_pipeline: occ_pipeline,
            occ_descriptor: occ_descriptor,
        }
    }
}

impl Scene for BasicScene {
    fn get_current_camera(&self) -> &Arc<RefCell<Camera<f32>>> {
        #[cfg(debug_assertions)]
        {
            if self.current_camera >= self.cameras.len() {
                logf!("Camera index out of range.");
            }
        }
        return &self.cameras[self.current_camera];
    }

    fn update(&mut self, frame_index: usize) {
        // TODO: step 1
        // TODO: step 2
        let camera = self.cameras[self.current_camera].borrow();
        self.uniform_data.vp = *camera.get_view_projection();
        for model in &mut self.models {
            model.borrow_mut().parent_update_uniform(&self.uniform_data, frame_index);
        }
        ////// Temporary @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@



        ////// Temporary @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
        // TODO: step 4
        // TODO: step 5
        // TODO: step 6
        // TODO: step 7
        // TODO: step 8
    }

    fn record(&mut self, cmd_buff: &mut CmdBuff, frame_index: usize) {
        self.buffer_manager.push_u(cmd_buff, frame_index);
        // cmd_buff.bind_descriptor_set(self.)
        // shader (descriptor, pipeline)
        // material the descriptor offseting
        // model mesh binding (vertex, index) and draw index
    }
}
