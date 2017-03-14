#[cfg(target_os = "android")]
pub mod android;

#[cfg(target_os = "linux")]
pub mod linux;

// #[macro_use]
// pub mod vulkan;
pub mod application;
pub mod os;
