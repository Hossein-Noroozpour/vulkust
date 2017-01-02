use super::super::vulkan::Driver as VulkanDriver;

pub struct Application {
    pub vulkan_driver: VulkanDriver,
}

impl Application {
    pub fn new() -> Self {
        logdbg!("Initializing");
        Application {
            vulkan_driver: VulkanDriver::new(),
        }
    }
    pub fn start(&mut self) {}
}