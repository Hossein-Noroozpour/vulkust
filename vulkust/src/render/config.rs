#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Configurations {
    pub enable_anistropic_texture: bool,
    pub max_number_mesh: u64,
    pub max_number_models: u64,
    pub max_number_scene: u64,
    pub max_number_texture: u64,
    pub number_cascaded_shadows: u8,
    pub shadow_map_aspect: u32,
}

impl Default for Configurations {
    fn default() -> Self {
        Configurations {
            number_cascaded_shadows: 6,
            enable_anistropic_texture: true,
            shadow_map_aspect: 1024,
            max_number_texture: 10,
            max_number_mesh: 20,
            max_number_models: 10,
            max_number_scene: 3,
        }
    }
}
