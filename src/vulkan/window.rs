extern crate libc;

#[cfg(target_os = "linux")]
use super::super::system::xcb::{
    xcb_cw_t,
    xcb_flush,
    xcb_setup_t,
    xcb_connect,
    xcb_window_t,
    xcb_get_setup,
    xcb_disconnect,
    xcb_map_window,
    xcb_generate_id,
    xcb_screen_next,
    xcb_intern_atom,
    xcb_prop_mode_t,
    xcb_connection_t,
    xcb_event_mask_t,
    xcb_create_window,
    xcb_destroy_window,
    xcb_window_class_t,
    xcb_config_window_t,
    xcb_generic_error_t,
    xcb_change_property,
    xcb_configure_window,
    xcb_intern_atom_reply,
    xcb_screen_iterator_t,
    xcb_setup_roots_iterator,

    XCB_COPY_FROM_PARENT,
};

use super::super::system::vulkan::{
    VkRect2D,
    VkFormat,
    VkResult,
    VkBool32,
    VkExtent2D,
    VkOffset2D,
    VkSurfaceKHR,
    VkQueueFlagBits,
    VkColorSpaceKHR,
    VkStructureType,
    VkSurfaceFormatKHR,
    vkDestroySurfaceKHR,
    VkAllocationCallbacks,
    VkQueueFamilyProperties,
    VkSurfaceCapabilitiesKHR,
    vkGetPhysicalDeviceSurfaceFormatsKHR,
    vkGetPhysicalDeviceSurfaceSupportKHR,
    vkGetPhysicalDeviceQueueFamilyProperties,
    vkGetPhysicalDeviceSurfaceCapabilitiesKHR,
};

use super::super::system::vulkan_xcb::{
    vkCreateXcbSurfaceKHR,
    VkXcbSurfaceCreateInfoKHR,
};

use std::mem::zeroed;
use std::default::Default;
use std::ffi::CString;
use std::mem::transmute;
use std::os::raw::{
    c_int,
    c_uint,
    c_char,
};
use std::sync::{
    Arc,
    RwLock,
};

use super::device::Device;

#[cfg(target_os = "linux")]
struct OsWindow {
    connection: *mut xcb_connection_t,
    window: xcb_window_t
}

