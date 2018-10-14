#[derive(Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
pub struct Configurations {
    pub enable_anistropic_texture: bool,
    pub max_number_mesh: u64,
    pub max_number_models: u64,
    pub max_number_scene: u64,
    pub max_number_texture: u64,
    pub cascaded_shadows_count: u8,
    pub shadow_map_aspect: u32,
}

impl Default for Configurations {
    fn default() -> Self {
        Configurations {
            cascaded_shadows_count: 4,
            enable_anistropic_texture: true,
            shadow_map_aspect: 1024,
            max_number_texture: 30,
            max_number_mesh: 20,
            max_number_models: 10,
            max_number_scene: 3,
        }
    }
}
