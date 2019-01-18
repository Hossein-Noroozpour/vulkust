use super::super::core::string::{cstrings_to_ptrs, strings_to_cstrings};
use super::super::render::config::Configurations;
use super::super::render::image::Format;
use super::surface::Surface;
use ash::version::DeviceV1_0;
use ash::version::InstanceV1_0;
use ash::vk;
use std::collections::HashSet;
use std::sync::Arc;
// use std::cmp::max;

fn get_supported_depth_format(ins: &ash::Instance, s: &vk::PhysicalDevice) -> vk::Format {
    let depth_formats = vec![
        vk::Format::D32_SFLOAT,
        vk::Format::D32_SFLOAT_S8_UINT,
        vk::Format::D24_UNORM_S8_UINT,
        vk::Format::D16_UNORM_S8_UINT,
        vk::Format::D16_UNORM,
    ];
    for format in depth_formats {
        let format_props = ins.get_physical_device_format_properties(*s, format);
        if vxflagcheck!(
            format_props.optimal_tiling_features,
            vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT
        ) {
            return format;
        }
    }
    vxlogf!("No depth format found!");
}

fn get_surface_formats(
    surface: &Arc<Surface>,
    physical_device: &vk::PhysicalDevice,
) -> Vec<vk::SurfaceFormatKHR> {
    let surface_loader = surface.get_loader();
    let vk_surface = surface.get_data();
    return vxresult!(unsafe {
        surface_loader.get_physical_device_surface_formats(*physical_device, *vk_surface)
    });
}

#[cfg_attr(debug_mode, derive(Debug))]
pub(super) struct Physical {
    surface: Arc<Surface>,
    graphics_queue_node_index: u32,
    transfer_queue_node_index: u32,
    compute_queue_node_index: u32,
    present_queue_node_index: u32,
    vk_data: vk::PhysicalDevice,
    memory_properties: vk::PhysicalDeviceMemoryProperties,
    properties: vk::PhysicalDeviceProperties,
    surface_caps: vk::SurfaceCapabilitiesKHR,
    supported_depth_format: vk::Format,
    surface_formats: Vec<vk::SurfaceFormatKHR>,
    features: vk::PhysicalDeviceFeatures,
}

impl Physical {
    pub(super) fn new(surface: &Arc<Surface>) -> Self {
        let instance = surface.get_instance();
        let vk_instance = instance.get_data();
        let (
            vk_data,
            graphics_queue_node_index,
            transfer_queue_node_index,
            compute_queue_node_index,
            present_queue_node_index,
        ) = Self::find_device(surface);
        let memory_properties = vk_instance.get_physical_device_memory_properties(vk_data);
        let properties = vk_instance.get_physical_device_properties(vk_data);
        let surface_caps = vxresult!(surface
            .get_loader()
            .get_physical_device_surface_capabilities(vk_data, *surface.get_data()));
        #[cfg(debug_mode)]
        vxlogi!("Surface capacities are {:?}", &surface_caps);
        let supported_depth_format = get_supported_depth_format(vk_instance, &vk_data);
        let surface_formats = get_surface_formats(surface, &vk_data);
        let features = unsafe { vk_instance.get_physical_device_features(vk_data) };
        let surface = surface.clone();
        Self {
            surface,
            graphics_queue_node_index,
            transfer_queue_node_index,
            compute_queue_node_index,
            present_queue_node_index,
            vk_data,
            memory_properties,
            properties,
            surface_caps,
            supported_depth_format,
            surface_formats,
            features,
        }
    }

    #[inline]
    pub(super) fn get_surface(&self) -> &Surface {
        return &self.surface;
    }

    #[inline]
    pub(super) fn get_surface_capabilities(&self) -> &vk::SurfaceCapabilitiesKHR {
        return &self.surface_caps;
    }

