use std::cell::RefCell;
use std::collections::BTreeMap;
use std::sync::{Arc, Weak};
use std::io::{Seek, SeekFrom};
use super::super::super::system::file::File;
use super::super::super::audio::manager::Manager as AudioManager;
use super::super::camera::manager::Manager as CameraManager;
use super::super::command::pool::Pool as CmdPool;
use super::super::light::manager::Manager as LightManager;
use super::super::model::manager::Manager as ModelManager;
use super::super::shader::manager::Manager as ShaderManager;
use super::super::texture::manager::Manager as TextureManager;
use super::{BasicScene, Scene};

pub struct Manager {
    pub cached: BTreeMap<u64, Weak<RefCell<Scene>>>,
    pub offsets: Vec<u64>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            cached: BTreeMap::new(),
            offsets: Vec::new(),
        }
    }

    pub fn read_table(&mut self, file: &mut File) {
        let count = file.read_count();
        self.offsets.resize(count as usize, 0);
        for i in 0..count as usize {
            self.offsets[i] = file.read_offset();
        }
    }

    pub fn get(
        &mut self,
        id: u64,
        file: &mut File,
        camera_manager: &mut CameraManager,
        audio_manager: &mut AudioManager,
        light_manager: &mut LightManager,
        model_manager: &mut ModelManager,
        shader_manager: &mut ShaderManager,
        texture_manager: &mut TextureManager,
        screen_ratio: f32,
        transfer_cmd_pool: Arc<CmdPool>
    ) -> Arc<RefCell<Scene>> {
        match self.cached.get(&id) {
            Some(res) => match res.upgrade() {
                Some(res) => {
                    return res;
                }
                None => {}
            },
            None => {}
        }
        let offset = self.offsets[id as usize];
        #[cfg(scene_import_debug)]
        logi!("scene with id {} has offset {}", id, offset);
        match file.seek(SeekFrom::Start(offset)) {
            Ok(o) => if o < offset {
                logf!("Seeked offset does not match!");
            },
            _ => {
                logf!("Can not seek to the requested offset.");
            }
        }
        let scene = RefCell::new(BasicScene::new(
            file, camera_manager, audio_manager, light_manager, model_manager, shader_manager,
            texture_manager, screen_ratio, transfer_cmd_pool));
        let scene: Arc<RefCell<Scene>> = Arc::new(scene);
        self.cached.insert(id, Arc::downgrade(&scene));
        return scene;
    }
}
