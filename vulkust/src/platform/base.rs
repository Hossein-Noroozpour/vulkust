use super::{
    super::render::engine::Engine as RenderEngine, config::Config,
    os::application::Application as OsApp,
};

pub struct Base {
    pub config: Config,
    pub is_running: bool,
    pub render_engine: Option<RenderEngine>,
}

impl Base {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            is_running: false,
            render_engine: None,
        }
    }

    pub fn init(os_app: &mut OsApp) {
        let render_engine = RenderEngine::new(os_app);
        os_app.base.render_engine.replace(render_engine);
    }
}
