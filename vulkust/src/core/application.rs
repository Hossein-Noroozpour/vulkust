use crate::platform::{base::Base as BaseOsApp, os::application::Application as OsApp};

pub struct Application {
    pub os_app: OsApp,
}

impl Application {
    pub fn new() -> Self {
        let mut myself = Self {
            os_app: OsApp::new(),
        };
        BaseOsApp::init(&mut myself.os_app);
        myself
    }

    pub fn run(&mut self) {
        self.os_app.run();
    }
}
