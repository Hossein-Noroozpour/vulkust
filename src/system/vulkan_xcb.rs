#![allow(dead_code, non_camel_case_types, non_upper_case_globals, non_snake_case)]
use system::vulkan::{
    VkFlags,
    VkResult,
    VkInstance,
    VkSurfaceKHR,
    VkStructureType,
    VkAllocationCallbacks,
};
use system::xcb::{
    xcb_window_t,
    xcb_connection_t,
};

pub type VkXcbSurfaceCreateFlagsKHR = VkFlags;

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct VkXcbSurfaceCreateInfoKHR {
    pub sType: VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: VkXcbSurfaceCreateFlagsKHR,
    pub connection: *mut xcb_connection_t,
    pub window: xcb_window_t,
}

impl ::std::default::Default for VkXcbSurfaceCreateInfoKHR {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[link(name = "vulkan", kind= "dylib")]
extern "C" {
    pub fn vkCreateXcbSurfaceKHR(
        instance: VkInstance,
        pCreateInfo: *const VkXcbSurfaceCreateInfoKHR,
        pAllocator: *const VkAllocationCallbacks,
        pSurface: *mut VkSurfaceKHR) -> VkResult;
}