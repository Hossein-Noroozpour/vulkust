pub mod manager;

use std::default::Default;
use std::sync::Arc;
use std::mem::transmute;
use super::super::audio::Audio;
use super::super::core::application::ApplicationTrait;
use super::super::math::matrix::Mat4x4;
use super::super::math::vector::Vec3;
use super::super::system::file::File;
use super::super::system::os::ApplicationTrait as OsApp;
use super::super::util::cell::DebugCell;
use super::command::buffer::Buffer as CmdBuff;
use super::camera::Camera;
use super::engine::RenderEngine;
use super::light::Light;
use super::material::{Material, White};
use super::model::Model;

#[derive(Default)]
pub struct UniformData {
    pub sun_dir: Vec3<f32>,
    pub eye_loc: Vec3<f32>,
    pub vp: Mat4x4<f32>,
}

pub trait Scene {
    fn get_current_camera(&self) -> &Arc<DebugCell<Camera<f32>>>;
    fn update(&mut self, frame_index: usize);
    fn record(&mut self, cmd_buff: &mut CmdBuff, frame_index: usize);
}

pub struct BasicScene {
    pub uniform_data: UniformData,
    pub current_camera: usize,
    pub cameras: Vec<Arc<DebugCell<Camera<f32>>>>,
    pub audios: Vec<Arc<DebugCell<Audio>>>,
    pub lights: Vec<Arc<DebugCell<Light>>>,
    pub models: Vec<Arc<DebugCell<Model>>>,
    pub occ_material: Arc<DebugCell<Material>>,
}

impl BasicScene {
    pub fn new<CoreApp>(
        file: &Arc<DebugCell<File>>, 
        engine: &mut RenderEngine<CoreApp>
    ) -> Self where CoreApp: ApplicationTrait {
        let engine_ptr: usize = unsafe { transmute(engine) };
        let engine1: &mut RenderEngine<CoreApp> = unsafe { transmute(engine_ptr) };
        let engine2: &mut RenderEngine<CoreApp> = unsafe { transmute(engine_ptr) };
        let window_ratio = engine1.os_app.get_window_ratio() as f32;
        let morph_meshes_size = file.borrow_mut().read_count() * 1024;
        let _ = morph_meshes_size;
        let u_size = file.borrow_mut().read_count() * 1024;
        let cameras_count = file.borrow_mut().read_count() as usize;
        let mut cameras_ids = vec![0; cameras_count];
        for i in 0..cameras_count {
            cameras_ids[i] = file.borrow_mut().read_id();
        }
        let audios_count = file.borrow_mut().read_count() as usize;
        let mut audios_ids = vec![0; audios_count];
        for i in 0..audios_count {
            audios_ids[i] = file.borrow_mut().read_id();
        }
        let lights_count = file.borrow_mut().read_count() as usize;
        let mut lights_ids = vec![0; lights_count];
        for i in 0..lights_count {
            lights_ids[i] = file.borrow_mut().read_id();
        }
        let models_count = file.borrow_mut().read_count() as usize;
        let mut models_ids = vec![0; models_count];
        for i in 0..models_count {
            models_ids[i] = file.borrow_mut().read_id();
        }
        let occ_material: Arc<DebugCell<Material>> = Arc::new(DebugCell::new(White::new(engine1)));
        let asset_manager = &mut engine1.os_app.asset_manager;
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
            models.push(asset_manager.get_model(i, engine2));
        }
        BasicScene {
            uniform_data: UniformData::default(),
            current_camera: 0,
            cameras: cameras,
            audios: audios,
            lights: lights,
            models: models,
            occ_material: occ_material,
        }
    }
}

impl Scene for BasicScene {
    fn get_current_camera(&self) -> &Arc<DebugCell<Camera<f32>>> {
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
        // cmd_buff.bind_descriptor_set(self.)
        // shader (descriptor, pipeline)
        // material the descriptor offseting
        // model mesh binding (vertex, index) and draw index
    }
}
