use super::types::Real;

pub const DEFAULT_WINDOW_WIDTH: Real = 1000.0;
pub const DEFAULT_WINDOW_HEIGHT: Real = 700.0;
pub const APPLICATION_NAME: &'static str = "Vulkust Demo Application";
pub const MAX_POINT_LIGHTS_COUNT: usize = 32; // todo in build script try to place this in shader
pub const MAX_DIRECTIONAL_LIGHTS_COUNT: usize = 8; // todo in build script try to place this in shader
pub const EPSILON: Real = 0.0001;
