#[cfg(target_os = "android")]
pub mod android;
#[cfg(target_os = "android")]
pub use self::android as os;
#[cfg(apple_os)]
pub mod apple;
#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use self::linux as os;
#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "macos")]
pub use self::macos as os;
// pub mod application;
pub mod linker;
// pub mod file;
// pub mod os;
// #[cfg(vulkan)]
// pub mod vulkan;
