use super::vulkan::instance::Instance;
use super::vulkan::device::Device;

pub fn run() {
	let ins = Instance::new();
	println!("Instance created.");
	let dev = Device::new(&ins);

	let _ = dev;
}
