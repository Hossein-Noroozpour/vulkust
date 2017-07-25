pub mod manager;

use std::mem::transmute;
use super::super::core::application::ApplicationTrait;
use super::super::system::os::OsApplication;
use super::super::system::file::File;
use super::camera::Camera;
use super::camera::perspective::Perspective;

pub trait Scene {
    fn get_current_camera(&self) -> &Camera<f32>;
    fn get_mut_current_camera(&mut self) -> &mut Camera<f32>;
}

pub struct BasicScene {
    current_camera: usize,
    cameras: Vec<*mut Camera<f32>>,
}

impl BasicScene {
    pub fn new<CoreApp>(file: &mut File, os_app: *mut OsApplication<CoreApp>) -> Self
    where CoreApp: ApplicationTrait {
        BasicScene {
            current_camera: 0,
            cameras: vec![Box::into_raw(Box::new(Perspective::new()))],
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
