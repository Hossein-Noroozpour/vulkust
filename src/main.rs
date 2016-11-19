#![feature(stmt_expr_attributes)]
#[macro_use] pub mod system;
mod pipeline;
mod vulkan;

fn main() {
	pipeline::run();
}
