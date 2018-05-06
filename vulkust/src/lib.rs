#![feature(repr_simd)]
#![feature(integer_atomics)]
#![feature(stmt_expr_attributes)]

#[cfg(apple_os)]
#[macro_use]
pub extern crate objc;

#[macro_use]
pub extern crate bitflags;

pub extern crate cgmath as math;
pub extern crate image;
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
