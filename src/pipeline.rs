use vulkan::instance::Instance;
use vulkan::device::Device;
use vulkan::window::Window;

use std;
use std::sync::{
	Arc,
	RwLock,
};

pub fn run() {
	let ins = Arc::new(RwLock::new(Instance::new()));
	let dev = Arc::new(RwLock::new(Device::new(ins.clone())));
	let win = Arc::new(RwLock::new(Window::new(dev.clone())));
    std::thread::sleep(std::time::Duration::from_millis(4000));
    let _ = win;
}
