#![feature(repr_simd)]

#[cfg(appleos)]
#[macro_use]
pub extern crate objc;
#[macro_use]
extern crate bitflags;

#[macro_use]
pub mod macros;
pub mod core;
pub mod math;
#[cfg(metal)]
#[macro_use]
pub mod metal;
pub mod render;
pub mod sync;
pub mod system;
#[macro_use]
pub mod util;
#[cfg(vulkan)]
#[macro_use]
pub mod vulkan;
