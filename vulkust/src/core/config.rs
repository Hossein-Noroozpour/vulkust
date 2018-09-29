use super::super::render::config::Configurations as RenderConfig;

#[derive(Default)]
#[cfg_attr(debug_mode, derive(Debug))]
pub struct Configurations {
    pub render: RenderConfig,
}
