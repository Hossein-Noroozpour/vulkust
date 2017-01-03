use super::super::vulkan::Driver as VulkanDriver;
use super::super::vulkan::surface::Surface;
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
    pub fn initialize(&mut self, surface: Surface) {
        self.vulkan_driver.initialize(surface);
    }
}