    fn find_device(surface: &Arc<Surface>) -> (vk::PhysicalDevice, u32, u32, u32, u32) {
        let instance = surface.get_instance();
        let vk_instance = instance.get_data();
        let devices = Self::enumerate_devices(vk_instance);
        #[cfg(debug_mode)]
        vxlogi!("Number of physical devices is: {}", devices.len());
        for device in &devices {
            if Self::device_is_discrete(vk_instance, device) {
                match Self::fetch_queues(*device, surface) {
                    Some((g, t, c, p)) => return (*device, g, t, c, p),
                    None => {}
                }
            }
        }
        for device in &devices {
            if !Self::device_is_discrete(vk_instance, device) {
                match Self::fetch_queues(*device, surface) {
                    Some((g, t, c, p)) => return (*device, g, t, c, p),
                    None => {}
                }
            }
        }
        vxlogf!("Required device not found!");
    }

    fn device_is_discrete(instance: &ash::Instance, device: &vk::PhysicalDevice) -> bool {
        get_properties(instance, device).device_type == vk::PhysicalDeviceType::DISCRETE_GPU
    }

    fn enumerate_devices(vk_instance: &ash::Instance) -> Vec<vk::PhysicalDevice> {
        return vxresult!(vk_instance.enumerate_physical_devices());
    }

    fn fetch_queues(
        device: vk::PhysicalDevice,
        surface: &Arc<Surface>,
    ) -> Option<(u32, u32, u32, u32)> {
        let instance = surface.get_instance();
        let vk_instance = instance.get_data();
        let vk_surface = surface.get_data();
        let surface_loader = surface.get_loader();
        let queue_family_properties = Self::get_device_queue_family_properties(vk_instance, device);
        if queue_family_properties.len() == 0 {
            return None;
        }
        let mut graphics_queue_node_index = u32::max_value();
        let mut transfer_queue_node_index = u32::max_value();
        let mut compute_queue_node_index = u32::max_value();
        let mut present_queue_node_index = u32::max_value();

        for i in 0..(queue_family_properties.len() as u32) {
            let ref queue_family = queue_family_properties[i as usize];
            let b = unsafe {
                surface_loader.get_physical_device_surface_support(device, i, *vk_surface)
            };
            if queue_family.queue_count > 0
                && b
                && vxflagcheck!(queue_family.queue_flags, vk::QueueFlags::GRAPHICS)
                && vxflagcheck!(queue_family.queue_flags, vk::QueueFlags::COMPUTE)
                && vxflagcheck!(queue_family.queue_flags, vk::QueueFlags::TRANSFER)
            {
                return Some((i, i, i, i));
            }
        }

        for i in 0..(queue_family_properties.len() as u32) {
            let ref queue_family = queue_family_properties[i as usize];
            let b = unsafe {
                surface_loader.get_physical_device_surface_support(device, i, *vk_surface)
            };
            if queue_family.queue_count > 0
                && b
                && vxflagcheck!(queue_family.queue_flags, vk::QueueFlags::GRAPHICS)
                && vxflagcheck!(queue_family.queue_flags, vk::QueueFlags::COMPUTE)
            {
                graphics_queue_node_index = i;
                compute_queue_node_index = i;
                present_queue_node_index = i;
            }
            if queue_family.queue_count > 0
                && vxflagcheck!(queue_family.queue_flags, vk::QueueFlags::TRANSFER)
            {
                transfer_queue_node_index = i;
            }
        }

        if graphics_queue_node_index == u32::max_value() {
            return None;
        }

        if transfer_queue_node_index == u32::max_value() {
            transfer_queue_node_index = graphics_queue_node_index;
        }

        return Some((
            graphics_queue_node_index,
            transfer_queue_node_index,
            compute_queue_node_index,
            present_queue_node_index,
        ));
    }

    fn get_device_queue_family_properties(
        vk_instance: &ash::Instance,
        device: vk::PhysicalDevice,
    ) -> Vec<vk::QueueFamilyProperties> {
        unsafe {
            return vxresult!(vk_instance.get_physical_device_queue_family_properties(device));
        }
    }

