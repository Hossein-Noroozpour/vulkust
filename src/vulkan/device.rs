use super::instance::Instance;
use ::system::vulkan::{
    uint32_t,
//    vulkan_check,
    VkResult,
    VkPhysicalDevice,
    vkEnumeratePhysicalDevices,
};

pub struct Device {

}

impl Device {
    pub fn new(instance: &Instance) -> Self {
        let mut gpu_count: uint32_t = 0;
        vulkan_check!(
            vkEnumeratePhysicalDevices(
                instance.vk_instance,
                &mut gpu_count as *mut uint32_t,
                0 as *mut VkPhysicalDevice));
        println!("Number of devices is: {}", gpu_count);
        Device {}
    }
}