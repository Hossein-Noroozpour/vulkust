use std::sync::Arc;
use std::cell::RefCell;
use super::super::super::audio::manager::Manager as AudioManager;
use super::super::super::audio::Audio;
use super::super::super::render::buffer::Buffer;
use super::super::super::render::camera::manager::Manager as CameraManager;
use super::super::super::render::camera::Camera;
use super::super::super::render::device::logical::Logical as LogicalDevice;
use super::super::super::render::light::manager::Manager as LightManager;
use super::super::super::render::light::Light;
use super::super::super::render::model::manager::Manager as ModelManager;
use super::super::super::render::model::Model;
use super::super::super::render::scene::manager::Manager as SceneManager;
use super::super::super::render::scene::Scene;
use super::super::super::render::shader::manager::Manager as ShaderManager;
use super::super::super::render::shader::Shader;
use super::super::super::render::texture::manager::Manager as TextureManager;
use super::super::super::render::texture::Texture;
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

    pub fn get_shader(&mut self, id: u64, logical_device: Arc<LogicalDevice>) -> Arc<Shader> {
        self.shader_manager.get(id, &mut self.file, logical_device)
    }

    pub fn get_camera(&mut self, id: u64, ratio: f32) -> Arc<RefCell<Camera<f32>>> {
        self.camera_manager.get(id, &mut self.file, ratio)
    }

    pub fn get_audio(&mut self, id: u64) -> Arc<RefCell<Audio>> {
        self.audio_manager.get(id, &mut self.file)
    }

    pub fn get_light(&mut self, id: u64) -> Arc<RefCell<Light>> {
        self.light_manager.get(id, &mut self.file)
    }

    pub fn get_texture(&mut self, id: u64) -> Arc<Texture> {
        self.texture_manager.get(id, &mut self.file)
    }

    pub fn get_model(
        &mut self,
        id: u64,
        vertices_buffer: &mut Buffer,
        indices_buffer: &mut Buffer
    ) -> Arc<RefCell<Model>> {
        let shader_manager = &mut self.shader_manager;
        let texture_manager = &mut self.texture_manager;
        self.model_manager.get(
            id, &mut self.file, vertices_buffer, indices_buffer, texture_manager, shader_manager)
    }

    pub fn get_scene(&mut self, id: u64) -> Arc<RefCell<Scene>> {
        self.scene_manager.get(id, &mut self.file
            camera_manager,
            audio_manager,
            light_manager,
            model_manager,
            shader_manager,
            texture_manager)
    }
}
