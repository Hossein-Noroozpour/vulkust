#[derive(Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
pub struct Configurations {
    pub(crate) enable_anistropic_texture: bool,
    pub(crate) max_meshes_count: u64,
    pub(crate) max_models_count: u64,
    pub(crate) max_scenes_count: u64,
    pub(crate) max_textures_count: u64,
    pub(crate) max_shadow_maps_count: u32,
    pub(crate) cascaded_shadows_count: u8,
    pub(crate) shadow_map_aspect: u32,
    pub(crate) max_shadow_maker_kernek_render_data_count: u64,
}

impl Default for Configurations {
    fn default() -> Self {
        Configurations {
            cascaded_shadows_count: 4,
            enable_anistropic_texture: true,
            shadow_map_aspect: 1024,
            max_textures_count: 30,
            max_meshes_count: 20,
            max_models_count: 10,
            max_scenes_count: 3,
            max_shadow_maps_count: 6,
            max_shadow_maker_kernek_render_data_count: 1,
        }
    }
}
