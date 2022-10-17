use crate::{
    core::string::{cstrings_to_ptrs, slice_to_string, strings_to_cstrings},
    platform::os::application::Application as OsApp,
};
use ash::vk;
use std::ffi::CString;

#[cfg(debug_mode)]
mod debug {
    use super::super::super::core::string::{slice_to_string, strings_to_cstrings};
    use ash::{extensions::ext::DebugUtils, vk};
    use std::{
        borrow::Cow,
        collections::BTreeMap,
        ffi::{CStr, CString},
    };

    extern "system" fn vulkan_debug_callback(
        message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
        message_type: vk::DebugUtilsMessageTypeFlagsEXT,
        callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
        _user_data: *mut std::os::raw::c_void,
    ) -> vk::Bool32 {
        let callback_data: &vk::DebugUtilsMessengerCallbackDataEXT = unsafe { &*callback_data };
        let message_id_number: i32 = callback_data.message_id_number as i32;
        let message_id_name = if callback_data.p_message_id_name.is_null() {
            Cow::from("")
        } else {
            unsafe { CStr::from_ptr(callback_data.p_message_id_name) }.to_string_lossy()
        };
        let message = if callback_data.p_message.is_null() {
            Cow::from("")
        } else {
            unsafe { CStr::from_ptr(callback_data.p_message) }.to_string_lossy()
        };
        if vx_flag_check!(
            message_severity,
            vk::DebugUtilsMessageSeverityFlagsEXT::INFO
        ) {
            vx_log_i!(
                "Vulkan Callback: {:?}:\n{:?} [{} ({})] : {}",
                message_severity,
                message_type,
                message_id_name,
                &message_id_number.to_string(),
                message,
            );
        }
        if vx_flag_check!(
            message_severity,
            vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
        ) {
            vx_log_e!(
                "Vulkan Callback: {:?}:\n{:?} [{} ({})] : {}",
                message_severity,
                message_type,
                message_id_name,
                &message_id_number.to_string(),
                message,
            );
        }
        if vx_flag_check!(
            message_severity,
            vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
        ) {
            vx_log_f!(
                "Vulkan Callback: {:?}:\n{:?} [{} ({})] : {}",
                message_severity,
                message_type,
                message_id_name,
                &message_id_number.to_string(),
                message,
            );
        }
        0
    }

    pub struct Debugger {
        loader: DebugUtils,
        vk_data: vk::DebugUtilsMessengerEXT,
    }

    impl Debugger {
        pub fn new(vk_entry: &ash::Entry, vk_instance: &ash::Instance) -> Self {
            let create_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
                .message_severity(
                    vk::DebugUtilsMessageSeverityFlagsEXT::INFO
                        | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                        | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
                )
                .message_type(
                    vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                        | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                        | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
                )
                .pfn_user_callback(Some(vulkan_debug_callback));
            let loader = DebugUtils::new(vk_entry, vk_instance);
            let vk_data = vx_result!(unsafe {
                loader.create_debug_utils_messenger(&create_info.build(), None)
            });
            Self { loader, vk_data }
        }

        pub fn terminate(&mut self) {
            unsafe {
                self.loader
                    .destroy_debug_utils_messenger(self.vk_data, None);
            }
        }
    }

    pub fn enumerate_layers(entry: &ash::Entry) -> Vec<CString> {
        let available_layers = vx_result!(entry.enumerate_instance_layer_properties());
        let mut found_layers = BTreeMap::new();
        for i in 0..available_layers.len() {
            let name = slice_to_string(&available_layers[i].layer_name);
            let des = slice_to_string(&available_layers[i].description);
            vx_log_i!("Layer {} with des: {} found.", name, des);
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
        insert_layer!("VK_LAYER_KHRONOS_validation");
        insert_layer!("MoltenVK");
        // insert_layer!("VK_LAYER_LUNARG_api_dump");
        insert_layer!("VK_LAYER_RENDERDOC_Capture");
        vx_log_i!("Layers that gonna be imported {:?}.", layers_names);
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

fn get_all_extensions(entry: &ash::Entry) -> Vec<vk::ExtensionProperties> {
    return vx_result!(entry.enumerate_instance_extension_properties(None));
}

fn enumerate_extensions(entry: &ash::Entry) -> Vec<String> {
    let properties = get_all_extensions(entry);
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
                vx_log_i!("Extension importing {}", name);
                extensions.push(name.to_string());
            }
            #[cfg(debug_mode)]
            "VK_EXT_debug_report" | "VK_EXT_debug_utils" => {
                vx_log_i!("Extension importing {}", name);
                extensions.push(name.to_string());
            }
            _ => {
                vx_log_i!("Extension '{}' found", name);
            }
        }
    }
    extensions
}

// fn contain_extension(s: &str, entry: &ash::Entry) -> bool {
//     let properties = get_all_extensions(entry);
//     for p in properties {
//         let name = slice_to_string(&p.extension_name);
//         if name == s {
//             return true;
//         }
//     }
//     return false;
// }

pub struct Instance {
    pub entry: ash::Entry,
    pub vk_data: ash::Instance,
    pub debugger: debug::Debugger,
}

impl Instance {
    pub(super) fn new(os_app: &OsApp) -> Self {
        let entry = ash::Entry::linked();
        let application_name = CString::new(&os_app.base.config.application_name as &str).unwrap();
        let engine_name = CString::new("Vulkust").unwrap();
        let application_info = vk::ApplicationInfo::builder()
            .api_version(vk::make_api_version(0, 1, 0, 0))
            .application_version(vk::make_api_version(0, 0, 1, 0))
            .application_name(&application_name)
            .engine_name(&engine_name)
            .engine_version(vk::make_api_version(0, 0, 1, 0));
        let vulkan_layers = debug::enumerate_layers(&entry);
        let vulkan_layers = cstrings_to_ptrs(&vulkan_layers);
        let vulkan_extensions = enumerate_extensions(&entry);
        let vulkan_extensions = strings_to_cstrings(vulkan_extensions);
        let vulkan_extensions = cstrings_to_ptrs(&vulkan_extensions);
        let instance_create_info = vk::InstanceCreateInfo::builder()
            .application_info(&application_info)
            .enabled_layer_names(&vulkan_layers)
            .enabled_extension_names(&vulkan_extensions);
        let vk_data = vx_result!(unsafe { entry.create_instance(&instance_create_info, None) });
        let debugger = debug::Debugger::new(&entry, &vk_data);
        Self {
            entry,
            vk_data,
            debugger,
        }
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        self.debugger.terminate();
        unsafe { self.vk_data.destroy_instance(None) };
    }
}
