#[cfg(any(target_os = "ios", target_os = "macos"))]
pub use super::super::super::metal::shader::stage::Stage;

#[cfg(any(target_os = "windows", target_os = "linux", target_os = "android"))]
pub use super::super::super::vulkan::shader::stage::Stage;
