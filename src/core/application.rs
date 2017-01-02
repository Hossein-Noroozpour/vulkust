use super::super::vulkan::Driver as VulkanDriver;

pub struct Application {
    pub vulkan_driver: VulkanDriver,
}

impl Application {
    pub fn initialize(&mut self) {
        logdbg!("Initializing");
        self.vulkan_driver.initialize();
        logdbg!("done!");
    }
    pub fn start(&mut self) {}
}