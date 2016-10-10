use ::system::vulkan::{
	VkApplicationInfo,
	VkStructureType,
	VkInstanceCreateFlags,
	VkInstanceCreateInfo,
	VkInstance,
	VkAllocationCallbacks,
	VkResult,

	vkMakeVersion,
	vkCreateInstance,
};

use std::ffi::{
	CString,
};
use std::os::raw::{
	c_void,
	c_char,
};

pub fn get_vulkan_layers() -> [*const c_char; 0] {
	[
		// CString::new("VK_LAYER_LUNARG_api_dump").unwrap().as_ptr(),
		// CString::new("VK_LAYER_LUNARG_core_validation").unwrap().as_ptr(),
		// CString::new("VK_LAYER_LUNARG_image").unwrap().as_ptr(),
		// CString::new("VK_LAYER_LUNARG_object_tracker").unwrap().as_ptr(),
		// CString::new("VK_LAYER_LUNARG_parameter_validation").unwrap().as_ptr(),
		// CString::new("VK_LAYER_LUNARG_screenshot").unwrap().as_ptr(),
		// CString::new("VK_LAYER_LUNARG_swapchain").unwrap().as_ptr(),
		// CString::new("VK_LAYER_GOOGLE_threading").unwrap().as_ptr(),
		// CString::new("VK_LAYER_GOOGLE_unique_objects").unwrap().as_ptr(),
		// CString::new("VK_LAYER_LUNARG_vktrace").unwrap().as_ptr(),
		// CString::new("VK_LAYER_RENDERDOC_Capture").unwrap().as_ptr(),
		// CString::new("VK_LAYER_NV_optimus").unwrap().as_ptr(),
		// CString::new("VK_LAYER_LUNARG_standard_validation").unwrap().as_ptr(),
	]
}

pub struct Instance {
	vk_instance: VkInstance,
}

macro_rules! vulkan_check {
    ( $x:expr ) => {
        unsafe {
            match $x {
                VkResult::VK_SUCCESS => {},
				VkResult::VK_NOT_READY => { println!("VK_NOT_READY in file: {:?}, line: {:?}", file!(), line!()) },
			    VkResult::VK_TIMEOUT => { println!("VK_TIMEOUT in file: {:?}, line: {:?}", file!(), line!()) },
			    VkResult::VK_EVENT_SET => { println!("VK_EVENT_SET in file: {:?}, line: {:?}", file!(), line!()) },
			    VkResult::VK_EVENT_RESET => { println!("VK_EVENT_RESET in file: {:?}, line: {:?}", file!(), line!()) },
			    VkResult::VK_INCOMPLETE => { println!("VK_INCOMPLETE in file: {:?}, line: {:?}", file!(), line!()) },
			    VkResult::VK_ERROR_OUT_OF_HOST_MEMORY => { println!("VK_ERROR_OUT_OF_HOST_MEMORY in file: {:?}, line: {:?}", file!(), line!()) },
			    VkResult::VK_ERROR_OUT_OF_DEVICE_MEMORY => { println!("VK_ERROR_OUT_OF_DEVICE_MEMORY in file: {:?}, line: {:?}", file!(), line!()) },
			    VkResult::VK_ERROR_INITIALIZATION_FAILED => { println!("VK_ERROR_INITIALIZATION_FAILED in file: {:?}, line: {:?}", file!(), line!()) },
			    VkResult::VK_ERROR_DEVICE_LOST => { println!("VK_ERROR_DEVICE_LOST in file: {:?}, line: {:?}", file!(), line!()) },
			    VkResult::VK_ERROR_MEMORY_MAP_FAILED => { println!("VK_ERROR_MEMORY_MAP_FAILED in file: {:?}, line: {:?}", file!(), line!()) },
			    VkResult::VK_ERROR_LAYER_NOT_PRESENT => { println!("VK_ERROR_LAYER_NOT_PRESENT in file: {:?}, line: {:?}", file!(), line!()) },
			    VkResult::VK_ERROR_EXTENSION_NOT_PRESENT => { println!("VK_ERROR_EXTENSION_NOT_PRESENT in file: {:?}, line: {:?}", file!(), line!()) },
			    VkResult::VK_ERROR_FEATURE_NOT_PRESENT => { println!("VK_ERROR_FEATURE_NOT_PRESENT in file: {:?}, line: {:?}", file!(), line!()) },
			    VkResult::VK_ERROR_INCOMPATIBLE_DRIVER => { println!("VK_ERROR_INCOMPATIBLE_DRIVER in file: {:?}, line: {:?}", file!(), line!()) },
			    VkResult::VK_ERROR_TOO_MANY_OBJECTS => { println!("VK_ERROR_TOO_MANY_OBJECTS in file: {:?}, line: {:?}", file!(), line!()) },
			    VkResult::VK_ERROR_FORMAT_NOT_SUPPORTED => { println!("VK_ERROR_FORMAT_NOT_SUPPORTED in file: {:?}, line: {:?}", file!(), line!()) },
			    VkResult::VK_ERROR_SURFACE_LOST_KHR => { println!("VK_ERROR_SURFACE_LOST_KHR in file: {:?}, line: {:?}", file!(), line!()) },
			    VkResult::VK_ERROR_NATIVE_WINDOW_IN_USE_KHR => { println!("VK_ERROR_NATIVE_WINDOW_IN_USE_KHR in file: {:?}, line: {:?}", file!(), line!()) },
			    VkResult::VK_SUBOPTIMAL_KHR => { println!("VK_SUBOPTIMAL_KHR in file: {:?}, line: {:?}", file!(), line!()) },
			    VkResult::VK_ERROR_OUT_OF_DATE_KHR => { println!("VK_ERROR_OUT_OF_DATE_KHR in file: {:?}, line: {:?}", file!(), line!()) },
			    VkResult::VK_ERROR_INCOMPATIBLE_DISPLAY_KHR => { println!("VK_ERROR_INCOMPATIBLE_DISPLAY_KHR in file: {:?}, line: {:?}", file!(), line!()) },
			    VkResult::VK_ERROR_VALIDATION_FAILED_EXT => { println!("VK_ERROR_VALIDATION_FAILED_EXT in file: {:?}, line: {:?}", file!(), line!()) },
			    VkResult::VK_ERROR_INVALID_SHADER_NV => { println!("VK_ERROR_INVALID_SHADER_NV in file: {:?}, line: {:?}", file!(), line!()) },
			    VkResult::VK_RESULT_RANGE_SIZE => { println!("VK_RESULT_RANGE_SIZE in file: {:?}, line: {:?}", file!(), line!()) },
			    VkResult::VK_RESULT_MAX_ENUM => { println!("VK_RESULT_MAX_ENUM in file: {:?}, line: {:?}", file!(), line!()) },
				// _ => { println!("Unknown error in file: {:?}, line: {:?}", file!(), line!()) },
            }
        }
    };
}

