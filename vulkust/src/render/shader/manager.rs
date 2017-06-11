#[cfg(any(feature = "metal", target_os = "macos"))]
pub use super::super::super::metal::shader::manager::Manager;
