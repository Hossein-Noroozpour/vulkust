use super::super::render::config::Configurations as RenderConfig;

#[derive(Default)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Configurations {
    pub render: RenderConfig,
}
