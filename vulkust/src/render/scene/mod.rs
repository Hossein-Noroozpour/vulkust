pub mod manager;

use std::mem::transmute;
use super::camera::Camera;
use super::camera::perspective::Perspective;

pub struct Scene {
    current_camera: usize,
    cameras: Vec<*mut Camera<f32>>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            current_camera: 0,
            cameras: vec![Box::into_raw(Box::new(Perspective::new()))],
        }
    }

    pub fn get_mut_current_camera(&mut self) -> &mut Camera<f32> {
        #[cfg(debug_assertions)]
        {
            if self.current_camera >= self.cameras.len() {
                logf!("Camera index out of range.");
            }
        }
        unsafe { transmute(self.cameras[self.current_camera]) }
    }

    pub fn get_current_camera(&self) -> &Camera<f32> {
        #[cfg(debug_assertions)]
        {
            if self.current_camera >= self.cameras.len() {
                logf!("Camera index out of range.");
            }
        }
        unsafe { transmute(self.cameras[self.current_camera]) }
    }
}