    // pub(super) fn get_queue_family_properties(&self) -> Vec<vk::VkQueueFamilyProperties> {
    //     Self::get_device_queue_family_properties(self.vk_data)
    // }

    #[inline]
    fn get_supported_depth_format(&self) -> vk::Format {
        return self.supported_depth_format;
    }

    #[inline]
    pub(super) fn get_surface_formats(&self) -> &[vk::SurfaceFormatKHR] {
        return &self.surface_formats;
    }

    pub(super) fn get_memory_type_index(
        &self,
        type_bits: u32,
        properties: ash::vk::MemoryPropertyFlags,
    ) -> u32 {
        let mut type_bits = type_bits;
        for i in 0..self.memory_properties.memory_type_count {
            if (type_bits & 1) == 1 {
                if vxflagcheck!(
                    self.memory_properties.memory_types[i as usize].property_flags,
                    properties
                ) {
                    return i;
                }
            }
            type_bits >>= 1;
        }
        vxlogf!("Could not find the requsted memory type.");
    }

    // pub(super) fn get_max_min_alignment(&self) -> u64 {
    //     let limits = &self.properties.limits;
    //     max(
    //         max(
    //             max(
    //                 limits.minMemoryMapAlignment as u64,
    //                 limits.minStorageBufferOffsetAlignment,
    //             ),
    //             max(
    //                 limits.minTexelBufferOffsetAlignment,
    //                 limits.minUniformBufferOffsetAlignment,
    //             ),
    //         ),
    //         max(
    //             max(
    //                 limits.optimalBufferCopyOffsetAlignment,
    //                 limits.optimalBufferCopyRowPitchAlignment,
    //             ),
    //             limits.bufferImageGranularity,
    //         ),
    //     )
    // }

    #[inline]
    pub(super) fn get_vk_features(&self) -> &vk::PhysicalDeviceFeatures {
        return &self.features;
    }

    #[inline]
    pub(super) fn get_graphics_queue_node_index(&self) -> u32 {
        return self.graphics_queue_node_index;
    }

    #[inline]
    pub(super) fn get_transfer_queue_node_index(&self) -> u32 {
        return self.transfer_queue_node_index;
    }

    #[inline]
    pub(super) fn get_compute_queue_node_index(&self) -> u32 {
        return self.compute_queue_node_index;
    }

    #[inline]
    pub(super) fn get_present_queue_node_index(&self) -> u32 {
        return self.present_queue_node_index;
    }

    #[inline]
    pub(super) fn get_data(&self) -> &vk::PhysicalDevice {
        return &self.vk_data;
    }

    #[inline]
    pub(super) fn get_properties(&self) -> &vk::PhysicalDeviceProperties {
        return &self.properties;
    }

    pub(super) fn convert_format(&self, f: Format) -> vk::Format {
        match f {
            Format::RgbaFloat => return vk::Format::R32G32B32A32_SFLOAT,
            Format::DepthFloat => return self.get_supported_depth_format(),
            Format::Float => return vk::Format::R32_SFLOAT,
            Format::FlagBits8 => return vk::Format::R8_UNORM,
            Format::FlagBits64 => return vk::Format::R32G32_UINT,
            _ => vxunexpected!(),
        }
    }

    #[inline]
    pub(super) fn get_vk_instance(&self) -> &ash::Instance {
        return self.surface.get_instance().get_data();
    }
}

impl Drop for Physical {
    fn drop(&mut self) {}
}

unsafe impl Send for Physical {}

unsafe impl Sync for Physical {}

fn get_properties(
    instance: &ash::Instance,
    device: &vk::PhysicalDevice,
) -> vk::PhysicalDeviceProperties {
    unsafe {
        return instance.get_physical_device_properties(*device);
    }
}

pub(crate) struct Logical {
    physical_device: Arc<Physical>,
    vk_data: ash::Device,
    vk_graphic_queue: vk::Queue,
    vk_compute_queue: vk::Queue,
    vk_present_queue: vk::Queue,
}

