use vulkan::instance::Instance;
use vulkan::device::Device;
use vulkan::window::Window;

use std;

pub fn run() {
	let ins = Instance::new();
	println!("Instance created.");
	let dev = Device::new(&ins);
	let win = Window::new(800, 500);
	let _ = dev;
    std::thread::sleep(std::time::Duration::from_millis(4000));
}
