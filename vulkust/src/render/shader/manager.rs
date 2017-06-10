#[cfg(any(feature = "metal", target_os = "macos"))]
use super::super::super::metal::shader::manager::Manager as ShaderManager;

pub type Manager = ShaderManager;
