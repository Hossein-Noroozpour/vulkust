use std::default::Default;
use std::ptr::null;
use std::sync::Arc;
use super::super::system::vulkan as vk;
// Android
#[cfg(target_os = "android")]
use super::super::system::android::vulkan::{
    VkAndroidSurfaceCreateInfoKHR,
    vkCreateAndroidSurfaceKHR,
};
#[cfg(target_os = "android")]
use super::super::system::android::window::ANativeWindow;
// Linux
#[cfg(target_os = "linux")]
use super::super::system::linux::vulkan::{
    VkXcbSurfaceCreateInfoKHR,
    vkCreateXcbSurfaceKHR,
};
#[cfg(target_os = "linux")]
use super::super::system::linux::xcb;

use super::instance::Instance;

pub struct Surface {
    pub instance: Arc<Instance>,
    pub vk_data: vk::VkSurfaceKHR,
}

impl Surface {
    #[cfg(target_os = "android")]
    pub fn new(instance: Arc<Instance>, window: *mut ANativeWindow) -> Self {
        let mut vk_surface = 0 as VkSurfaceKHR;
        let mut create_info = VkAndroidSurfaceCreateInfoKHR::default();
        create_info.structure_type =
            VkStructureType::VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR;
        create_info.window = window;
        vulkan_check!(vkCreateAndroidSurfaceKHR(
                instance.vk_instance, &create_info, null(), &mut vk_surface));
        logdbg!(format!("vk surface {:?}", vk_surface));
        Surface {
            instance: instance,
            vk_surface: vk_surface,
        }
    }
    #[cfg(target_os = "linux")]
    pub fn new(
            instance: Arc<Instance>, connection: *mut xcb::xcb_connection_t,
            window: xcb::xcb_window_t,) -> Self {
        let mut vk_surface = 0 as vk::VkSurfaceKHR;
        let mut create_info = VkXcbSurfaceCreateInfoKHR::default();
        create_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR;
        create_info.window = window;
        create_info.connection = connection;
        vulkan_check!(vkCreateXcbSurfaceKHR(
                instance.vk_data, &create_info, null(), &mut vk_surface));
        logi!("vk surface {:?}", vk_surface);
        Surface {
            instance: instance,
            vk_data: vk_surface,
        }
    }
    #[cfg(target_os = "windows")]
    pub fn new(
            instance: Arc<Instance>, connection: *mut xcb::xcb_connection_t,
            window: xcb::xcb_window_t,) -> Self {
        let mut vk_surface = 0 as vk::VkSurfaceKHR;
        let mut create_info = VkXcbSurfaceCreateInfoKHR::default();
        create_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR;
        create_info.window = window;
        create_info.connection = connection;
        vulkan_check!(vkCreateXcbSurfaceKHR(
                instance.vk_data, &create_info, null(), &mut vk_surface));
        logi!("vk surface {:?}", vk_surface);
        Surface {
            instance: instance,
            vk_data: vk_surface,
        }
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            logi!("terminated {:?}", self.vk_data);
            vk::vkDestroySurfaceKHR(self.instance.vk_data, self.vk_data, null());
        }
    }
}