#[cfg(target_os = "linux")]
impl OsWindow {
    fn new(window: &mut Window, width: u32, height: u32) -> Self {
        let setup: *const xcb_setup_t;
        let mut iter: xcb_screen_iterator_t;
        let mut screen = 0 as c_int;
        let xcb_connection = unsafe { xcb_connect(0 as *const c_char, &mut screen as *mut c_int) };
        if xcb_connection == (0 as *mut xcb_connection_t) {
            panic!("Cannot find a compatible Vulkan ICD.");
        }
        setup = unsafe {xcb_get_setup(xcb_connection) };
        iter = unsafe { xcb_setup_roots_iterator(setup) };
        let _ = setup;
        for _ in 0..screen {
            unsafe { xcb_screen_next(&mut iter as *mut xcb_screen_iterator_t); }
        }
        let xcb_screen = iter.data;
        let _ = iter;
        let dimensions = VkRect2D {
            offset: VkOffset2D {
                x: 0,
                y: 0,
            },
            extent: VkExtent2D {
                width: width,
                height: height
            },
        };
        let value_mask: c_uint;
        let mut value_list = [0 as c_uint; 32];
        let xcb_window = unsafe { xcb_generate_id(xcb_connection) };
        value_mask = (xcb_cw_t::XCB_CW_BACK_PIXEL as c_uint) |
            (xcb_cw_t::XCB_CW_EVENT_MASK as c_uint);
        value_list[0] = unsafe { (*xcb_screen).black_pixel };
        value_list[1] =
            (xcb_event_mask_t::XCB_EVENT_MASK_KEY_RELEASE as c_uint) |
            (xcb_event_mask_t::XCB_EVENT_MASK_KEY_PRESS as c_uint) |
            (xcb_event_mask_t::XCB_EVENT_MASK_EXPOSURE as c_uint) |
            (xcb_event_mask_t::XCB_EVENT_MASK_STRUCTURE_NOTIFY as c_uint) |
            (xcb_event_mask_t::XCB_EVENT_MASK_POINTER_MOTION as c_uint) |
            (xcb_event_mask_t::XCB_EVENT_MASK_BUTTON_PRESS as c_uint) |
            (xcb_event_mask_t::XCB_EVENT_MASK_BUTTON_RELEASE as c_uint);
        unsafe {
            xcb_create_window(
                xcb_connection, XCB_COPY_FROM_PARENT as u8, xcb_window, (*xcb_screen).root,
                dimensions.offset.x as i16, dimensions.offset.y as i16,
                dimensions.extent.width as u16, dimensions.extent.height as u16, 0,
                xcb_window_class_t::XCB_WINDOW_CLASS_INPUT_OUTPUT as u16, (*xcb_screen).root_visual,
                value_mask, value_list.as_ptr() as *const u32);
        }
        let wm_protocols = CString::new("WM_PROTOCOLS").unwrap();
        let cookie = unsafe { xcb_intern_atom(xcb_connection, 1, 12, wm_protocols.as_ptr()) };
        let reply = unsafe { xcb_intern_atom_reply(
            xcb_connection, cookie, 0 as *mut *mut xcb_generic_error_t) };
        let wm_delete_window = CString::new("WM_DELETE_WINDOW").unwrap();
        let cookie2 = unsafe { xcb_intern_atom(xcb_connection, 0, 16, wm_delete_window.as_ptr()) };
        let xcb_atom_window_reply = unsafe { xcb_intern_atom_reply(
            xcb_connection, cookie2, 0 as *mut *mut xcb_generic_error_t) };
        unsafe {
            xcb_change_property(
                xcb_connection, xcb_prop_mode_t::XCB_PROP_MODE_REPLACE as u8, xcb_window,
                (*reply).atom, 4, 32, 1, transmute(&(*xcb_atom_window_reply).atom));
        }
        unsafe { libc::free(reply as *mut libc::c_void) };
        unsafe { xcb_map_window(xcb_connection, xcb_window) };
        let coords = [100 as c_uint; 2];
        unsafe {
            xcb_configure_window(
                xcb_connection, xcb_window,
                (xcb_config_window_t::XCB_CONFIG_WINDOW_X as u16) |
                    (xcb_config_window_t::XCB_CONFIG_WINDOW_Y as u16),
                coords.as_ptr() as *const u32);
        }
        unsafe {
            xcb_flush(xcb_connection);
        }
        let create_info = VkXcbSurfaceCreateInfoKHR {
            sType: VkStructureType::VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR,
            connection: xcb_connection,
            window: xcb_window,
            ..VkXcbSurfaceCreateInfoKHR::default()
        };
        let dev = window.device.read().unwrap();
        let ins = dev.instance.read().unwrap();
        vulkan_check!(vkCreateXcbSurfaceKHR(ins.vk_instance,
            &create_info as *const VkXcbSurfaceCreateInfoKHR, 0 as *const VkAllocationCallbacks,
            &mut window.surface as *mut VkSurfaceKHR));
        OsWindow {
            connection: xcb_connection,
            window: xcb_window,
        }
    }
}

#[cfg(target_os = "linux")]
impl Default for OsWindow {
    fn default() -> Self {
        unsafe {
            zeroed()
        }
    }
}

#[cfg(target_os = "linux")]
impl Drop for OsWindow {
    fn drop(&mut self) {
        if self.connection == 0 as *mut xcb_connection_t {
            return;
        }
        unsafe { xcb_destroy_window(self.connection, self.window); }
        unsafe { xcb_disconnect(self.connection); }
        self.connection = 0 as *mut xcb_connection_t;
    }
}


pub struct Window {
    device: Arc<RwLock<Device>>,
    window: OsWindow,
    surface: VkSurfaceKHR,
    surface_format: VkSurfaceFormatKHR,
}

