#![feature(stmt_expr_attributes)]
#[macro_use] pub mod system;
mod vulkan;
mod render;

fn main() {
	render::initialize();
}
