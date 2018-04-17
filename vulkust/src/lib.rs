#![feature(repr_simd)]
#![feature(integer_atomics)]

#[cfg(appleos)]
#[macro_use]
pub extern crate objc;

#[macro_use]
pub mod macros;
// pub mod audio;
pub mod core;
// pub mod math;
pub mod render;
// pub mod sync;
pub mod system;
// #[macro_use]
// pub mod util;
// #[macro_use]
pub mod vulkan;
