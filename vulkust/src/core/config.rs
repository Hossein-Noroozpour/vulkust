use super::super::render::config::Configurations as RenderConfig;

#[derive(Default)]
#[cfg_attr(debug_mode, derive(Debug))]
pub struct Configurations {
    render: RenderConfig,
}

impl Configurations {
    pub fn get_render(&self) -> &RenderConfig {
        return &self.render;
    }
}
