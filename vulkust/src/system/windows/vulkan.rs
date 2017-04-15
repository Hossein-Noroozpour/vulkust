#![allow(dead_code, non_camel_case_types, non_upper_case_globals, non_snake_case)]
use super::super::vulkan as vk;
use super::xcb::{
    xcb_window_t,
    xcb_connection_t,
};

pub type VkWin32SurfaceCreateFlagsKHR = VkFlags;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VkWin32SurfaceCreateInfoKHR {
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
