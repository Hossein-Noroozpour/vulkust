#![feature(repr_simd)]
#![feature(integer_atomics)]
#![feature(stmt_expr_attributes)]
#![feature(duration_as_u128)]
#![feature(concat_idents)]

pub extern crate cgmath;
pub extern crate gltf;
pub extern crate image;
pub extern crate libc;
pub extern crate num_cpus;
pub extern crate rand;
pub extern crate rusttype;

#[cfg(apple_os)]
#[macro_use]
pub extern crate objc;

#[cfg(unix_based_os)]
#[macro_use]
pub extern crate bitflags;

#[cfg(target_os = "windows")]
pub extern crate winapi;

#[macro_use]
pub mod macros;

// pub mod audio;
#[cfg(blank_gapi)]
pub mod blank_gapi;
pub mod collision;
pub mod core;
#[cfg(directx12_api)]
pub mod d3d12;
pub mod physics;
pub mod render;
pub mod system;
#[cfg(vulkan_api)]
pub mod vulkan;
