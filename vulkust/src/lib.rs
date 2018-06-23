#![feature(repr_simd)]
#![feature(integer_atomics)]
#![feature(stmt_expr_attributes)]

pub extern crate image;
pub extern crate libc;
pub extern crate cgmath as math;

#[cfg(apple_os)]
#[macro_use]
pub extern crate objc;

#[cfg(any(apple_os, target_os = "android"))]
#[macro_use]
pub extern crate bitflags;

#[cfg(target_os = "windows")]
pub extern crate winapi;

#[macro_use]
pub mod macros;
// pub mod audio;
pub mod core;
// pub mod math;
pub mod render;
// pub mod sync;
pub mod system;
pub mod vulkan;
