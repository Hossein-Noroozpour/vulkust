use std::ffi::{
    CString,
};
use std::default::Default;
use std::mem::zeroed;
#[cfg(all(not(target_os = "android"), debug_assertions))]
use std::mem::transmute;

use super::super::system::vulkan::{
    VkResult,
    VkInstance,
    vkMakeVersion,
    VkStructureType,
    vkCreateInstance,
    VkApplicationInfo,
    vkDestroyInstance,
    VkInstanceCreateInfo,
    VkAllocationCallbacks,
};

#[cfg(debug_assertions)]
use super::super::system::vulkan::{
    VkLayerProperties,
    vkEnumerateInstanceLayerProperties,
};

#[cfg(all(not(target_os = "android"), debug_assertions))]
use super::super::system::vulkan::{
    VkDebugReportFlagsEXT,
    VkInstanceCreateFlags,
    VkAllocationCallbacks,
    VkDebugReportCallbackEXT,
    VkDebugReportFlagBitsEXT,
    VkDebugReportObjectTypeEXT,
    vkEnumerateInstanceLayerProperties,
    VkDebugReportCallbackCreateInfoEXT,
    PFN_vkCreateDebugReportCallbackEXT,
    PFN_vkDestroyDebugReportCallbackEXT,
};

#[cfg(debug_assertions)]
use super::super::util::string::{
    slice_to_string,
    strings_to_cstrings,
    cstrings_to_ptrs,
};

pub struct Instance {
    pub vk_instance: VkInstance,
    #[cfg(all(not(target_os = "android"), debug_assertions))]
    vk_debug_callback: VkDebugReportCallbackEXT,
}

impl Default for Instance {
    fn default() -> Self {
        unsafe {
            zeroed()
        }
    }
}

#[cfg(all(not(target_os = "android"), debug_assertions))]
unsafe extern fn vulkan_debug_callback(
    flags: VkDebugReportFlagsEXT,
    obj_type: VkDebugReportObjectTypeEXT,
    src_obj: u64, location: usize, msg_code: i32, layer_prefix: *const c_char,
    msg: *const c_char, user_data: *mut c_void) -> u32 {
    let mut flg = String::new();
    if flags & (VkDebugReportFlagBitsEXT::VK_DEBUG_REPORT_INFORMATION_BIT_EXT as u32) != 0 {
        flg += "info, ";
    }
    if flags & (VkDebugReportFlagBitsEXT::VK_DEBUG_REPORT_WARNING_BIT_EXT as u32) != 0 {
        flg += "warn, ";
    }
    if flags & (VkDebugReportFlagBitsEXT::VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT as u32) != 0 {
        flg += "performance, ";
    }
    if flags & (VkDebugReportFlagBitsEXT::VK_DEBUG_REPORT_ERROR_BIT_EXT as u32) != 0 {
        flg += "error, ";
    }
    if flags & (VkDebugReportFlagBitsEXT::VK_DEBUG_REPORT_DEBUG_BIT_EXT as u32) != 0 {
        flg += "debug, ";
    }
    println!("flag: {}, obj_type: {:?}, src_obj: {:?}, location: {:?}, msg_code: {:?}, \
        layer_prefix: {:?}, msg : {:?}, user_data {:?}",
             flg, obj_type, src_obj, location, msg_code,
             CStr::from_ptr(layer_prefix).to_str(),
             CStr::from_ptr(msg).to_str(), user_data);
    return 0u32
}

impl Instance {
    pub fn new() -> Self {
        let application_name = CString::new("Vulkust App").unwrap();
        let engine_name = CString::new("Vulkuts").unwrap();
        let mut application_info = VkApplicationInfo::default();
        application_info.sType = VkStructureType::VK_STRUCTURE_TYPE_APPLICATION_INFO;
        application_info.apiVersion = vkMakeVersion(1, 0, 21);
        application_info.applicationVersion = vkMakeVersion(0, 1, 0);
        application_info.pApplicationName = application_name.as_ptr();
        application_info.pEngineName = engine_name.as_ptr();
        application_info.engineVersion = vkMakeVersion(0, 1, 0);
        let vk_khr_surface_ext = CString::new("VK_KHR_surface").unwrap();
        #[cfg(target_os = "windows")]
        let vk_platform_surface_ext = CString::new("VK_KHR_win32_surface").unwrap();
        #[cfg(target_os = "linux")]
        let vk_platform_surface_ext = CString::new("VK_KHR_xcb_surface").unwrap();
        #[cfg(target_os = "android")]
        let vk_platform_surface_ext = CString::new("VK_KHR_android_surface").unwrap();
        #[cfg(all(not(target_os = "android"), debug_assertions))]
        let vk_ext_debug_report_ext = CString::new("VK_EXT_debug_report").unwrap();
        let mut vulkan_extensions = Vec::new();
        vulkan_extensions.push(vk_khr_surface_ext.as_ptr());
        vulkan_extensions.push(vk_platform_surface_ext.as_ptr());
        #[cfg(all(not(target_os = "android"), debug_assertions))]
        vulkan_extensions.push(vk_ext_debug_report_ext.as_ptr());
        let mut instance_create_info = VkInstanceCreateInfo::default();
        instance_create_info.sType = VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;
        instance_create_info.pApplicationInfo = &application_info;
        #[cfg(debug_assertions)]
        {
            let layers_names = Instance::enumerate_layers();
            let vulkan_layers = cstrings_to_ptrs(&layers_names);
            instance_create_info.enabledLayerCount = vulkan_layers.len() as u32;
            instance_create_info.ppEnabledLayerNames = if vulkan_layers.len() != 0 {
                vulkan_layers.as_ptr() } else { 0 as *const *const u8 };
        }
        instance_create_info.enabledExtensionCount = vulkan_extensions.len() as u32;
        instance_create_info.ppEnabledExtensionNames = vulkan_extensions.as_ptr();
        let mut vk_instance = 0 as VkInstance;
        vulkan_check!(vkCreateInstance( & instance_create_info, 0 as *const VkAllocationCallbacks, & mut vk_instance));
        let mut instance = Instance::default();
        instance.vk_instance = vk_instance;
        #[cfg(all(not(target_os = "android"), debug_assertions))]
        instance.set_report_callback();
        return instance;
    }

