#![feature(repr_simd)]
#![feature(integer_atomics)]
#![feature(stmt_expr_attributes)]

pub extern crate libc;

#[cfg(apple_os)]
#[macro_use]
pub extern crate objc;

#[cfg(any(apple_os, target_os = "android"))]
#[macro_use]
pub extern crate bitflags;

#[cfg(target_os = "windows")]
pub extern crate winapi;

// pub extern crate cgmath as math;
// pub extern crate image;

#[macro_use]
pub mod macros;
// pub mod audio;
pub mod core;
// pub mod math;
pub mod render;
// pub mod sync;
pub mod system;
pub mod vulkan;
