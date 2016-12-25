#[cfg(target_os = "android")] #[macro_use] pub mod android;
#[cfg(target_os = "android")] #[macro_use] pub mod jni;
#[macro_use] pub mod vulkan;
//#[cfg(target_os = "linux")] pub mod xcb;
//#[cfg(target_os = "linux")] pub mod vulkan_xcb;
