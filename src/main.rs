#![feature(stmt_expr_attributes)]
#[macro_use] pub mod system;
pub mod vulkan;
pub mod render;
pub mod math;

fn main() {
	render::initialize();
}