impl Logical {
    pub(super) fn new(physical_device: &Arc<Physical>, config: &Configurations) -> Self {
        let mut device_extensions = Vec::new();
        device_extensions.push("VK_KHR_swapchain".to_string());
        let device_extensions = strings_to_cstrings(device_extensions);
        let device_extensions = cstrings_to_ptrs(&device_extensions);
        let mut queue_family_index_set = HashSet::new();
        queue_family_index_set.insert(physical_device.get_graphics_queue_node_index());
        queue_family_index_set.insert(physical_device.get_transfer_queue_node_index());
        queue_family_index_set.insert(physical_device.get_compute_queue_node_index());
        queue_family_index_set.insert(physical_device.get_present_queue_node_index());
        let mut queue_create_info_s = Vec::new();
        let queue_priorities = [1f32];
        // TODO: create as many as possible queue to separate independent works as many as possible
        // on the queues but it is not required currently
        for q in queue_family_index_set {
            queue_create_info_s.push(
                vk::DeviceQueueCreateInfo::builder()
                    .queue_family_index(q)
                    .queue_priorities(&queue_priorities)
                    .build(),
            );
        }
        let available_features = physical_device.get_vk_features();
        let mut features = vk::PhysicalDeviceFeatures::default();
        if config.get_enable_anistropic_texture() {
            features.sampler_anisotropy = available_features.sampler_anisotropy;
        }
        let device_create_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(&queue_create_info_s)
            .enabled_extension_names(&device_extensions)
            .enabled_features(&features);
        let vk_instance = physical_device.get_vk_instance();
        let vk_data = vxresult!(unsafe {
            vk_instance.create_device(*physical_device.get_data(), &device_create_info, None)
        });
        let vk_graphic_queue =
            unsafe { vk_data.get_device_queue(physical_device.get_graphics_queue_node_index(), 0) };
        let vk_compute_queue =
            unsafe { vk_data.get_device_queue(physical_device.get_compute_queue_node_index(), 0) };
        let vk_present_queue =
            unsafe { vk_data.get_device_queue(physical_device.get_present_queue_node_index(), 0) };
        Self {
            physical_device: physical_device.clone(),
            vk_data,
            vk_graphic_queue,
            vk_compute_queue,
            vk_present_queue,
        }
    }

    pub(crate) fn get_uniform_buffer_alignment(&self) -> isize {
        self.physical_device
            .get_properties()
            .limits
            .min_uniform_buffer_offset_alignment as isize
    }

    #[inline]
    pub(super) fn wait_idle(&self) {
        vxresult!(unsafe { self.vk_data.device_wait_idle() });
    }

    #[inline]
    pub(super) fn get_data(&self) -> &ash::Device {
        return &self.vk_data;
    }

    #[inline]
    pub(super) fn get_physical(&self) -> &Physical {
        return &self.physical_device;
    }

    #[inline]
    pub(super) fn get_vk_graphic_queue(&self) -> vk::Queue {
        return self.vk_graphic_queue;
    }

    #[inline]
    pub(super) fn convert_format(&self, f: Format) -> vk::Format {
        return self.physical_device.convert_format(f);
    }

    #[inline]
    pub(super) fn get_vk_instance(&self) -> &ash::Instance {
        return self.physical_device.get_vk_instance();
    }

    // pub(super) fn get_vk_compute_queue(&self) -> vk::VkQueue {
    //     return self.vk_compute_queue;
    // }

    // pub(super) fn get_vk_present_queue(&self) -> vk::VkQueue {
    //     return self.vk_present_queue;
    // }
}

impl Drop for Logical {
    fn drop(&mut self) {
        self.wait_idle();
    }
}

#[cfg(debug_mode)]
impl std::fmt::Debug for Logical {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Vulkan Logical Device")
    }
}

unsafe impl Send for Logical {}

unsafe impl Sync for Logical {}
