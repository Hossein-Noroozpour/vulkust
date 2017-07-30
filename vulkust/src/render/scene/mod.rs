pub mod manager;

use std::cell::RefCell;
use std::mem::transmute;
use std::sync::Arc;
use super::super::audio::Audio;
use super::super::core::application::ApplicationTrait;
use super::super::system::os::OsApplication;
use super::super::system::file::File;
use super::buffer::{Buffer, Usage as BufferUsage};
use super::camera::Camera;
use super::camera::perspective::Perspective;
use super::light::Light;
use super::model::Model;

pub trait Scene {
    fn get_current_camera(&self) -> &Camera<f32>;
    fn get_mut_current_camera(&mut self) -> &mut Camera<f32>;
}

pub struct BasicScene {
    meshes_vertices_buffer: Buffer,
    meshes_indices_buffer: Buffer,
    current_camera: usize,
    cameras: Vec<Arc<RefCell<Camera<f32>>>>,
    audios: Vec<Arc<RefCell<Audio>>>,
    lights: Vec<Arc<RefCell<Light>>>,
    models: Vec<Arc<RefCell<Model>>>,
}

impl BasicScene {
    pub fn new<CoreApp>(file: &mut File, os_app: *mut OsApplication<CoreApp>) -> Self
    where CoreApp: ApplicationTrait {
        let cmd_pool = unsafe {
            (*(*os_app).render_engine).transfer_cmd_pool.as_ref().unwrap().clone()
        };
        let ref mut asset_manager = unsafe { &((*os_app).asset_manager) };
        let v_size = file.read_type::<u64>() * 1024;
        let i_size = file.read_type::<u64>() * 1024;
        let meshes_vertices_buffer = Buffer::new(cmd_pool.clone(), v_size, BufferUsage::Vertex);
        let meshes_indices_buffer = Buffer::new(cmd_pool.clone(), i_size, BufferUsage::Index);
        let cameras_count: u64 = file.read_type();
        let mut cameras = Vec::new();
        for _ in 0..cameras_count {
            let id: u64 = file.read_type();
            cameras.push(asset_manager.get_camera(id, os_app));
        }
        let speaker_count: u64 = file.read_type();
        let mut speakers = Vec::new();
        for _ in 0..speakers_count {
            let id: u64 = file.read_type();
            s.push(asset_manager.get_camera(id, os_app));
        }
        let cameras_count: u64 = file.read_type();
        let mut cameras = Vec::new();
        for _ in 0..cameras_count {
            let id: u64 = file.read_type();
            cameras.push(asset_manager.get_camera(id, os_app));
        }
        let cameras_count: u64 = file.read_type();
        let mut cameras = Vec::new();
        for _ in 0..cameras_count {
            let id: u64 = file.read_type();
            cameras.push(asset_manager.get_camera(id, os_app));
        }
        BasicScene {
            meshes_vertices_buffer: meshes_vertices_buffer,
            meshes_indices_buffer: meshes_indices_buffer,
            current_camera: 0,
            cameras: cameras,
        }
    }
}

impl Scene for BasicScene {
    fn get_mut_current_camera(&mut self) -> &mut Camera<f32> {
        #[cfg(debug_assertions)]
        {
            if self.current_camera >= self.cameras.len() {
                logf!("Camera index out of range.");
            }
        }
        unsafe { transmute(self.cameras[self.current_camera]) }
    }

    fn get_current_camera(&self) -> &Camera<f32> {
        #[cfg(debug_assertions)]
        {
            if self.current_camera >= self.cameras.len() {
                logf!("Camera index out of range.");
            }
        }
        unsafe { transmute(self.cameras[self.current_camera]) }
    }
}
