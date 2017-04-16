#![allow(dead_code, non_camel_case_types, non_upper_case_globals, non_snake_case)]
extern crate winapi;
use std::default::Default;
use std::mem::zeroed;
use self::winapi::minwindef::HINSTANCE;
use self::winapi::windef::HWND;
use super::super::super::system::vulkan as vk;
pub type VkWin32SurfaceCreateFlagsKHR = vk::VkFlags;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VkWin32SurfaceCreateInfoKHR {
    pub sType: vk::VkStructureType,
    pub pNext: *const ::std::os::raw::c_void,
    pub flags: VkWin32SurfaceCreateFlagsKHR,
    pub hinstance: HINSTANCE,
    pub hwnd: HWND,
}
impl Default for VkWin32SurfaceCreateInfoKHR {
    fn default() -> Self {
        unsafe {
            zeroed()
        }
    }
}
#[link(name = "vulkan-1", kind= "dylib")]
extern {
    pub fn vkCreateWin32SurfaceKHR(
        instance: vk::VkInstance,
        pCreateInfo: *const VkWin32SurfaceCreateInfoKHR,
        pAllocator: *const vk::VkAllocationCallbacks,
        pSurface: *mut vk::VkSurfaceKHR) -> vk::VkResult;
}
