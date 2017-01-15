use std::sync::Arc;
use std::ptr::null_mut;
use super::super::super::system::vulkan::{
    VkResult,
    VkPhysicalDevice,
    VkQueueFamilyProperties,
    vkEnumeratePhysicalDevices,
    vkGetPhysicalDeviceQueueFamilyProperties,
};
use super::super::instance::Instance;
// this properties of device is using in setup process no other time, no in rendering procedure
// if this assumption was wrong cache this properties in this structure.
//     Properties
//     Features
//     MemoryProperties
//     QueueFamilyProperties
pub struct Physical {
    pub instance: Arc<Instance>,
    pub vk_physical_device: VkPhysicalDevice,
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
    pub fn get_queue_family_properties(&self) -> Vec<VkQueueFamilyProperties> {
        let mut count = 0u32;
        unsafe {
            vkGetPhysicalDeviceQueueFamilyProperties(
                self.vk_physical_device, &mut count, null_mut());
        }
        let mut families = vec![VkQueueFamilyProperties::default(); count as usize];
        unsafe {
            vkGetPhysicalDeviceQueueFamilyProperties(
                self.vk_physical_device, &mut count, families.as_mut_ptr());
        }
        return families;
    }
}
impl Drop for Physical {
    fn drop(&mut self) {
//        unsafe {} TODO: it may be unnecessary
    }
}