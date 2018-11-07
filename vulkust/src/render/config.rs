pub const MAX_DIRECTIONAL_CASCADES_COUNT: u32 = 6;
pub const MAX_DIRECTIONAL_CASCADES_MATRIX_COUNT: u32 = 6;

#[derive(Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
pub struct Configurations {
    enable_anistropic_texture: bool,
    max_meshes_count: u64,
    max_models_count: u64,
    max_scenes_count: u64,
    max_textures_count: u64,
    max_shadow_maps_count: u32,
    cascaded_shadows_count: u8,
    shadow_map_aspect: u32,
    max_shadow_maker_kernek_render_data_count: u64,
}

impl Default for Configurations {
    fn default() -> Self {
        Configurations {
            cascaded_shadows_count: 4,
            enable_anistropic_texture: true,
            shadow_map_aspect: 1024,
            max_textures_count: 40,
            max_meshes_count: 20,
            max_models_count: 10,
            max_scenes_count: 3,
            max_shadow_maps_count: 6,
            max_shadow_maker_kernek_render_data_count: 6,
        }
    }
}

impl Configurations {
    pub fn get_cascaded_shadows_count(&self) -> u8 {
        return self.cascaded_shadows_count;
    }

    pub fn get_enable_anistropic_texture(&self) -> bool {
        return self.enable_anistropic_texture;
    }

    pub fn get_max_meshes_count(&self) -> u64 {
        return self.max_meshes_count;
    }

    pub fn get_max_models_count(&self) -> u64 {
        return self.max_models_count;
    }

    pub fn get_max_scenes_count(&self) -> u64 {
        return self.max_scenes_count;
    }

    pub fn get_max_textures_count(&self) -> u64 {
        return self.max_textures_count;
    }

    pub fn get_max_shadow_maps_count(&self) -> u32 {
        return self.max_shadow_maps_count;
    }

    pub fn get_shadow_map_aspect(&self) -> u32 {
        return self.shadow_map_aspect;
    }

    pub fn get_max_shadow_maker_kernek_render_data_count(&self) -> u64 {
        return self.max_shadow_maker_kernek_render_data_count;
    }
}
