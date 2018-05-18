use super::super::system::os::application::Application as OsApp;
use super::instance::Instance;
use super::vulkan as vk;
use std::default::Default;
use std::ptr::null;
use std::sync::{Arc, RwLock};

pub struct Surface {
    pub instance: Arc<Instance>,
    pub vk_data: vk::VkSurfaceKHR,
}

impl Surface {
    #[cfg(target_os = "ios")]
    pub fn new(instance: &Arc<Instance>, os_app: &Arc<RwLock<OsApp>>) -> Self {
        let mut vk_data = 0 as vk::VkSurfaceKHR;
        let os_app = vxresult!(os_app.read());
        let mut create_info = vk::VkIOSSurfaceCreateInfoMVK::default();
        create_info.structure_type =
            vk::VkStructureType::VK_STRUCTURE_TYPE_IOS_SURFACE_CREATE_INFO_MVK;
        create_info.view = os_app.view;
        vulkan_check!(vk::vkCreateIOSSurfaceMVK(
            instance.vk_data,
            &create_info,
            null(),
            &mut vk_data,
        ));
        Surface {
            instance: instance.clone(),
            vk_data,
        }
    }

    #[cfg(target_os = "macos")]
    pub fn new(instance: &Arc<Instance>, os_app: &Arc<RwLock<OsApp>>) -> Self {
        let mut vk_data = 0 as vk::VkSurfaceKHR;
        let os_app = vxresult!(os_app.read());
        let mut create_info = vk::VkMacOSSurfaceCreateInfoMVK::default();
        create_info.structure_type =
            vk::VkStructureType::VK_STRUCTURE_TYPE_MACOS_SURFACE_CREATE_INFO_MVK;
        create_info.view = os_app.view;
        vulkan_check!(vk::vkCreateMacOSSurfaceMVK(
            instance.vk_data,
            &create_info,
            null(),
            &mut vk_data,
        ));
        Surface {
            instance: instance.clone(),
            vk_data,
        }
    }

    #[cfg(target_os = "android")]
    pub fn new(instance: &Arc<Instance>, os_app: &Arc<RwLock<OsApp>>) -> Self {
        let mut vk_data = 0 as vk::VkSurfaceKHR;
        let os_app = vxresult!(os_app.read());
        let mut create_info = vk::VkAndroidSurfaceCreateInfoKHR::default();
        create_info.structure_type =
            vk::VkStructureType::VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR;
        create_info.window = *vxunwrap!(*vxresult!(os_app.window.read()));
        vulkan_check!(vk::vkCreateAndroidSurfaceKHR(
            instance.vk_data,
            &create_info,
            null(),
            &mut vk_data,
        ));
        Surface {
            instance: instance.clone(),
            vk_data,
        }
    }

    #[cfg(target_os = "linux")]
    pub fn new(instance: &Arc<Instance>, os_app: &Arc<RwLock<OsApp>>) -> Self {
        let mut vk_surface = 0 as vk::VkSurfaceKHR;
        let os_app = vxresult!(os_app.read());
        let mut create_info = vk::VkXcbSurfaceCreateInfoKHR::default();
        create_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR;
        create_info.window = os_app.window;
        create_info.connection = os_app.connection;
        vulkan_check!(vk::vkCreateXcbSurfaceKHR(
            instance.vk_data,
            &create_info,
            null(),
            &mut vk_surface,
        ));
        Surface {
            instance: instance.clone(),
            vk_data: vk_surface,
        }
    }

    #[cfg(target_os = "windows")]
    pub fn new<CoreApp>(instance: Arc<Instance>, os_app: *mut OsApplication<CoreApp>) -> Self
    where
        CoreApp: CoreAppTrait,
    {
        let mut vk_data = 0 as vk::VkSurfaceKHR;
        let mut create_info = vk::VkWin32SurfaceCreateInfoKHR::default();
        create_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR;
        create_info.hinstance = unsafe { (*os_app).h_instance };
        create_info.hwnd = unsafe { (*os_app).h_window };
        vulkan_check!(vk::vkCreateWin32SurfaceKHR(
            instance.vk_data,
            &create_info,
            null(),
            &mut vk_data,
        ));
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
            vk::vkDestroySurfaceKHR(self.instance.vk_data, self.vk_data, null());
        }
    }
}
