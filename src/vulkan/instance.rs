use ::system::vulkan::{
	VkAllocationCallbacks,
	VkApplicationInfo,
	VkStructureType,
	VkInstanceCreateFlags,
	VkInstanceCreateInfo,
	VkDebugReportCallbackEXT,
	VkInstance,
	VkDebugReportFlagsEXT,
	VkDebugReportFlagBitsEXT,
	VkDebugReportObjectTypeEXT,
	VkDebugReportCallbackCreateInfoEXT,
	VkResult,

	vkMakeVersion,
	vkCreateInstance,
	vkDestroyInstance,
	vkGetInstanceProcAddr,
	PFN_vkVoidFunction,
	PFN_vkCreateDebugReportCallbackEXT,
	PFN_vkDestroyDebugReportCallbackEXT,
};

use std::fs::File;
use std::io::Write;
use std::ffi::{
	CString,
	CStr,
};
use std::os::raw::{
	c_void,
	c_char,
};
use std::mem::transmute;

pub struct Instance {
	debug_file: File,
	warning_file: File,
	performance_file: File,
	error_file: File,
	info_file: File,
	vk_instance: VkInstance,
	vk_debug_callback: VkDebugReportCallbackEXT,
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

const VULKAN_LOG_INFO_FILE_NAME: &'static str = "vulkust-info.txt";
const VULKAN_LOG_WARN_FILE_NAME: &'static str = "vulkust-warn.txt";
const VULKAN_LOG_PERF_FILE_NAME: &'static str = "vulkust-perf.txt";
const VULKAN_LOG_ERRO_FILE_NAME: &'static str = "vulkust-erro.txt";
const VULKAN_LOG_DEBG_FILE_NAME: &'static str = "vulkust-debg.txt";

unsafe extern fn vulkan_debug_callback(
	flags: VkDebugReportFlagsEXT,
	obj_type: VkDebugReportObjectTypeEXT,
	src_obj: u64, location: usize, msg_code: i32, layer_prefix: *const c_char,
	msg: *const c_char, user_data: *mut c_void) -> u32 {
	let instance = transmute::<*mut c_void, *mut Instance>(user_data);
	let file: &mut File =
		if flags & (VkDebugReportFlagBitsEXT::VK_DEBUG_REPORT_INFORMATION_BIT_EXT as u32) != 0 {
			&mut ((*instance).info_file)
		} else if flags & (VkDebugReportFlagBitsEXT::VK_DEBUG_REPORT_WARNING_BIT_EXT as u32) != 0 {
			&mut ((*instance).warning_file)
		} else if flags & (VkDebugReportFlagBitsEXT::VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT as u32) != 0 {
			&mut ((*instance).performance_file)
		} else if flags & (VkDebugReportFlagBitsEXT::VK_DEBUG_REPORT_ERROR_BIT_EXT as u32) != 0 {
			&mut ((*instance).error_file)
		} else if flags & (VkDebugReportFlagBitsEXT::VK_DEBUG_REPORT_DEBUG_BIT_EXT as u32) != 0 {
			&mut ((*instance).debug_file)
		} else {
			&mut ((*instance).debug_file)
		};
	let s = format!("obj_type: {:?}, src_obj: {:?}, location: {:?}, msg_code: {:?}, layer_prefix: {:?}, msg : {:?}, user_data {:?}\n",
		obj_type, src_obj, location, msg_code,
		CStr::from_ptr(layer_prefix).to_str(),
		CStr::from_ptr(msg).to_str(), user_data);
	if s.len() != file.write(s.as_bytes()).expect("Can not write to file.") {
		panic!("Can not write to file. Size is not correct.");
	}
	return 0;
}

fn init_debug_files() -> (File, File, File, File, File) {
	const INIT_STR: &'static str = "started\n";
	const ERROR_FILE_CREATION: &'static str = "Can not create log file";
	let mut info_file = File::create(VULKAN_LOG_INFO_FILE_NAME).expect(ERROR_FILE_CREATION);
	if INIT_STR.len() != info_file.write(INIT_STR.as_bytes()).expect(ERROR_FILE_CREATION) {
		panic!("{}", ERROR_FILE_CREATION);
	}
	let mut warning_file = File::create(VULKAN_LOG_WARN_FILE_NAME).expect(ERROR_FILE_CREATION);
	if INIT_STR.len() != warning_file.write(INIT_STR.as_bytes()).expect(ERROR_FILE_CREATION) {
		panic!("{}", ERROR_FILE_CREATION);
	}
	let mut performance_file = File::create(VULKAN_LOG_PERF_FILE_NAME).expect(ERROR_FILE_CREATION);
	if INIT_STR.len() != performance_file.write(INIT_STR.as_bytes()).expect(ERROR_FILE_CREATION) {
		panic!("{}", ERROR_FILE_CREATION);
	}
	let mut error_file = File::create(VULKAN_LOG_ERRO_FILE_NAME).expect(ERROR_FILE_CREATION);
	if INIT_STR.len() != error_file.write(INIT_STR.as_bytes()).expect(ERROR_FILE_CREATION) {
		panic!("{}", ERROR_FILE_CREATION);
	}
	let mut debug_file = File::create(VULKAN_LOG_DEBG_FILE_NAME).expect(ERROR_FILE_CREATION);
	if INIT_STR.len() != debug_file.write(INIT_STR.as_bytes()).expect(ERROR_FILE_CREATION) {
		panic!("{}", ERROR_FILE_CREATION);
	}
	return (info_file, warning_file, performance_file, error_file, debug_file);
}

impl Instance {
	pub fn new() -> Self {
		let application_name = CString::new("Vulkust App").unwrap();
		let engine_name = CString::new("Vulkuts").unwrap();
		let application_info = VkApplicationInfo {
			sType: VkStructureType::VK_STRUCTURE_TYPE_APPLICATION_INFO,
			apiVersion: vkMakeVersion(1, 0, 21),
			applicationVersion: vkMakeVersion(0, 1, 0),
			pApplicationName: application_name.as_ptr(),
		    pEngineName: engine_name.as_ptr(),
		    engineVersion: vkMakeVersion(0, 1, 0),
		    pNext: 0 as *const c_void,
		};
		let vk_ly_api_dup = CString::new("VK_LAYER_LUNARG_api_dump").unwrap();
		let vk_ly_core_val = CString::new("VK_LAYER_LUNARG_core_validation").unwrap();
		let vk_ly_image = CString::new("VK_LAYER_LUNARG_image").unwrap();
		let vk_ly_obj_trk = CString::new("VK_LAYER_LUNARG_object_tracker").unwrap();
		let vk_ly_par_val = CString::new("VK_LAYER_LUNARG_parameter_validation").unwrap();
		let vk_ly_scrsh = CString::new("VK_LAYER_LUNARG_screenshot").unwrap();
		let vk_ly_swap = CString::new("VK_LAYER_LUNARG_swapchain").unwrap();
		let vk_ly_thrd = CString::new("VK_LAYER_GOOGLE_threading").unwrap();
		let vk_ly_uniq = CString::new("VK_LAYER_GOOGLE_unique_objects").unwrap();
		let vk_ly_ren_doc = CString::new("VK_LAYER_RENDERDOC_Capture").unwrap();
		let vk_ly_optimus = CString::new("VK_LAYER_NV_optimus").unwrap();
		let vk_ly_std_val = CString::new("VK_LAYER_LUNARG_standard_validation").unwrap();
		let vulkan_layers = [
			vk_ly_api_dup.as_ptr(),
			vk_ly_core_val.as_ptr(),
			vk_ly_image.as_ptr(),
			vk_ly_obj_trk.as_ptr(),
			vk_ly_par_val.as_ptr(),
			vk_ly_scrsh.as_ptr(),
			vk_ly_swap.as_ptr(),
			vk_ly_thrd.as_ptr(),
			vk_ly_uniq.as_ptr(),
			vk_ly_ren_doc.as_ptr(),
			vk_ly_optimus.as_ptr(),
			vk_ly_std_val.as_ptr(),
		];
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
		    enabledLayerCount: vulkan_layers.len() as u32,
		    ppEnabledLayerNames: vulkan_layers.as_ptr(),
		    enabledExtensionCount: vulkan_extentions.len() as u32,
		    ppEnabledExtensionNames: vulkan_extentions.as_ptr(),
		};
		let mut vk_instance = 0 as VkInstance;
		vulkan_check!(vkCreateInstance(&instance_create_info, 0 as *const VkAllocationCallbacks, &mut vk_instance));
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
		let mut vk_debug_callback = 0 as VkDebugReportCallbackEXT;
		let vk_proc_name = CString::new("vkCreateDebugReportCallbackEXT").unwrap();
		let vk_create_debug_report_callback_ext = unsafe {
			transmute::<PFN_vkVoidFunction, PFN_vkCreateDebugReportCallbackEXT>(
				vkGetInstanceProcAddr(vk_instance, vk_proc_name.as_ptr())) };
		if vk_create_debug_report_callback_ext == unsafe {
			transmute::<usize, PFN_vkCreateDebugReportCallbackEXT>(0) } {
			panic!("Error in finding vkCreateDebugReportCallbackEXT process location.");
		}
		let (debug_file, warning_file, performance_file, error_file, info_file) = init_debug_files();
		let mut instance = Instance {
			debug_file: debug_file,
			warning_file: warning_file,
			performance_file: performance_file,
			error_file: error_file,
			info_file: info_file,
			vk_instance: vk_instance,
			vk_debug_callback: vk_debug_callback,
		};
		report_callback_create_info.pUserData = unsafe { transmute::<&mut Instance, *mut c_void>(&mut instance) };
		vulkan_check!(vk_create_debug_report_callback_ext(vk_instance, &report_callback_create_info, 0 as *const VkAllocationCallbacks, &mut vk_debug_callback));
		return instance;
	}
}

impl Drop for Instance {
    fn drop(&mut self) {
		unsafe {
			let vk_proc_name = CString::new("vkDestroyDebugReportCallbackEXT").unwrap();
			let vk_destroy_debug_report_callback_ext =
				transmute::<PFN_vkVoidFunction, PFN_vkDestroyDebugReportCallbackEXT>(
					vkGetInstanceProcAddr(self.vk_instance, vk_proc_name.as_ptr()));
			if vk_destroy_debug_report_callback_ext == transmute::<usize, PFN_vkDestroyDebugReportCallbackEXT>(0) {
				panic!("Error in finding vkDestroyDebugReportCallbackEXT process location.");
			}
			vk_destroy_debug_report_callback_ext(self.vk_instance, self.vk_debug_callback, 0 as *const VkAllocationCallbacks);
			vkDestroyInstance(self.vk_instance, 0 as *const VkAllocationCallbacks);
		}
    }
}
