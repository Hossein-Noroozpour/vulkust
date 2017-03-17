#[cfg(target_os = "android")]
pub mod android;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "windows")]
pub mod windows;

// #[macro_use]
// pub mod vulkan;
pub mod application;
pub mod os;
