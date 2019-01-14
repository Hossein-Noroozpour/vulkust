use super::super::system::os::application::Application as OsApp;
use super::instance::Instance;
use ash::extensions::khr::Surface as SurfaceLoader;
use ash::vk;
use std::mem::transmute;
use std::sync::{Arc, RwLock};

pub struct Surface {
    instance: Arc<Instance>,
    vk_data: vk::SurfaceKHR,
    loader: SurfaceLoader,
}

impl Surface {
    #[cfg(target_os = "ios")]
    pub(super) fn new(instance: &Arc<Instance>, os_app: &Arc<RwLock<OsApp>>) -> Self {
        let mut vk_data = 0 as vk::VkSurfaceKHR;
        let os_app = vxresult!(os_app.read());
        let mut create_info = vk::VkIOSSurfaceCreateInfoMVK::default();
        create_info.structure_type =
            vk::VkStructureType::VK_STRUCTURE_TYPE_IOS_SURFACE_CREATE_INFO_MVK;
        create_info.view = os_app.view;
        vulkan_check!(vk::vkCreateIOSSurfaceMVK(
            instance.get_data(),
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
    pub(super) fn new(instance: &Arc<Instance>, os_app: &Arc<RwLock<OsApp>>) -> Self {
        let mut vk_data = 0 as vk::VkSurfaceKHR;
        let os_app = vxresult!(os_app.read());
        let mut create_info = vk::VkMacOSSurfaceCreateInfoMVK::default();
        create_info.structure_type =
            vk::VkStructureType::VK_STRUCTURE_TYPE_MACOS_SURFACE_CREATE_INFO_MVK;
        create_info.view = os_app.get_view();
        vulkan_check!(vk::vkCreateMacOSSurfaceMVK(
            instance.get_data(),
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
    pub(super) fn new(instance: &Arc<Instance>, os_app: &Arc<RwLock<OsApp>>) -> Self {
        let mut vk_data = 0 as vk::VkSurfaceKHR;
        let os_app = vxresult!(os_app.read());
        let mut create_info = vk::VkAndroidSurfaceCreateInfoKHR::default();
        create_info.structure_type =
            vk::VkStructureType::VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR;
        create_info.window = unsafe { (*os_app.and_app).window };
        vulkan_check!(vk::vkCreateAndroidSurfaceKHR(
            instance.get_data(),
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
    pub(super) fn new(instance: &Arc<Instance>, os_app: &Arc<RwLock<OsApp>>) -> Self {
        let mut vk_surface = 0 as vk::VkSurfaceKHR;
        let os_app = vxresult!(os_app.read());
        let mut create_info = vk::VkXcbSurfaceCreateInfoKHR::default();
        create_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR;
        create_info.window = os_app.window;
        create_info.connection = os_app.connection;
        vulkan_check!(vk::vkCreateXcbSurfaceKHR(
            instance.get_data(),
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
    pub(super) fn new(instance: &Arc<Instance>, os_app: &Arc<RwLock<OsApp>>) -> Self {
        use ash::extensions::khr::Win32Surface;
        let os_app = vxresult!(os_app.read());
        let create_info = vk::Win32SurfaceCreateInfoKHR::builder()
            .hinstance(transmute(os_app.instance))
            .hwnd(transmute(os_app.window));
        let loader = Win32Surface::new(instance.get_entry(), instance.get_data());
        let vk_data = vxresult!(loader.create_win32_surface(&create_info, None));
        let loader = SurfaceLoader::new(instance.get_entry(), instance.get_data());
        Self {
            instance: instance.clone(),
            vk_data,
            loader,
        }
    }

    pub(super) fn get_data(&self) -> vk::SurfaceKHR {
        return self.vk_data;
    }

    pub(super) fn get_instance(&self) -> &Instance {
        return &self.instance;
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            self.loader.destroy_surface(self.vk_data, None);
        }
    }
}

#[cfg(debug_mode)]
impl std::fmt::Debug for Surface {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Vulkan Surface")
    }
}
