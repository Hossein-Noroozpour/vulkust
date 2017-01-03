use std::default::Default;
use std::ptr::null;
use std::sync::Arc;
use super::super::system::vulkan::{
    VkResult,
    VkSurfaceKHR,
    VkStructureType,
    vkDestroySurfaceKHR,
};
#[cfg(target_os = "android")]
use super::super::system::android::vulkan::{
    VkAndroidSurfaceCreateInfoKHR,
    vkCreateAndroidSurfaceKHR,
};
#[cfg(target_os = "android")]
use super::super::system::android::window::ANativeWindow;
use super::instance::Instance;

pub struct Surface {
    pub instance: Arc<Instance>,
    pub vk_surface: VkSurfaceKHR,
}

impl Surface {
    #[cfg(target_os = "android")]
    pub fn new(instance: Arc<Instance>, window: *mut ANativeWindow) -> Self {
        let mut vk_surface = 0 as VkSurfaceKHR;
        {
            let mut create_info = VkAndroidSurfaceCreateInfoKHR::default();
            create_info.structure_type =
                VkStructureType::VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR;
            create_info.window = window;
            vulkan_check!(vkCreateAndroidSurfaceKHR(
                instance.vk_instance, &create_info, null(), &mut vk_surface));
        }
        Surface {
            instance: instance,
            vk_surface: vk_surface,
        }
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            vkDestroySurfaceKHR(self.instance.vk_instance, self.vk_surface, null());
        }
    }
}
