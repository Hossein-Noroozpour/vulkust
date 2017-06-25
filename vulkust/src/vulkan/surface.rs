#[cfg(target_os = "windows")]
extern crate winapi;

use std::default::Default;
use std::ptr::null;
use std::sync::Arc;
use super::super::system::vulkan as vk;
use super::super::system::os::OsApplication;
use super::super::core::application::ApplicationTrait as CoreAppTrait;
use super::instance::Instance;

pub struct Surface {
    pub instance: Arc<Instance>,
    pub vk_data: vk::VkSurfaceKHR,
}

impl Surface {
    #[cfg(target_os = "android")]
    pub fn new<CoreApp>(
            instance: Arc<Instance>, os_app: *mut OsApplication<CoreApp>) -> Self
            where CoreApp: CoreAppTrait  {
        loge!("Reached {:?}", os_app);
        use super::super::system::android::window::ANativeWindow;
        let mut vk_data = 0 as vk::VkSurfaceKHR;
        let mut create_info = vk::VkAndroidSurfaceCreateInfoKHR::default();
        create_info.structure_type =
            vk::VkStructureType::VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR;
        create_info.window = unsafe { (*os_app).window };
        loge!("{:?}.{:?}", os_app, create_info.window);
        vulkan_check!(vk::vkCreateAndroidSurfaceKHR(
                instance.vk_data, &create_info, null(), &mut vk_data));
        loge!("Reached");
        Surface {
            instance: instance,
            vk_data: vk_data,
        }
    }
    #[cfg(target_os = "linux")]
    pub fn new<CoreApp>(
            instance: Arc<Instance>, os_app: *mut OsApplication<CoreApp>) -> Self
            where CoreApp: CoreAppTrait  {
        let mut vk_surface = 0 as vk::VkSurfaceKHR;
        let mut create_info = vk::VkXcbSurfaceCreateInfoKHR::default();
        create_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR;
        create_info.window = unsafe { (*os_app).window };
        create_info.connection = unsafe { (*os_app).connection };
        vulkan_check!(vk::vkCreateXcbSurfaceKHR(
                instance.vk_data, &create_info, null(), &mut vk_surface));
        logi!("vk surface {:?}", vk_surface);
        Surface {
            instance: instance,
            vk_data: vk_surface,
        }
    }
    #[cfg(target_os = "windows")]
    pub fn new<CoreApp>(
            instance: Arc<Instance>, os_app: *mut OsApplication<CoreApp>) -> Self
            where CoreApp: CoreAppTrait  {
        let mut vk_data = 0 as vk::VkSurfaceKHR;
        let mut create_info = vk::VkWin32SurfaceCreateInfoKHR::default();
        create_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR;
        create_info.hinstance = unsafe { (*os_app).h_instance };
        create_info.hwnd = unsafe { (*os_app).h_window };
        vulkan_check!(vk::vkCreateWin32SurfaceKHR(
                instance.vk_data, &create_info, null(), &mut vk_data));
        logi!("vk surface {:?}", vk_data);
        Surface {
            instance: instance,
            vk_data: vk_data,
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
