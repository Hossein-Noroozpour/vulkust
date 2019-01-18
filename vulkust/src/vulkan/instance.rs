use std::ffi::CString;
use std::ptr::{null, null_mut};

use super::super::core::string::{cstrings_to_ptrs, slice_to_string, strings_to_cstrings};
use ash::version::EntryV1_0;
use ash::version::InstanceV1_0;
use ash::vk;

#[cfg(debug_mode)]
mod debug {
    use std::collections::BTreeMap;
    use std::ffi::{CStr, CString};
    use std::os::raw::{c_char, c_void};
    use std::ptr::null_mut;

    use super::vk;
    use ash::extensions::ext::DebugReport;
    use ash::version::EntryV1_0;

    use super::super::super::core::string::{slice_to_string, strings_to_cstrings};

    extern "system" fn vulkan_debug_callback(
        flags: vk::DebugReportFlagsEXT,
        obj_type: vk::DebugReportObjectTypeEXT,
        src_obj: u64,
        location: usize,
        msg_code: i32,
        layer_prefix: *const c_char,
        msg: *const c_char,
        user_data: *mut c_void,
    ) -> u32 {
        let mut flg = String::new();
        if vxflagcheck!(flags, vk::DebugReportFlagsEXT::INFORMATION) {
            flg += "info, ";
        }
        if vxflagcheck!(flags, vk::DebugReportFlagsEXT::WARNING) {
            flg += "warn, ";
        }
        if vxflagcheck!(flags, vk::DebugReportFlagsEXT::PERFORMANCE_WARNING) {
            flg += "performance, ";
        }
        if vxflagcheck!(flags, vk::DebugReportFlagsEXT::ERROR) {
            flg += "error, ";
        }
        if vxflagcheck!(flags, vk::DebugReportFlagsEXT::DEBUG) {
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

    pub struct Debugger {
        loader: DebugReport,
        vk_data: vk::DebugReportCallbackEXT,
    }

    impl Debugger {
        pub fn new(vk_entry: &ash::Entry, vk_instance: &ash::Instance) -> Self {
            let create_info = vk::DebugReportCallbackCreateInfoEXT::builder()
                .flags(
                    vk::DebugReportFlagsEXT::INFORMATION
                        | vk::DebugReportFlagsEXT::WARNING
                        | vk::DebugReportFlagsEXT::PERFORMANCE_WARNING
                        | vk::DebugReportFlagsEXT::ERROR
                        | vk::DebugReportFlagsEXT::DEBUG,
                )
                .pfn_callback(Some(vulkan_debug_callback));
            let loader = DebugReport::new(vk_entry, vk_instance);
            let vk_data = vxunwrap!(loader.create_debug_report_callback(&create_info, None));
            Self { loader, vk_data }
        }

        pub fn terminate(&mut self) {
            self.loader
                .destroy_debug_report_callback(self.vk_data, None);
        }
    }

    pub fn enumerate_layers(entry: &vk::EntryFnV1_0) -> Vec<CString> {
        let mut layer_count = 0u32;
        unsafe {
            entry.enumerate_instance_layer_properties(&mut layer_count, null_mut());
        }
        vxlogi!("Number of layers found is: {}", layer_count);
        let mut available_layers = vec![vk::LayerProperties::default(); layer_count as usize];
        unsafe {
            entry.enumerate_instance_layer_properties(
                &mut layer_count,
                available_layers.as_mut_ptr(),
            );
        }
        let mut found_layers = BTreeMap::new();
        for i in 0..available_layers.len() {
            let name = slice_to_string(&available_layers[i].layer_name);
            let des = slice_to_string(&available_layers[i].description);
            vxlogi!("Layer {} with des: {} found.", name, des);
            found_layers.insert(name, true);
        }
        let mut layers_names = Vec::new();
        macro_rules! insert_layer {
            ($e:expr) => {
                let layer_name = $e.to_string();
                if found_layers.contains_key(&layer_name) {
                    layers_names.push(layer_name);
                }
            };
        }
        insert_layer!("VK_LAYER_GOOGLE_threading");
        insert_layer!("MoltenVK");
        // insert_layer!("VK_LAYER_LUNARG_api_dump");
        insert_layer!("VK_LAYER_LUNARG_parameter_validation");
        insert_layer!("VK_LAYER_LUNARG_object_tracker");
        insert_layer!("VK_LAYER_LUNARG_core_validation");
        insert_layer!("VK_LAYER_LUNARG_image");
        insert_layer!("VK_LAYER_LUNARG_swapchain");
        insert_layer!("VK_LAYER_RENDERDOC_Capture");
        insert_layer!("VK_LAYER_GOOGLE_unique_objects");
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
        pub fn new(_vk_entry: &ash::Entry, _vk_instance: &ash::Instance) -> Self {
            Self {}
        }

        pub fn terminate(&mut self) {}
    }

    pub fn enumerate_layers() -> Vec<CString> {
        Vec::new()
    }
}

fn get_all_extensions() -> Vec<vk::ExtensionProperties> {
    let mut count = 0u32;
    unsafe {
        vulkan_check!(vk::vkEnumerateInstanceExtensionProperties(
            null(),
            &mut count,
            null_mut()
        ));
    }
    let mut properties = vec![vk::ExtensionProperties::default(); count as usize];
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
        let name = slice_to_string(&p.extension_name);
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

fn contain_extension(s: &str) -> bool {
    let properties = get_all_extensions();
    for p in properties {
        let name = slice_to_string(&p.extension_name);
        if name == s {
            return true;
        }
    }
    return false;
}

pub(crate) struct Instance {
    entry: ash::Entry,
    vk_data: ash::Instance,
    debugger: debug::Debugger,
}

impl Instance {
    pub(super) fn new(application_name: &str) -> Self {
        let entry = ash::Entry::new().unwrap();
        let application_name = CString::new(application_name).unwrap();
        let engine_name = CString::new("Vulkust").unwrap();
        let application_info = vk::ApplicationInfo::builder()
            .api_version(vk_make_version!(1, 0, 0))
            .application_version(vk_make_version!(0, 1, 0))
            .application_name(&application_name)
            .engine_name(&engine_name)
            .engine_version(vk_make_version!(0, 1, 0));
        let vulkan_layers = debug::enumerate_layers(entry.fp_v1_0());
        let vulkan_layers = cstrings_to_ptrs(&vulkan_layers);
        let vulkan_extensions = enumerate_extensions();
        let vulkan_extensions = strings_to_cstrings(vulkan_extensions);
        let vulkan_extensions = cstrings_to_ptrs(&vulkan_extensions);
        let instance_create_info = vk::InstanceCreateInfo::builder()
            .application_info(&application_info)
            .enabled_layer_names(&vulkan_layers)
            .enabled_extension_names(&vulkan_extensions);
        let vk_data = vxresult!(entry.create_instance(&instance_create_info, None));
        let debugger = debug::Debugger::new(&entry, &vk_data);
        Self {
            entry,
            vk_data,
            debugger,
        }
    }

    #[inline]
    pub(super) fn get_data(&self) -> &ash::Instance {
        return &self.vk_data;
    }

    #[inline]
    pub(super) fn get_entry(&self) -> &ash::Entry {
        return &self.entry;
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        self.debugger.terminate();
        self.vk_data.destroy_instance(None);
    }
}

#[cfg(debug_mode)]
impl std::fmt::Debug for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Vulkan instance")
    }
}

unsafe impl Send for Instance {}

unsafe impl Sync for Instance {}
