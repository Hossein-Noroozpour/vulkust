use super::device::Logical as LogicalDevice;
use super::image::View as ImageView;
use super::sync::Semaphore;
use ash::extensions::khr::Swapchain as SwapchainLoader;
use ash::version::InstanceV1_0;
use ash::vk;
use std::sync::Arc;

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) enum NextImageResult {
    NeedsRefresh,
    Next(u32),
}

pub(crate) struct Swapchain {
    logical_device: Arc<LogicalDevice>,
    _surface_format: vk::SurfaceFormatKHR,
    image_views: Vec<Arc<ImageView>>,
    vk_data: vk::SwapchainKHR,
    loader: SwapchainLoader,
}

impl Swapchain {
    pub fn new(logical_device: &Arc<LogicalDevice>) -> Self {
        let surface_caps = logical_device.get_physical().get_surface_capabilities();
        let surface_formats = logical_device.get_physical().get_surface_formats();
        let mut best_surface_format = vk::SurfaceFormatKHR::default();
        for format in surface_formats {
            if format.format == vk::Format::R8G8B8A8_UNORM {
                best_surface_format = *format;
                break;
            }
        }
        for format in surface_formats {
            if format.format == vk::Format::B8G8R8A8_UNORM {
                best_surface_format = *format;
                break;
            }
        }
        if best_surface_format.format != vk::Format::R8G8B8A8_UNORM
            && best_surface_format.format != vk::Format::B8G8R8A8_UNORM
        {
            vxlogi!("VK_FORMAT_R8G8B8A8_UNORM not found in the surface.");
            best_surface_format = surface_formats[0];
            vxlogi!("The specified format is {:?}", best_surface_format);
        }
        let mut swapchain_images_count = surface_caps.min_image_count + 1;
        if surface_caps.max_image_count > 0 && swapchain_images_count > surface_caps.max_image_count
        {
            swapchain_images_count = surface_caps.max_image_count;
        }
        #[cfg(debug_mode)]
        vxlogi!("Swapchain images count: {:?}", swapchain_images_count);
        let mut image_usage = vk::ImageUsageFlags::COLOR_ATTACHMENT;
        let physical_device = logical_device.get_physical();
        let vk_physical_device = physical_device.get_data();
        let vk_instance = physical_device.get_vk_instance();
        let format_props = unsafe {
            vk_instance.get_physical_device_format_properties(
                *vk_physical_device,
                best_surface_format.format,
            )
        };
        if vxflagcheck!(
            format_props.optimal_tiling_features,
            vk::FormatFeatureFlags::BLIT_DST
        ) {
            image_usage |= vk::ImageUsageFlags::TRANSFER_SRC;
        }
        let queue_family_indices = [
            logical_device
                .get_physical()
                .get_graphics_queue_node_index(),
            logical_device.get_physical().get_present_queue_node_index(),
        ];
        let shared = queue_family_indices[0] != queue_family_indices[1];
        let swapchain_create_info = vk::SwapchainCreateInfoKHR::builder()
            .surface(*logical_device.get_physical().get_surface().get_data())
            .min_image_count(swapchain_images_count)
            .image_format(best_surface_format.format)
            .image_color_space(best_surface_format.color_space)
            .image_extent(surface_caps.current_extent)
            .image_usage(image_usage)
            .pre_transform(vk::SurfaceTransformFlagsKHR::IDENTITY)
            .image_array_layers(1)
            .image_sharing_mode(if shared {
                vk::SharingMode::CONCURRENT
            } else {
                vk::SharingMode::EXCLUSIVE
            })
            .queue_family_indices(if shared { &queue_family_indices } else { &[] })
            .present_mode(vk::PresentModeKHR::FIFO)
            .clipped(true)
            .composite_alpha(
                if vxflagcheck!(
                    surface_caps.supported_composite_alpha,
                    vk::CompositeAlphaFlagsKHR::OPAQUE
                ) {
                    vk::CompositeAlphaFlagsKHR::OPAQUE
                } else if vxflagcheck!(
                    surface_caps.supported_composite_alpha,
                    vk::CompositeAlphaFlagsKHR::INHERIT
                ) {
                    vk::CompositeAlphaFlagsKHR::INHERIT
                } else if vxflagcheck!(
                    surface_caps.supported_composite_alpha,
                    vk::CompositeAlphaFlagsKHR::PRE_MULTIPLIED
                ) {
                    vk::CompositeAlphaFlagsKHR::PRE_MULTIPLIED
                } else if vxflagcheck!(
                    surface_caps.supported_composite_alpha,
                    vk::CompositeAlphaFlagsKHR::POST_MULTIPLIED
                ) {
                    vk::CompositeAlphaFlagsKHR::POST_MULTIPLIED
                } else {
                    vxlogf!("Error composite is unknown.");
                },
            );
        let vk_dev = logical_device.get_data();
        let vk_instance = logical_device.get_vk_instance();
        let loader = SwapchainLoader::new(vk_instance, vk_dev);
        let vk_data = vxresult!(unsafe { loader.create_swapchain(&swapchain_create_info, None) });
        let images = vxresult!(unsafe { loader.get_swapchain_images(vk_data) });
        let mut image_views = Vec::with_capacity(images.len());
        for img in images {
            image_views.push(Arc::new(ImageView::new_with_vk_image(
                logical_device.clone(),
                img,
                best_surface_format.format,
                vk::ImageLayout::UNDEFINED,
                vk::ImageUsageFlags::COLOR_ATTACHMENT,
                surface_caps.current_extent.width,
                surface_caps.current_extent.height,
            )));
        }
        vxlogi!("Swapchain created.");
        Self {
            logical_device: logical_device.clone(),
            _surface_format: best_surface_format,
            image_views,
            vk_data,
            loader,
        }
    }

    pub(crate) fn get_next_image_index(&self, sem: &Arc<Semaphore>) -> NextImageResult {
        let (image_index, is_suboptimal) = vxresult!(unsafe {
            self.loader.acquire_next_image(
                self.vk_data,
                u64::max_value(),
                *sem.get_data(),
                vk::Fence::null(),
            )
        });
        if is_suboptimal {
            return NextImageResult::NeedsRefresh;
        }
        return NextImageResult::Next(image_index);
    }

    pub(crate) fn get_image_views(&self) -> &[Arc<ImageView>] {
        return &self.image_views;
    }

    pub(crate) fn get_logical_device(&self) -> &Arc<LogicalDevice> {
        return &self.logical_device;
    }

    pub(super) fn get_loader(&self) -> &SwapchainLoader {
        return &self.loader;
    }

    pub(super) fn get_data(&self) -> &vk::SwapchainKHR {
        return &self.vk_data;
    }
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        self.image_views.clear();
        unsafe {
            self.loader.destroy_swapchain(self.vk_data, None);
        }
    }
}

#[cfg(debug_mode)]
impl std::fmt::Debug for Swapchain {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Vulkan Swapchain")
    }
}
