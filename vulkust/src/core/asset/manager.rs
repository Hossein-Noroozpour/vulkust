use std::sync::Arc;
use std::cell::RefCell;
use super::super::super::audio::manager::Manager as AudioManager;
use super::super::super::audio::Audio;
use super::super::super::render::camera::manager::Manager as CameraManager;
use super::super::super::render::camera::Camera;
use super::super::super::render::light::manager::Manager as LightManager;
use super::super::super::render::light::Light;
use super::super::super::render::model::manager::Manager as ModelManager;
use super::super::super::render::model::Model;
use super::super::super::render::scene::manager::Manager as SceneManager;
use super::super::super::render::scene::Scene;
use super::super::super::render::shader::manager::Manager as ShaderManager;
use super::super::super::render::shader::ShaderTrait;
use super::super::super::render::texture::manager::Manager as TextureManager;
use super::super::super::render::texture::TextureTrait;
use super::super::super::system::file::File;
use super::super::super::system::os::OsApplication;
use super::super::application::ApplicationTrait;

pub struct Manager {
    pub file: File,
    pub shader_manager: ShaderManager,
    pub camera_manager: CameraManager,
    pub audio_manager: AudioManager,
    pub light_manager: LightManager,
    pub texture_manager: TextureManager,
    pub model_manager: ModelManager,
    pub scene_manager: SceneManager,
}

impl Manager {
    pub fn new(file: File) -> Self {
        Manager {
            file: file,
            shader_manager: ShaderManager::new(),
            camera_manager: CameraManager::new(),
            audio_manager: AudioManager::new(),
            light_manager: LightManager::new(),
            texture_manager: TextureManager::new(),
            model_manager: ModelManager::new(),
            scene_manager: SceneManager::new(),
        }
    }

    pub fn initialize(&mut self) {
        self.shader_manager.read_table(&mut self.file);
        self.camera_manager.read_table(&mut self.file);
        self.audio_manager.read_table(&mut self.file);
        self.light_manager.read_table(&mut self.file);
        self.texture_manager.read_table(&mut self.file);
        self.model_manager.read_table(&mut self.file);
        self.scene_manager.read_table(&mut self.file);
    }

    pub fn get_shader<CoreApp>(
        &mut self,
        id: u64,
        os_app: *mut OsApplication<CoreApp>,
    ) -> Arc<ShaderTrait>
    where
        CoreApp: ApplicationTrait,
    {
        self.shader_manager.get(id, &mut self.file, os_app)
    }

    pub fn get_camera<CoreApp>(
        &mut self,
        id: u64,
        os_app: *mut OsApplication<CoreApp>,
    ) -> Arc<RefCell<Camera<f32>>>
    where
        CoreApp: ApplicationTrait,
    {
        self.camera_manager.get(id, &mut self.file, os_app)
    }

    pub fn get_audio<CoreApp>(
        &mut self,
        id: u64,
        os_app: *mut OsApplication<CoreApp>,
    ) -> Arc<Audio>
    where
        CoreApp: ApplicationTrait,
    {
        self.audio_manager.get(id, &mut self.file, os_app)
    }

    pub fn get_light<CoreApp>(
        &mut self,
        id: u64,
        os_app: *mut OsApplication<CoreApp>,
    ) -> Arc<Light>
    where
        CoreApp: ApplicationTrait,
    {
        self.light_manager.get(id, &mut self.file, os_app)
    }

    pub fn get_texture<CoreApp>(
        &mut self,
        id: u64,
        os_app: *mut OsApplication<CoreApp>,
    ) -> Arc<TextureTrait>
    where
        CoreApp: ApplicationTrait,
    {
        self.texture_manager.get(id, &mut self.file, os_app)
    }

    pub fn get_model<CoreApp>(
        &mut self,
        id: u64,
        os_app: *mut OsApplication<CoreApp>,
    ) -> Arc<Model>
    where
        CoreApp: ApplicationTrait,
    {
        self.model_manager.get(id, &mut self.file, os_app)
    }

    pub fn get_scene<CoreApp>(
        &mut self,
        id: u64,
        os_app: *mut OsApplication<CoreApp>,
    ) -> Arc<RefCell<Scene>>
    where
        CoreApp: ApplicationTrait,
    {
        self.scene_manager.get(id, &mut self.file, os_app)
    }
}
