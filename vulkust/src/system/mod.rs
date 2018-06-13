#[cfg(target_os = "android")]
pub mod android;
#[cfg(target_os = "android")]
pub use self::android as os;
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub mod apple;
#[cfg(target_os = "ios")]
pub mod ios;
#[cfg(target_os = "ios")]
pub use self::ios as os;
#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use self::linux as os;
#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "macos")]
pub use self::macos as os;
#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use self::windows as os;
// pub mod linker;
pub mod file;
