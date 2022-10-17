pub struct Configurations {
    pub enable_anistropic_texture: bool,
    pub enable_ssao: bool,
    pub max_meshes_count: u64,
    pub max_models_count: u64,
    pub max_scenes_count: u64,
    pub max_textures_count: u64,
    pub max_shadow_maker_lights_count: u64,
    pub max_shadow_maps_count: u32,
    pub max_ssao_samples: u32,
    pub cascaded_shadows_count: u8,
    pub shadow_map_aspect: u32,
    pub max_shadow_maker_kernel_render_data_count: u64,
    pub content_width: u32,
    pub content_height: u32,
}

impl Default for Configurations {
    fn default() -> Self {
        Self {
            cascaded_shadows_count: 4,
            enable_anistropic_texture: true,
            enable_ssao: true,
            shadow_map_aspect: 1024,
            max_textures_count: 100,
            max_shadow_maker_lights_count: 100,
            max_meshes_count: 200,
            max_models_count: 200,
            max_scenes_count: 3,
            max_shadow_maps_count: 6,
            max_ssao_samples: 64,
            max_shadow_maker_kernel_render_data_count: 600,
            content_width: 1000,
            content_height: 700,
        }
    }
}