impl Instance {
	pub fn new() -> Self {
		let application_info = VkApplicationInfo {
			sType: VkStructureType::VK_STRUCTURE_TYPE_APPLICATION_INFO,
			apiVersion: vkMakeVersion(1, 0, 21),
			applicationVersion: vkMakeVersion(0, 1, 0),
			pApplicationName: CString::new("Vulkust App").unwrap().as_ptr(),
		    pEngineName: CString::new("Vulkus").unwrap().as_ptr(),
		    engineVersion: vkMakeVersion(0, 1, 0),
		    pNext: 0 as *const c_void,
		};
		let vk_khr_surface_ext = CString::new("VK_KHR_surface").unwrap();
		let vk_khr_win32_surface_ext = CString::new("VK_KHR_win32_surface").unwrap();
		let vk_ext_debug_report_ext = CString::new("VK_EXT_debug_report").unwrap();
		let vulkan_extentions = [
			vk_khr_surface_ext.as_ptr(),
			vk_khr_win32_surface_ext.as_ptr(),
			vk_ext_debug_report_ext.as_ptr(),
		];
		let instance_create_info = VkInstanceCreateInfo {
		    sType: VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
		    pNext: 0 as *const c_void,
		    flags: 0 as VkInstanceCreateFlags,
		    pApplicationInfo: &application_info,
		    enabledLayerCount: get_vulkan_layers().len() as u32,
		    ppEnabledLayerNames: get_vulkan_layers().as_ptr(),
		    enabledExtensionCount: vulkan_extentions.len() as u32,
		    ppEnabledExtensionNames: vulkan_extentions.as_ptr(),
		};
		let mut vk_instance = 0 as VkInstance;
		vulkan_check!(vkCreateInstance(&instance_create_info, 0 as *const VkAllocationCallbacks, &mut vk_instance));
		Instance {
			vk_instance: vk_instance,
		}
	}
}
