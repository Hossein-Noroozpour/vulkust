use std::sync::Arc;
use super::super::super::audio::manager::Manager as AudioManager;
use super::super::super::audio::Audio;
use super::super::super::core::application::ApplicationTrait;
use super::super::super::render::camera::manager::Manager as CameraManager;
use super::super::super::render::camera::Camera;
use super::super::super::render::device::logical::Logical as LogicalDevice;
use super::super::super::render::engine::RenderEngine;
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
use super::super::super::util::cell::DebugCell;

pub struct Manager {
    pub file: Arc<DebugCell<File>>,
    pub shader_manager: Arc<DebugCell<ShaderManager>>,
    pub camera_manager: Arc<DebugCell<CameraManager>>,
    pub audio_manager: Arc<DebugCell<AudioManager>>,
    pub light_manager: Arc<DebugCell<LightManager>>,
    pub texture_manager: Arc<DebugCell<TextureManager>>,
    pub model_manager: Arc<DebugCell<ModelManager>>,
    pub scene_manager: Arc<DebugCell<SceneManager>>,
}

impl Manager {
    pub fn new(file: File) -> Self {
        let file = Arc::new(DebugCell::new(file));
        Manager {
            file: file.clone(),
            shader_manager: Arc::new(DebugCell::new(ShaderManager::new(file.clone()))),
            camera_manager: Arc::new(DebugCell::new(CameraManager::new(file.clone()))),
            audio_manager: Arc::new(DebugCell::new(AudioManager::new(file.clone()))),
            light_manager: Arc::new(DebugCell::new(LightManager::new(file.clone()))),
            texture_manager: Arc::new(DebugCell::new(TextureManager::new(file.clone()))),
            model_manager: Arc::new(DebugCell::new(ModelManager::new(file.clone()))),
            scene_manager: Arc::new(DebugCell::new(SceneManager::new(file))),
        }
    }

    pub fn initialize(&mut self) {
        self.shader_manager.borrow_mut().read_table();
        self.camera_manager.borrow_mut().read_table();
        self.audio_manager.borrow_mut().read_table();
        self.light_manager.borrow_mut().read_table();
        self.texture_manager.borrow_mut().read_table();
        self.model_manager.borrow_mut().read_table();
        self.scene_manager.borrow_mut().read_table();
    }
    
    pub fn get_shader(&self, id: u64, logical_device: Arc<LogicalDevice>) -> Arc<DebugCell<Shader>> {
        self.shader_manager.borrow_mut().get(id, logical_device)
    }

    pub fn get_camera(&self, id: u64, ratio: f32) -> Arc<DebugCell<Camera<f32>>> {
        self.camera_manager.borrow_mut().get(id, ratio)
    }

    pub fn get_audio(&self, id: u64) -> Arc<DebugCell<Audio>> {
        self.audio_manager.borrow_mut().get(id)
    }

    pub fn get_light(&self, id: u64) -> Arc<DebugCell<Light>> {
        self.light_manager.borrow_mut().get(id)
    }

    pub fn get_texture(&mut self, id: u64) -> Arc<DebugCell<Texture>> {
        self.texture_manager.borrow_mut().get(id)
    }

    pub fn get_model<CoreApp>(
        &mut self,
        id: u64,
        engine: &mut RenderEngine<CoreApp>,
    ) -> Arc<DebugCell<Model>> 
    where
        CoreApp: ApplicationTrait,
    {
        self.model_manager.borrow_mut().get(id, engine)
    }

    pub fn get_scene<CoreApp>(
        &mut self,
        id: u64,
        engine: &mut RenderEngine<CoreApp>,
    ) -> Arc<DebugCell<Scene>>
    where
        CoreApp: ApplicationTrait,
    {
        self.scene_manager.borrow_mut().get(id, engine)
    }
}
