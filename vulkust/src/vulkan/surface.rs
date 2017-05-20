#[cfg(target_os = "windows")]
extern crate winapi;
use std::default::Default;
use std::ptr::null;
use std::sync::Arc;
use super::super::system::vulkan as vk;
use super::super::system::os::OsApplication;
use super::super::core::application::ApplicationTrait as CoreAppTrait;
#[cfg(target_os = "windows")]
use super::super::system::windows::vulkan::{
    VkWin32SurfaceCreateInfoKHR,
    vkCreateWin32SurfaceKHR,
};
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
        use super::super::system::android::window::ANativeWindow;
        let mut vk_data = 0 as vk::VkSurfaceKHR;
        let mut create_info = vk::VkAndroidSurfaceCreateInfoKHR::default();
        create_info.structure_type =
            vk::VkStructureType::VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR;
        create_info.window = unsafe { (*os_app).window };
        use std::mem::transmute;
        use std::ffi::CString;
        let vk_proc_name = CString::new("vkCreateAndroidSurfaceKHR").unwrap();
        let proc_ptr: vk::PFN_VkCreateAndroidSurfaceKhr = unsafe { transmute(
            vk::vkGetInstanceProcAddr(instance.vk_data, vk_proc_name.as_ptr()))};
        vulkan_check!(proc_ptr(
                instance.vk_data, &create_info, null(), &mut vk_data));
        // vulkan_check!(vkCreateAndroidSurfaceKHR(
        //         instance.vk_data, &create_info, null(), &mut vk_data));
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
    pub fn new(
            instance: Arc<Instance>, hinstance: HINSTANCE, hwnd: HWND) -> Self {
        use self::winapi::minwindef::HINSTANCE;
        use self::winapi::windef::HWND;
        let mut vk_data = 0 as vk::VkSurfaceKHR;
        let mut create_info = VkWin32SurfaceCreateInfoKHR::default();
        create_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR;
        create_info.hinstance = hinstance;
        create_info.hwnd = hwnd;
        vulkan_check!(vkCreateWin32SurfaceKHR(
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
