#![feature(repr_simd)]
#![feature(integer_atomics)]
#![feature(stmt_expr_attributes)]

#[cfg(target_os = "macos")]
#[macro_use]
pub extern crate objc;

#[cfg(target_os = "macos")]
#[macro_use]
pub extern crate bitflags;

#[cfg(target_os = "windows")]
extern crate winapi;

#[cfg(target_os = "windows")]
extern crate kernel32;

#[cfg(target_os = "windows")]
extern crate user32;

#[cfg(target_os = "windows")]
extern crate gdi32;

// pub extern crate cgmath as math;
// pub extern crate image;
#[cfg(any(target_os = "macos", target_os = "linux"))]
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
