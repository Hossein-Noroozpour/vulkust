pub mod manager;

use std::cell::RefCell;
use std::sync::Arc;
use super::super::audio::Audio;
use super::super::audio::manager::Manager as AudioManager;
use super::super::system::file::File;
use super::buffer::{Buffer, Usage as BufferUsage};
use super::buffer::uniform::Uniform;
use super::camera::Camera;
use super::camera::manager::Manager as CameraManager;
use super::command::pool::Pool as CmdPool;
use super::light::Light;
use super::light::manager::Manager as LightManager;
use super::model::Model;
use super::model::manager::Manager as ModelManager;
use super::shader::manager::Manager as ShaderManager;
use super::texture::manager::Manager as TextureManager;

pub trait Scene {
    fn get_current_camera(&self) -> &Arc<RefCell<Camera<f32>>>;
}

pub struct BasicScene {
    pub meshes_vertices_buffer: Buffer,
    pub meshes_indices_buffer: Buffer,
    pub uniform_buffer: Uniform,
    pub current_camera: usize,
    pub cameras: Vec<Arc<RefCell<Camera<f32>>>>,
    pub audios: Vec<Arc<RefCell<Audio>>>,
    pub lights: Vec<Arc<RefCell<Light>>>,
    pub models: Vec<Arc<RefCell<Model>>>,
}

impl BasicScene {
    pub fn new(
        file: &mut File,
        camera_manager: &mut CameraManager,
        audio_manager: &mut AudioManager,
        light_manager: &mut LightManager,
        model_manager: &mut ModelManager,
        shader_manager: &mut ShaderManager,
        texture_manager: &mut TextureManager,
        screen_ratio: f32,
        transfer_cmd_pool: Arc<CmdPool>) -> Self {
        let device = transfer_cmd_pool.logical_device.clone();
        let v_size = file.read_type::<u64>() * 1024;
        let i_size = file.read_type::<u64>() * 1024;
        let mut meshes_vertices_buffer =
            Buffer::new(transfer_cmd_pool.clone(), v_size, BufferUsage::Vertex);
        let mut meshes_indices_buffer =
            Buffer::new(transfer_cmd_pool.clone(), i_size, BufferUsage::Index);
        let uniform_buffer = Uniform::new(device, 1024);
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
        let mut cameras = Vec::new();
        for i in cameras_ids {
            cameras.push(camera_manager.get(i, file, screen_ratio));
        }
        let mut audios = Vec::new();
        for i in audios_ids {
            audios.push(audio_manager.get(i, file));
        }
        let mut lights = Vec::new();
        for i in lights_ids {
            lights.push(light_manager.get(i, file));
        }
        let mut models = Vec::new();
        for i in models_ids {
            models.push(model_manager.get(
                i, file,
                &mut meshes_vertices_buffer, &mut meshes_indices_buffer,
                texture_manager, shader_manager));
        }
        BasicScene {
            meshes_vertices_buffer: meshes_vertices_buffer,
            meshes_indices_buffer: meshes_indices_buffer,
            uniform_buffer: uniform_buffer,
            current_camera: 0,
            cameras: cameras,
            audios: audios,
            lights: lights,
            models: models,
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
}
