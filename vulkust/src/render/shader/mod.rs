pub mod manager;
pub mod stage;

#[cfg(any(feature = "metal", target_os = "macos"))]
pub use super::super::metal::shader::ShaderTrait;
