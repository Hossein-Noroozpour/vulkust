#![feature(repr_simd)]
#![feature(integer_atomics)]
#![feature(stmt_expr_attributes)]

#[cfg(target_os = "macos")]
#[macro_use]
pub extern crate objc;

#[cfg(target_os = "macos")]
#[macro_use]
pub extern crate bitflags;

// pub extern crate cgmath as math;
// pub extern crate image;
#[cfg(target_os = "macos")]
pub extern crate libc;

#[macro_use]
pub mod macros;
// pub mod audio;
pub mod core;
// pub mod math;
pub mod render;
// pub mod sync;
pub mod system;
pub mod vulkan;
