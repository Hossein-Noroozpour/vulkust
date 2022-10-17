pub struct Windowed {
    pub width: u32,
    pub height: u32,
}

impl Default for Windowed {
    fn default() -> Self {
        Self {
            width: 1000,
            height: 700,
        }
    }
}

pub enum ScreenState {
    Fullscreen,
    Windowed(Windowed),
}

pub struct Config {
    pub screen_state: ScreenState,
    pub application_name: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            screen_state: ScreenState::Windowed(Windowed::default()),
            application_name: "Gearoenix App".to_string(),
        }
    }
}
