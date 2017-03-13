#[cfg(any(not(debug_assertions), target_os = "android"))]
pub const FULLSCREEN_MODE: bool = true;
#[cfg(not(any(not(debug_assertions), target_os = "android")))]
pub const FULLSCREEN_MODE: bool = false;

