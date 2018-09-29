use std::default::Default;
use std::ffi::CString;
use std::ptr::{null, null_mut};

use super::super::core::string::{cstrings_to_ptrs, slice_to_string, strings_to_cstrings};
use super::vulkan as vk;

#[cfg(debug_mode)]
mod debug {
    use std::collections::BTreeMap;
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

    extern "C" fn vulkan_debug_callback(
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
            unsafe { CStr::from_ptr(layer_prefix).to_str() },
            unsafe { CStr::from_ptr(msg).to_str() },
            user_data
        );
        0u32
    }

    #[derive(Debug)]
    pub struct Debugger {
        vk_data: Option<vk::VkDebugReportCallbackEXT>,
    }

    impl Debugger {
        pub fn new(vk_instance: vk::VkInstance) -> Self {
            if !super::contain_extension("VK_EXT_debug_report") {
                return Debugger { vk_data: None };
            }
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
                vk_data: Some(vk_debug_callback),
                // vk_data: None,
            }
        }

        pub fn terminate(&mut self, vk_instance: vk::VkInstance) {
            if self.vk_data.is_none() {
                return;
            }
            let destroy_debug_report_callback: vk::PFN_vkDestroyDebugReportCallbackEXT =
                unsafe { transmute(get_function(vk_instance, "vkDestroyDebugReportCallbackEXT")) };
            unsafe {
                destroy_debug_report_callback(vk_instance, *vxunwrap!(&self.vk_data), null());
            }
            self.vk_data = None;
        }
    }

    impl Drop for Debugger {
        fn drop(&mut self) {
            if self.vk_data.is_some() {
                vxlogf!("Unexpected drop of debugger.");
            }
        }
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
        let mut found_layers = BTreeMap::new();
        for i in 0..available_layers.len() {
            let name = slice_to_string(&available_layers[i].layerName);
            let des = slice_to_string(&available_layers[i].description);
            vxlogi!("Layer {} with des: {} found.", name, des);
            found_layers.insert(name, true);
        }
        let mut layers_names = Vec::new();
        let layer_name = "VK_LAYER_GOOGLE_threading".to_string();
        if found_layers.contains_key(&layer_name) {
            layers_names.push(layer_name);
        }
        let layer_name = "MoltenVK".to_string();
        if found_layers.contains_key(&layer_name) {
            layers_names.push(layer_name);
        }
        // let layer_name = "VK_LAYER_LUNARG_api_dump".to_string();
        // if found_layers.contains_key(&layer_name) {
        //     layers_names.push(layer_name);
        // }
        let layer_name = "VK_LAYER_LUNARG_parameter_validation".to_string();
        if found_layers.contains_key(&layer_name) {
            layers_names.push(layer_name);
        }
        let layer_name = "VK_LAYER_LUNARG_object_tracker".to_string();
        if found_layers.contains_key(&layer_name) {
            layers_names.push(layer_name);
        }
        let layer_name = "VK_LAYER_LUNARG_core_validation".to_string();
        if found_layers.contains_key(&layer_name) {
            layers_names.push(layer_name);
        }
        let layer_name = "VK_LAYER_LUNARG_image".to_string();
        if found_layers.contains_key(&layer_name) {
            layers_names.push(layer_name);
        }
        let layer_name = "VK_LAYER_LUNARG_swapchain".to_string();
        if found_layers.contains_key(&layer_name) {
            layers_names.push(layer_name);
        }
        let layer_name = "VK_LAYER_GOOGLE_unique_objects".to_string();
        if found_layers.contains_key(&layer_name) {
            layers_names.push(layer_name);
        }
        vxlogi!("Layers that gonna be imported {:?}.", layers_names);
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

fn get_all_extensions() -> Vec<vk::VkExtensionProperties> {
    let mut count = 0u32;
    unsafe {
        vulkan_check!(vk::vkEnumerateInstanceExtensionProperties(
            null(),
            &mut count,
            null_mut()
        ));
    }
    let mut properties = vec![vk::VkExtensionProperties::default(); count as usize];
    unsafe {
        vulkan_check!(vk::vkEnumerateInstanceExtensionProperties(
            null(),
            &mut count,
            properties.as_mut_ptr()
        ));
    }
    properties
}

fn enumerate_extensions() -> Vec<String> {
    let properties = get_all_extensions();
    let mut extensions = Vec::new();
    for p in properties {
        let name = slice_to_string(&p.extensionName);
        let name: &str = &name;
        match name {
            "VK_KHR_surface"
            | "VK_KHR_win32_surface"
            | "VK_KHR_xcb_surface"
            | "VK_KHR_android_surface"
            | "VK_MVK_macos_surface"
            | "VK_MVK_ios_surface"
            | "VK_MVK_moltenvk" => {
                vxlogi!("Extension importing {}", name);
                extensions.push(name.to_string());
            }
            #[cfg(debug_mode)]
            "VK_EXT_debug_report" => {
                vxlogi!("Extension importing {}", name);
                extensions.push(name.to_string());
            }
            _ => {
                vxlogi!("Extension '{}' found", name);
            }
        }
    }
    extensions
}

pub fn contain_extension(s: &str) -> bool {
    let properties = get_all_extensions();
    for p in properties {
        let name = slice_to_string(&p.extensionName);
        if name == s {
            return true;
        }
    }
    return false;
}

#[cfg_attr(debug_mode, derive(Debug))]
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
        application_info.apiVersion = vk::vkMakeVersion(1, 0, 0);
        application_info.applicationVersion = vk::vkMakeVersion(0, 1, 0);
        application_info.pApplicationName = application_name.as_ptr();
        application_info.pEngineName = engine_name.as_ptr();
        application_info.engineVersion = vk::vkMakeVersion(0, 1, 0);
        let vulkan_layers = debug::enumerate_layers();
        let vulkan_layers = cstrings_to_ptrs(&vulkan_layers);
        let vulkan_extensions = enumerate_extensions();
        let vulkan_extensions = strings_to_cstrings(vulkan_extensions);
        let vulkan_extensions = cstrings_to_ptrs(&vulkan_extensions);
        let mut instance_create_info = vk::VkInstanceCreateInfo::default();
        instance_create_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;
        instance_create_info.pApplicationInfo = &application_info;
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
