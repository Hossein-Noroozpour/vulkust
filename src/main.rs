#![feature(stmt_expr_attributes)]
mod pipeline;
mod vulkan;
mod system;

fn main() {
	pipeline::run();
}
