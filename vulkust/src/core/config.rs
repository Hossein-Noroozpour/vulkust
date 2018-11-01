use super::super::render::config::Configurations as RenderConfig;

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Configurations {
    gx3d_file_name: String,
    render: RenderConfig,
}

impl Configurations {
    pub fn get_render(&self) -> &RenderConfig {
        return &self.render;
    }

    pub fn set_render(&mut self, conf: RenderConfig) {
        self.render = conf;
    }

    pub fn get_gx3d_file_name(&self) -> &str {
        return &self.gx3d_file_name;
    }

    pub fn set_gx3d_file_name(&mut self, name: String) {
        self.gx3d_file_name = name;
    }
}

impl Default for Configurations {
    fn default() -> Self {
        Self {
            gx3d_file_name: "gx3d/data.gx3d".to_string(),
            render: RenderConfig::default(),
        }
    }
}
