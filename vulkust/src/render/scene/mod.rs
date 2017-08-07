pub mod manager;

use std::cell::RefCell;
use std::mem::transmute;
use std::sync::Arc;
use super::super::audio::Audio;
use super::super::core::application::ApplicationTrait;
use super::super::system::os::OsApplication;
use super::super::system::file::File;
use super::buffer::{Buffer, Usage as BufferUsage};
use super::buffer::uniform::Uniform;
use super::camera::Camera;
use super::camera::perspective::Perspective;
use super::light::Light;
use super::model::Model;

pub trait Scene {
    fn get_current_camera(&self) -> &Arc<RefCell<Camera<f32>>>;
}

pub struct BasicScene {
    meshes_vertices_buffer: Buffer,
    meshes_indices_buffer: Buffer,
    uniform_buffer: Uniform,
    current_camera: usize,
    cameras: Vec<Arc<RefCell<Camera<f32>>>>,
    audios: Vec<Arc<RefCell<Audio>>>,
    lights: Vec<Arc<RefCell<Light>>>,
    models: Vec<Arc<RefCell<Model>>>,
}

impl BasicScene {
    pub fn new<CoreApp>(file: &mut File, os_app: *mut OsApplication<CoreApp>) -> Self
    where
        CoreApp: ApplicationTrait,
    {
        let cmd_pool = unsafe {
            (*(*os_app).render_engine)
                .transfer_cmd_pool
                .as_ref()
                .unwrap()
                .clone()
        };
        let device = cmd_pool.logical_device.clone();
        let mut asset_manager = unsafe { &mut ((*os_app).asset_manager) };
        let v_size = file.read_type::<u64>() * 1024;
        let i_size = file.read_type::<u64>() * 1024;
        let meshes_vertices_buffer = Buffer::new(cmd_pool.clone(), v_size, BufferUsage::Vertex);
        let meshes_indices_buffer = Buffer::new(cmd_pool.clone(), i_size, BufferUsage::Index);
        let uniform_buffer = Uniform::new(device, 1024);
        let cameras_count: u64 = file.read_type();
        let mut cameras = Vec::new();
        for _ in 0..cameras_count {
            let id: u64 = file.read_type();
            cameras.push(asset_manager.get_camera(id, os_app));
        }
        let audios_count: u64 = file.read_type();
        let mut audios = Vec::new();
        for _ in 0..audios_count {
            let id: u64 = file.read_type();
            audios.push(asset_manager.get_audio(id, os_app));
        }
        let lights_count: u64 = file.read_type();
        let mut lights = Vec::new();
        for _ in 0..lights_count {
            let id: u64 = file.read_type();
            lights.push(asset_manager.get_light(id, os_app));
        }
        let models_count: u64 = file.read_type();
        let mut models = Vec::new();
        for _ in 0..models_count {
            let id: u64 = file.read_type();
            models.push(asset_manager.get_model(id, os_app));
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
