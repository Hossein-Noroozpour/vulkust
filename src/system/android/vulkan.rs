use std::default::Default;
use std::mem::zeroed;
use libc::{
    c_void,
};
use super::super::vulkan::{
    VkFlags,
    VkResult,
    VkInstance,
    VkSurfaceKHR,
    VkStructureType,
    VkAllocationCallbacks,
};
use super::window::ANativeWindow;

pub const VK_KHR_ANDROID_SURFACE_SPEC_VERSION: u32 = 6u32;
pub const VK_KHR_ANDROID_SURFACE_EXTENSION_NAME: &'static str = "VK_KHR_android_surface";

type VkAndroidSurfaceCreateFlagsKHR = VkFlags;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkAndroidSurfaceCreateInfoKHR {
    pub structure_type: VkStructureType,
    pub pointer_next: *const c_void,
    pub flags: VkAndroidSurfaceCreateFlagsKHR,
    pub window: *mut ANativeWindow,
}

impl Default for VkAndroidSurfaceCreateInfoKHR {
    fn default() -> Self {
        unsafe {
            zeroed()
        }
    }
}

pub type PfnVkCreateAndroidSurfaceKhr = unsafe extern fn (
    instance: VkInstance, p_create_info: *const VkAndroidSurfaceCreateInfoKHR,
    p_allocator: *const VkAllocationCallbacks, p_surface: *mut VkSurfaceKHR) -> VkResult;

#[cfg_attr(target_os = "android", link(name = "vulkan", kind= "dylib"))]
extern {
    pub fn vkCreateAndroidSurfaceKHR(
        instance: VkInstance, p_create_info: *const VkAndroidSurfaceCreateInfoKHR,
        p_allocator: *const VkAllocationCallbacks, p_surface: *mut VkSurfaceKHR) -> VkResult;
}