impl Window {
    pub fn new(device: Arc<RwLock<Device>>) -> Self {
        let mut window = Window {
            device: device,
            window: OsWindow::default(),
            surface: 0 as VkSurfaceKHR,
            surface_format: VkSurfaceFormatKHR::default(),
        };
        window.window = OsWindow::new(&mut window, 900, 500);
        window.initialize_surface();
        return window;
    }
    fn initialize_surface(&mut self) {
        let mut dev = self.device.write().unwrap();
        let mut queue_count = 0u32;
        unsafe {
            vkGetPhysicalDeviceQueueFamilyProperties(
                dev.gpu, &mut queue_count, 0 as *mut VkQueueFamilyProperties);
        }
        if queue_count < 1 {
            panic!("Error no queue found.");
        }
        let mut queue_props = vec![VkQueueFamilyProperties::default(); queue_count as usize];
        unsafe {
            vkGetPhysicalDeviceQueueFamilyProperties(
                dev.gpu, &mut queue_count, queue_props.as_mut_ptr());
        }
        let mut supports_present = vec![0 as VkBool32; queue_count as usize];
        {
            let mut ptr_supports_present = supports_present.as_mut_ptr();
            for i in 0..queue_count {
                vulkan_check!(vkGetPhysicalDeviceSurfaceSupportKHR(
                    dev.gpu, i, self.surface, ptr_supports_present.offset(i as isize)));
            }
        }
        let mut graphics_queue_node_index = u32::max_value();
        let mut present_queue_node_index = u32::max_value();
        for i in 0..queue_count {
            if (queue_props[i as usize].queueFlags &
                (VkQueueFlagBits::VK_QUEUE_GRAPHICS_BIT as u32)) != 0 {
                if graphics_queue_node_index == u32::max_value() {
                    graphics_queue_node_index = i;
                }
                if supports_present[i as usize] != 0 as VkBool32 {
                    graphics_queue_node_index = i;
                    present_queue_node_index = i;
                    break;
                }
            }
        }
        if present_queue_node_index == u32::max_value() {
            for i in 0..queue_count {
                if supports_present[i as usize] != 0 as VkBool32 {
                    present_queue_node_index = i;
                    break;
                }
            }
        }
        if graphics_queue_node_index == u32::max_value() ||
            present_queue_node_index == u32::max_value() {
            panic!("Could not find a graphics and/or presenting queue!");
        }
        // TODO: Add support for separate graphics and presenting queue
        if graphics_queue_node_index != present_queue_node_index {
            panic!("Separate graphics and presenting queues are not supported yet!");
        }
        dev.graphics_family_index = graphics_queue_node_index;
        let mut wsi_supported = 0u32;
        unsafe {
            vkGetPhysicalDeviceSurfaceSupportKHR(
                dev.gpu, dev.graphics_family_index, self.surface, &mut wsi_supported as *mut u32);
        }
        if wsi_supported == 0 {
            panic!("Error WSI is not supported for device.");
        }
        let mut surface_capabilities = VkSurfaceCapabilitiesKHR::default();
        unsafe {
            vkGetPhysicalDeviceSurfaceCapabilitiesKHR(
                dev.gpu, self.surface,
                &mut surface_capabilities as *mut VkSurfaceCapabilitiesKHR);
        }
        let mut format_count = 0u32;
        unsafe {
            vkGetPhysicalDeviceSurfaceFormatsKHR(
                dev.gpu, self.surface,
                &mut format_count as *mut u32, 0 as *mut VkSurfaceFormatKHR);
        }
        if format_count == 0 {
            panic!("Surface formats missing.");
        }
        let mut formats = vec![VkSurfaceFormatKHR::default(); format_count as usize];
        unsafe {
            vkGetPhysicalDeviceSurfaceFormatsKHR(
                dev.gpu, self.surface, &mut format_count as *mut u32,
                formats.as_mut_ptr() as *mut VkSurfaceFormatKHR);
        }
        if (formats[0].format as u32) == (VkFormat::VK_FORMAT_UNDEFINED as u32) {
            self.surface_format.format = VkFormat::VK_FORMAT_B8G8R8A8_UNORM;
            self.surface_format.colorSpace = VkColorSpaceKHR::VK_COLORSPACE_SRGB_NONLINEAR_KHR;
        } else {
            self.surface_format = formats[0];
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        let dev = self.device.read().unwrap();
        let ins = dev.instance.read().unwrap();
        unsafe {
            vkDestroySurfaceKHR(ins.vk_instance, self.surface, 0 as *const VkAllocationCallbacks);
        }
        self.window = OsWindow::default();
    }
}