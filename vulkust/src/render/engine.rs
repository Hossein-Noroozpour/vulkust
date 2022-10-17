use crate::{gapi, platform::os::application::Application as OsApp};

pub struct Engine {
    pub gapi_engine: gapi::engine::Engine,
}

impl Engine {
    pub fn new(os_app: &mut OsApp) -> Self {
        let gapi_engine = gapi::engine::Engine::new(os_app);
        Self { gapi_engine }
    }

    pub fn update(&mut self) {
        self.gapi_engine.update();
    }
}
