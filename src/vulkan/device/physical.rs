use std::sync::Arc;
use super::super::super::system::vulkan::{
    VkResult,
    VkPhysicalDevice,
    vkEnumeratePhysicalDevices,
};
use super::super::instance::Instance;
pub struct Physical {
    instance: Arc<Instance>,
    vk_physical_device: VkPhysicalDevice,
}
impl Physical {
    pub fn new(instance: Arc<Instance>) -> Self {
        let mut physical = Physical {
            instance: instance,
            vk_physical_device: 0 as VkPhysicalDevice,
        };
        physical.init_physical_device();
        physical
    }
    fn init_physical_device(&mut self) {
        let mut gpu_count = 0u32;
        vulkan_check!(vkEnumeratePhysicalDevices(
            self.instance.vk_instance, &mut gpu_count as *mut u32, 0 as *mut VkPhysicalDevice));
        logdbg!(format!("Number of devices is: {}", gpu_count));
        let mut devices = vec![0 as VkPhysicalDevice; gpu_count as usize];
        vulkan_check!(vkEnumeratePhysicalDevices(
            self.instance.vk_instance, &mut gpu_count,
            devices.as_mut_ptr() as *mut VkPhysicalDevice));
        self.vk_physical_device = devices[0];
    }
}
impl Drop for Physical {
    fn drop(&mut self) {
//        unsafe {}
    }
}