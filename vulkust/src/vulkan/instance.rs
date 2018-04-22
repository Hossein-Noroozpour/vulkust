use std::default::Default;
use std::ffi::CString;
use std::ptr::null;

use super::super::core::string::cstrings_to_ptrs;
use super::vulkan as vk;

#[cfg(debug_mode)]
mod debug {
    use std::ffi::{CStr, CString};
    use std::mem::transmute;
    use std::os::raw::{c_char, c_void};
    use std::ptr::{null, null_mut};

    use super::vk;

    use super::super::super::core::string::{slice_to_string, strings_to_cstrings};

    pub fn get_function(vk_instance: vk::VkInstance, s: &str) -> vk::PFN_vkVoidFunction {
        let n = CString::new(s).unwrap();
        let proc_addr = unsafe { vk::vkGetInstanceProcAddr(vk_instance, n.as_ptr()) };
        if proc_addr == unsafe { transmute(0usize) } {
            vxlogf!("Function pointer not found");
        }
        return proc_addr;
    }

    pub struct Debugger {
        vk_data: vk::VkDebugReportCallbackEXT,
    }

    impl Debugger {
        pub fn new(vk_instance: vk::VkInstance) -> Self {
            let mut report_callback_create_info = vk::VkDebugReportCallbackCreateInfoEXT::default();
            report_callback_create_info.sType =
                vk::VkStructureType::VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT;
            report_callback_create_info.flags =
                (vk::VkDebugReportFlagBitsEXT::VK_DEBUG_REPORT_INFORMATION_BIT_EXT as u32)
                    | (vk::VkDebugReportFlagBitsEXT::VK_DEBUG_REPORT_WARNING_BIT_EXT as u32)
                    | (vk::VkDebugReportFlagBitsEXT::VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT
                        as u32)
                    | (vk::VkDebugReportFlagBitsEXT::VK_DEBUG_REPORT_ERROR_BIT_EXT as u32)
                    | (vk::VkDebugReportFlagBitsEXT::VK_DEBUG_REPORT_DEBUG_BIT_EXT as u32)
                        as vk::VkDebugReportFlagsEXT;
            report_callback_create_info.pfnCallback = vulkan_debug_callback;
            report_callback_create_info.pUserData = null_mut();
            let mut vk_debug_callback = 0 as vk::VkDebugReportCallbackEXT;
            let create_debug_report_callback: vk::PFN_vkCreateDebugReportCallbackEXT =
                unsafe { transmute(get_function(vk_instance, "vkCreateDebugReportCallbackEXT")) };
            vulkan_check!(create_debug_report_callback(
                vk_instance,
                &report_callback_create_info,
                null(),
                &mut vk_debug_callback,
            ));
            Debugger {
                vk_data: vk_debug_callback,
            }
        }

        pub fn terminate(&mut self, vk_instance: vk::VkInstance) {
            let destroy_debug_report_callback: vk::PFN_vkDestroyDebugReportCallbackEXT =
                unsafe { transmute(get_function(vk_instance, "vkDestroyDebugReportCallbackEXT")) };
            unsafe {
                destroy_debug_report_callback(vk_instance, self.vk_data, null());
            }
            self.vk_data = 0 as vk::VkDebugReportCallbackEXT;
        }
    }

    impl Drop for Debugger {
        fn drop(&mut self) {
            if self.vk_data != null_mut() {
                vxlogf!("Unexpected drop of debugger.");
            }
        }
    }

    unsafe extern "C" fn vulkan_debug_callback(
        flags: vk::VkDebugReportFlagsEXT,
        obj_type: vk::VkDebugReportObjectTypeEXT,
        src_obj: u64,
        location: usize,
        msg_code: i32,
        layer_prefix: *const c_char,
        msg: *const c_char,
        user_data: *mut c_void,
    ) -> u32 {
        let mut flg = String::new();
        if flags & (vk::VkDebugReportFlagBitsEXT::VK_DEBUG_REPORT_INFORMATION_BIT_EXT as u32) != 0 {
            flg += "info, ";
        }
        if flags & (vk::VkDebugReportFlagBitsEXT::VK_DEBUG_REPORT_WARNING_BIT_EXT as u32) != 0 {
            flg += "warn, ";
        }
        if flags
            & (vk::VkDebugReportFlagBitsEXT::VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT as u32)
            != 0
        {
            flg += "performance, ";
        }
        if flags & (vk::VkDebugReportFlagBitsEXT::VK_DEBUG_REPORT_ERROR_BIT_EXT as u32) != 0 {
            flg += "error, ";
        }
        if flags & (vk::VkDebugReportFlagBitsEXT::VK_DEBUG_REPORT_DEBUG_BIT_EXT as u32) != 0 {
            flg += "debug, ";
        }
        vxlogi!(
            "flag: {}, obj_type: {}, src_obj: {:?}, location: {:?}, msg_code: {:?}, layer_prefix: \
             {:?}, msg : {:?}, user_data {:?}",
            flg,
            obj_type,
            src_obj,
            location,
            msg_code,
            CStr::from_ptr(layer_prefix).to_str(),
            CStr::from_ptr(msg).to_str(),
            user_data
        );
        0u32
    }

