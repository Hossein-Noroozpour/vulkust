use super::super::render::camera::Manager as CameraManager;
use super::super::render::engine::Engine;
use super::super::render::font::Manager as FontManager;
use super::super::render::light::Manager as LightManager;
use super::super::render::mesh::Manager as MeshManager;
use super::super::render::model::Manager as ModelManager;
use super::super::render::scene::Manager as SceneManager;
use super::super::render::skybox::Manager as SkyboxManager;
use super::super::render::texture::Manager as TextureManager;
use super::config::Configurations;
use super::gx3d::{Gx3DReader, Table as Gx3dTable};
use super::object::NEXT_ID;
use super::types::Id;
use std::sync::atomic::Ordering;
use std::sync::{Arc, RwLock, Weak};

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Manager {
    scene_manager: Arc<RwLock<SceneManager>>,
    texture_manager: Arc<RwLock<TextureManager>>,
    skybox_manager: Arc<RwLock<SkyboxManager>>,
    light_manager: Arc<RwLock<LightManager>>,
    camera_manager: Arc<RwLock<CameraManager>>,
    mesh_manager: Arc<RwLock<MeshManager>>,
    font_manager: Arc<RwLock<FontManager>>,
    model_manager: Arc<RwLock<ModelManager>>,
}

impl Manager {
    pub(crate) fn new(config: &Configurations) -> Self {
        let scene_manager = Arc::new(RwLock::new(SceneManager::new()));
        let texture_manager = Arc::new(RwLock::new(TextureManager::new()));
        let light_manager = Arc::new(RwLock::new(LightManager::new()));
        let skybox_manager = Arc::new(RwLock::new(SkyboxManager::new()));
        let camera_manager = Arc::new(RwLock::new(CameraManager::new()));
        let mesh_manager = Arc::new(RwLock::new(MeshManager::new()));
        let font_manager = Arc::new(RwLock::new(FontManager::new()));
        let model_manager = Arc::new(RwLock::new(ModelManager::new()));

        let mut gx3d_file = Gx3DReader::new(config.get_gx3d_file_name());
        if let Some(gx3d_file) = &mut gx3d_file {
            let last_id: Id = gx3d_file.read();
            #[cfg(debug_gx3d)]
            vx_log_i!("GX3D last id is: {}", last_id);
            NEXT_ID.store(last_id, Ordering::Relaxed);
            macro_rules! set_table {
                ($mgr:ident) => {{
                    vx_result!($mgr.write()).set_gx3d_table(Gx3dTable::new(gx3d_file, config));
                }};
            }
            set_table!(camera_manager);
            let _audio_table = Gx3dTable::new(gx3d_file, config);
            set_table!(light_manager);
            set_table!(texture_manager);
            set_table!(font_manager);
            set_table!(mesh_manager);
            set_table!(model_manager);
            set_table!(skybox_manager);
            let _constraint_table = Gx3dTable::new(gx3d_file, config);
            set_table!(scene_manager);
        }

        Self {
            scene_manager,
            texture_manager,
            light_manager,
            camera_manager,
            mesh_manager,
            font_manager,
            model_manager,
            skybox_manager,
        }
    }

    pub(crate) fn set_engine(&self, engine: &Weak<RwLock<Engine>>) {
        vx_result!(self.texture_manager.write()).set_engine(engine.clone());
        vx_result!(self.scene_manager.write()).set_engine(engine.clone());
        vx_result!(self.light_manager.write()).set_engine(engine.clone());
        vx_result!(self.camera_manager.write()).set_engine(engine.clone());
        vx_result!(self.mesh_manager.write()).set_engine(engine.clone());
        vx_result!(self.model_manager.write()).set_engine(engine.clone());
        vx_result!(self.skybox_manager.write()).set_engine(engine.clone());
    }

    pub fn get_scene_manager(&self) -> &Arc<RwLock<SceneManager>> {
        return &self.scene_manager;
    }

    pub fn get_texture_manager(&self) -> &Arc<RwLock<TextureManager>> {
        return &self.texture_manager;
    }

    pub fn get_light_manager(&self) -> &Arc<RwLock<LightManager>> {
        return &self.light_manager;
    }

    pub fn get_camera_manager(&self) -> &Arc<RwLock<CameraManager>> {
        return &self.camera_manager;
    }

    pub fn get_mesh_manager(&self) -> &Arc<RwLock<MeshManager>> {
        return &self.mesh_manager;
    }

    pub fn get_font_manager(&self) -> &Arc<RwLock<FontManager>> {
        return &self.font_manager;
    }

    pub fn get_model_manager(&self) -> &Arc<RwLock<ModelManager>> {
        return &self.model_manager;
    }

    pub fn get_skybox_manager(&self) -> &Arc<RwLock<SkyboxManager>> {
        return &self.skybox_manager;
    }
}

unsafe impl Send for Manager {}
