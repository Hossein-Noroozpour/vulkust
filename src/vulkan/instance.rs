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

//	vulkan_check,
};

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
	pub vk_instance: VkInstance,
	vk_debug_callback: VkDebugReportCallbackEXT,
}

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
		let application_info = VkApplicationInfo {
			sType: VkStructureType::VK_STRUCTURE_TYPE_APPLICATION_INFO,
			apiVersion: vkMakeVersion(1, 0, 21),
			applicationVersion: vkMakeVersion(0, 1, 0),
			pApplicationName: application_name.as_ptr(),
		    pEngineName: engine_name.as_ptr(),
		    engineVersion: vkMakeVersion(0, 1, 0),
		    pNext: 0 as *const c_void,
		};
		#[cfg(target_os = "windows")]
		let vk_ly_api_dup = CString::new("VK_LAYER_LUNARG_api_dump").unwrap();
		let vk_ly_core_val = CString::new("VK_LAYER_LUNARG_core_validation").unwrap();
		let vk_ly_image = CString::new("VK_LAYER_LUNARG_image").unwrap();
		let vk_ly_obj_trk = CString::new("VK_LAYER_LUNARG_object_tracker").unwrap();
		let vk_ly_par_val = CString::new("VK_LAYER_LUNARG_parameter_validation").unwrap();
		#[cfg(target_os = "windows")]
		let vk_ly_scrsh = CString::new("VK_LAYER_LUNARG_screenshot").unwrap();
		let vk_ly_swap = CString::new("VK_LAYER_LUNARG_swapchain").unwrap();
		let vk_ly_thrd = CString::new("VK_LAYER_GOOGLE_threading").unwrap();
		let vk_ly_uniq = CString::new("VK_LAYER_GOOGLE_unique_objects").unwrap();
		#[cfg(target_os = "windows")]
		let vk_ly_ren_doc = CString::new("VK_LAYER_RENDERDOC_Capture").unwrap();
		#[cfg(target_os = "windows")]
		let vk_ly_optimus = CString::new("VK_LAYER_NV_optimus").unwrap();
		#[cfg(target_os = "windows")]
		let vk_ly_std_val = CString::new("VK_LAYER_LUNARG_standard_validation").unwrap();
		let vulkan_layers = [
			#[cfg(target_os = "windows")]
			vk_ly_api_dup.as_ptr(),
			vk_ly_core_val.as_ptr(),
			vk_ly_image.as_ptr(),
			vk_ly_obj_trk.as_ptr(),
			vk_ly_par_val.as_ptr(),
			#[cfg(target_os = "windows")]
			vk_ly_scrsh.as_ptr(),
			vk_ly_swap.as_ptr(),
			vk_ly_thrd.as_ptr(),
			vk_ly_uniq.as_ptr(),
			#[cfg(target_os = "windows")]
			vk_ly_ren_doc.as_ptr(),
			#[cfg(target_os = "windows")]
			vk_ly_optimus.as_ptr(),
			#[cfg(target_os = "windows")]
			vk_ly_std_val.as_ptr(),
		];
		let vk_khr_surface_ext = CString::new("VK_KHR_surface").unwrap();
		#[cfg(target_os = "windows")]
		let vk_khr_win32_surface_ext = CString::new("VK_KHR_win32_surface").unwrap();
		#[cfg(target_os = "linux")]
		let vk_khr_xcb_surface_ext = CString::new("VK_KHR_xcb_surface").unwrap();
		#[cfg(target_os = "linux")]
		let vk_khr_xlib_surface_ext = CString::new("VK_KHR_xlib_surface").unwrap();
		let vk_ext_debug_report_ext = CString::new("VK_EXT_debug_report").unwrap();
		let vulkan_extentions = [
			vk_khr_surface_ext.as_ptr(),
			#[cfg(target_os = "windows")]
			vk_khr_win32_surface_ext.as_ptr(),
			#[cfg(target_os = "linux")]
			vk_khr_xcb_surface_ext.as_ptr(),
			#[cfg(target_os = "linux")]
			vk_khr_xlib_surface_ext.as_ptr(),
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
		let vk_proc_name = CString::new("vkCreateDebugReportCallbackEXT").unwrap();
		let vk_create_debug_report_callback_ext = unsafe {
			transmute::<PFN_vkVoidFunction, PFN_vkCreateDebugReportCallbackEXT>(
				vkGetInstanceProcAddr(vk_instance, vk_proc_name.as_ptr())) };
		if vk_create_debug_report_callback_ext == unsafe {
			transmute::<usize, PFN_vkCreateDebugReportCallbackEXT>(0) } {
			panic!("Error in finding vkCreateDebugReportCallbackEXT process location.");
		}
		let mut instance = Instance {
			vk_instance: vk_instance,
			vk_debug_callback: 0 as VkDebugReportCallbackEXT,
		};
		report_callback_create_info.pUserData = unsafe { transmute::<&mut Instance, *mut c_void>(&mut instance) };
		vulkan_check!(vk_create_debug_report_callback_ext(vk_instance, &report_callback_create_info, 0 as *const VkAllocationCallbacks, &mut instance.vk_debug_callback));
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
