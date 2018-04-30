#[cfg(target_os = "android")]
pub mod android;
#[cfg(target_os = "android")]
pub use self::android as os;
#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use self::linux as os;
// #[cfg(target_os = "macos")]
// pub mod mac;
// #[cfg(target_os = "windows")]
// pub mod windows;
pub mod application;
pub mod linker;
// pub mod file;
// #[cfg(metal)]
// pub mod metal;
// pub mod os;
// #[cfg(vulkan)]
// pub mod vulkan;
