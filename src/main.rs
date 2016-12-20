#![feature(stmt_expr_attributes)]
extern crate libc;

#[macro_use] pub mod system;
pub mod io;
pub mod math;
pub mod render;
pub mod texture;
pub mod vulkan;

fn main() {
	render::initialize();
}
