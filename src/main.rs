#![feature(stmt_expr_attributes)]
#[macro_use] pub mod system;
mod vulkan;

fn main() {
	vulkan::run();
}