    pub fn enumerate_layers() -> Vec<CString> {
        let mut layer_count = 0u32;
        unsafe {
            vk::vkEnumerateInstanceLayerProperties(&mut layer_count, null_mut());
        }
        vxlogi!("Number of layers found is: {}", layer_count);
        let mut available_layers = vec![vk::VkLayerProperties::default(); layer_count as usize];
        unsafe {
            vk::vkEnumerateInstanceLayerProperties(&mut layer_count, available_layers.as_mut_ptr());
        }
        let mut layers_names = Vec::new();
        for i in 0..available_layers.len() {
            let name = slice_to_string(&available_layers[i].layerName);
            let des = slice_to_string(&available_layers[i].description);
            vxlogi!("Layer {} with des: {} found.", name, des);
        }
        layers_names.push("VK_LAYER_GOOGLE_threading".to_string());
        // layers_names.push("VK_LAYER_LUNARG_api_dump".to_string());
        layers_names.push("VK_LAYER_LUNARG_parameter_validation".to_string());
        layers_names.push("VK_LAYER_LUNARG_object_tracker".to_string());
        layers_names.push("VK_LAYER_LUNARG_core_validation".to_string());
        // layers_names.push("VK_LAYER_LUNARG_image".to_string());
        // layers_names.push("VK_LAYER_LUNARG_swapchain".to_string());
        layers_names.push("VK_LAYER_GOOGLE_unique_objects".to_string());
        strings_to_cstrings(layers_names)
    }

}

#[cfg(not(debug_mode))]
mod debug {
    use super::vk;
    use std::ffi::CString;

    pub struct Debugger {}

    impl Debugger {
        pub fn new(_vk_instance: vk::VkInstance) -> Self {
            Debugger {}
        }

        pub fn terminate(&mut self, _vk_instance: vk::VkInstance) {}
    }

    pub fn enumerate_layers() -> Vec<CString> {
        Vec::new()
    }
}

pub struct Instance {
    pub vk_data: vk::VkInstance,
    debugger: debug::Debugger,
}

impl Instance {
    pub fn new() -> Self {
        let application_name = CString::new("Vulkust App").unwrap();
        let engine_name = CString::new("Vulkust").unwrap();
        let mut application_info = vk::VkApplicationInfo::default();
        application_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_APPLICATION_INFO;
        application_info.apiVersion = vk::vkMakeVersion(1, 0, 21);
        application_info.applicationVersion = vk::vkMakeVersion(0, 1, 0);
        application_info.pApplicationName = application_name.as_ptr();
        application_info.pEngineName = engine_name.as_ptr();
        application_info.engineVersion = vk::vkMakeVersion(0, 1, 0);
        let vk_khr_surface_ext = CString::new("VK_KHR_surface").unwrap();
        #[cfg(target_os = "windows")]
        let vk_platform_surface_ext = CString::new("VK_KHR_win32_surface").unwrap();
        #[cfg(target_os = "linux")]
        let vk_platform_surface_ext = CString::new("VK_KHR_xcb_surface").unwrap();
        #[cfg(target_os = "android")]
        let vk_platform_surface_ext = CString::new("VK_KHR_android_surface").unwrap();
        #[cfg(debug_mode)]
        let vk_ext_debug_report_ext = CString::new("VK_EXT_debug_report").unwrap();
        let mut vulkan_extensions = Vec::new();
        vulkan_extensions.push(vk_khr_surface_ext.as_ptr());
        vulkan_extensions.push(vk_platform_surface_ext.as_ptr());
        #[cfg(debug_mode)]
        vulkan_extensions.push(vk_ext_debug_report_ext.as_ptr());
        let mut instance_create_info = vk::VkInstanceCreateInfo::default();
        instance_create_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;
        instance_create_info.pApplicationInfo = &application_info;
        let layers_names = debug::enumerate_layers();
        let vulkan_layers = cstrings_to_ptrs(&layers_names);
        instance_create_info.enabledLayerCount = vulkan_layers.len() as u32;
        instance_create_info.ppEnabledLayerNames = vulkan_layers.as_ptr();
        instance_create_info.enabledExtensionCount = vulkan_extensions.len() as u32;
        instance_create_info.ppEnabledExtensionNames = vulkan_extensions.as_ptr();
        let mut vk_instance = 0 as vk::VkInstance;
        vulkan_check!(vk::vkCreateInstance(
            &instance_create_info,
            null(),
            &mut vk_instance,
        ));
        Instance {
            vk_data: vk_instance,
            debugger: debug::Debugger::new(vk_instance),
        }
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        self.debugger.terminate(self.vk_data);
        unsafe {
            vk::vkDestroyInstance(self.vk_data, null());
        }
        self.vk_data = 0 as vk::VkInstance;
    }
}