    #[cfg(debug_assertions)]
    fn enumerate_layers() -> Vec<CString> {
        let mut layer_count = 0u32;
        unsafe {
            vkEnumerateInstanceLayerProperties(&mut layer_count, 0 as *mut VkLayerProperties);
        }
        logdbg!(format!("Number of layers found is: {}", layer_count));
        let mut available_layers = vec![VkLayerProperties::default(); layer_count as usize];
        unsafe {
            vkEnumerateInstanceLayerProperties(&mut layer_count, available_layers.as_mut_ptr());
        }
        let mut layers_names = Vec::new();
        for i in 0..available_layers.len() {
            let name = slice_to_string(&available_layers[i].layerName);
            let des = slice_to_string(&available_layers[i].description);
            logdbg!(format!("Layer {} with des: {} found.", name, des));
            layers_names.push(name);
        }
        strings_to_cstrings(layers_names)
    }

    #[cfg(all(not(target_os = "android"), debug_assertions))]
    fn set_report_callback(&mut self) {
        let mut report_callback_create_info = VkDebugReportCallbackCreateInfoEXT {
            sType: VkStructureType::VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT,
            pNext: 0 as *const c_void,
            flags: (VkDebugReportFlagBitsEXT::VK_DEBUG_REPORT_INFORMATION_BIT_EXT as u32)
                | (VkDebugReportFlagBitsEXT::VK_DEBUG_REPORT_WARNING_BIT_EXT as u32)
                | (VkDebugReportFlagBitsEXT::VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT as u32)
                | (VkDebugReportFlagBitsEXT::VK_DEBUG_REPORT_ERROR_BIT_EXT as u32)
                | (VkDebugReportFlagBitsEXT::VK_DEBUG_REPORT_DEBUG_BIT_EXT as u32)
                as VkDebugReportFlagsEXT,
            pfnCallback: vulkan_debug_callback,
            pUserData: 0 as *mut c_void,
        };
        let vk_proc_name = CString::new("vkCreateDebugReportCallbackEXT").unwrap();
        let vk_create_debug_report_callback_ext = unsafe {
            transmute::<PFN_vkVoidFunction, PFN_vkCreateDebugReportCallbackEXT>(
                vkGetInstanceProcAddr(vk_instance, vk_proc_name.as_ptr()))
        };
        if vk_create_debug_report_callback_ext == unsafe {
            transmute::<usize, PFN_vkCreateDebugReportCallbackEXT>(0)
        } {
            logftl!("Error in finding vkCreateDebugReportCallbackEXT process location.");
        }
        report_callback_create_info.pUserData = unsafe { transmute::<&mut Instance, *mut c_void>(&mut instance) };
        vulkan_check!(vk_create_debug_report_callback_ext(vk_instance, & report_callback_create_info, 0 as * const VkAllocationCallbacks, & mut instance.vk_debug_callback));
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            #[cfg(all(not(target_os = "android"), debug_assertions))]
            {
                let vk_proc_name = CString::new("vkDestroyDebugReportCallbackEXT").unwrap();
                let vk_destroy_debug_report_callback_ext =
                transmute::<PFN_vkVoidFunction, PFN_vkDestroyDebugReportCallbackEXT>(
                    vkGetInstanceProcAddr(self.vk_instance, vk_proc_name.as_ptr()));
                if vk_destroy_debug_report_callback_ext ==
                    transmute::<usize, PFN_vkDestroyDebugReportCallbackEXT>(0) {
                    logftl!("Error in finding vkDestroyDebugReportCallbackEXT process location.");
                }
                vk_destroy_debug_report_callback_ext(
                    self.vk_instance, self.vk_debug_callback, 0 as *const VkAllocationCallbacks);
            }
            logerr!("Instance is deleted now!");
            vkDestroyInstance(self.vk_instance, 0 as *const VkAllocationCallbacks);
        }
    }
}
