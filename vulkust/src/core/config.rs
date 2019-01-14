use super::super::render::config::Configurations as RenderConfig;

#[derive(Clone)]
#[cfg_attr(debug_mode, derive(Debug))]
pub struct Configurations {
    gx3d_file_name: String,
    window_width: usize,
    window_height: usize,
    fullscreen: bool,
    render: RenderConfig,
    application_name: String,
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

    pub fn set_window_width(&mut self, v: usize) {
        self.window_width = v;
    }

    pub fn set_window_height(&mut self, v: usize) {
        self.window_height = v;
    }

    pub fn set_fullscreen(&mut self, v: bool) {
        self.fullscreen = v;
    }

    pub fn get_window_width(&self) -> usize {
        return self.window_width;
    }

    pub fn get_window_height(&self) -> usize {
        return self.window_height;
    }

    pub fn get_fullscreen(&self) -> bool {
        return self.fullscreen;
    }

    pub fn get_application_name(&self) -> &str {
        return &self.application_name;
    }

    pub fn set_application_name(&mut self, name: String) {
        self.application_name = name;
    }
}

impl Default for Configurations {
    fn default() -> Self {
        Self {
            gx3d_file_name: "gx3d/data.gx3d".to_string(),
            window_width: 1000,
            window_height: 700,
            fullscreen: false,
            render: RenderConfig::default(),
            application_name: "Vulkust Application".to_string(),
        }
    }
}
