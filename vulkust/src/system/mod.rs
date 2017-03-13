// #[cfg(target_os = "android")]
// #[macro_use]
// pub mod android;
// #[macro_use]
// pub mod application;
// #[macro_use]
// pub mod vulkan;
#[cfg(target_os = "linux")]
#[macro_use]
pub mod linux;
//#[cfg(target_os = "linux")] pub mod vulkan_xcb;
