#[macro_use]
extern crate objc;
#[macro_use]
extern crate bitflags;

#[macro_use]
pub mod macros;
pub mod core;
pub mod io;
pub mod math;
#[cfg(any(target_os = "macos"))]
#[macro_use]
pub mod metal;
pub mod render;
pub mod sync;
pub mod system;
#[macro_use]
pub mod util;
#[cfg(any(target_os = "linux", target_os = "windows"))]
#[macro_use]
pub mod vulkan;
