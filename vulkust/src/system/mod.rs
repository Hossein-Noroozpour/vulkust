#[cfg(target_os = "android")]
pub mod android;
#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "macos")]
#[macro_use]
pub mod mac;
#[cfg(target_os = "windows")]
pub mod windows;
pub mod application;
pub mod os;
#[cfg(any(target_os = "linux", target_os = "windows"))]
pub mod vulkan;
