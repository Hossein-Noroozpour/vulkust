use super::super::buffer::Manager as BufferManager;
use super::super::device::logical::Logical as LogicalDevice;
use super::super::memory::Manager as MemeoryManager;
use super::super::vulkan as vk;
use super::Image;
use std::default::Default;
use std::ptr::null;
use std::sync::{Arc, RwLock};

pub struct View {
    pub image: Arc<RwLock<Image>>,
    pub vk_data: vk::VkImageView,
}

impl View {
    pub fn new_depth_stencil(
        logical_device: Arc<LogicalDevice>,
        memory_mgr: &Arc<RwLock<MemeoryManager>>,
    ) -> Self {
        let depth_format = logical_device.physical_device.get_supported_depth_format();
        let surface_caps = logical_device.physical_device.surface_caps;
        let mut image_info = vk::VkImageCreateInfo::default();
        image_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO;
        image_info.imageType = vk::VkImageType::VK_IMAGE_TYPE_2D;
        image_info.format = depth_format;
        image_info.extent.width = surface_caps.currentExtent.width;
        image_info.extent.height = surface_caps.currentExtent.height;
        image_info.extent.depth = 1;
        image_info.mipLevels = 1;
        image_info.arrayLayers = 1;
        image_info.samples = vk::VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT;
        image_info.tiling = vk::VkImageTiling::VK_IMAGE_TILING_OPTIMAL;
        image_info.usage = vk::VkImageUsageFlagBits::VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT
            as u32
            | vk::VkImageUsageFlagBits::VK_IMAGE_USAGE_TRANSFER_SRC_BIT as u32;
        image_info.initialLayout = vk::VkImageLayout::VK_IMAGE_LAYOUT_UNDEFINED;
        let image = Image::new_with_info(&image_info, memory_mgr);
        let mut depth_stencil_view_info = vk::VkImageViewCreateInfo::default();
        depth_stencil_view_info.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO;
        depth_stencil_view_info.viewType = vk::VkImageViewType::VK_IMAGE_VIEW_TYPE_2D;
        depth_stencil_view_info.format = depth_format;
        depth_stencil_view_info.subresourceRange.aspectMask =
            vk::VkImageAspectFlagBits::VK_IMAGE_ASPECT_DEPTH_BIT as u32
                | vk::VkImageAspectFlagBits::VK_IMAGE_ASPECT_STENCIL_BIT as u32;
        depth_stencil_view_info.subresourceRange.levelCount = 1;
        depth_stencil_view_info.subresourceRange.layerCount = 1;
        depth_stencil_view_info.image = image.vk_data;
        let mut vk_data = 0 as vk::VkImageView;
        vulkan_check!(vk::vkCreateImageView(
            logical_device.vk_data,
            &depth_stencil_view_info,
            null(),
            &mut vk_data,
        ));
        View {
            image: Arc::new(RwLock::new(image)),
            vk_data,
        }
    }

    pub fn new_with_vk_image(
        logical_device: Arc<LogicalDevice>,
        vk_image: vk::VkImage,
        format: vk::VkFormat,
        layout: vk::VkImageLayout,
    ) -> Self {
        Self::new_with_image(Arc::new(RwLock::new(Image::new_with_vk_data(
            logical_device.clone(),
            vk_image,
            layout,
            format,
        ))))
    }

    pub fn new_texture_with_file(file: &str, buffmgr: &Arc<RwLock<BufferManager>>) -> Self {
        let image = Image::new_with_file_name(file, buffmgr);
        Self::new_with_image(image)
    }

    pub fn new_with_image(image: Arc<RwLock<Image>>) -> Self {
        let vk_data = Self::create_vk_data_with_image(&image);
        View { image, vk_data }
    }

    pub fn create_vk_data_with_image(image: &Arc<RwLock<Image>>) -> vk::VkImageView {
        let img = vxresult!(image.read());
        let ref dev = &img.logical_device;
        let mut view_create_info = vk::VkImageViewCreateInfo::default();
        view_create_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO;
        view_create_info.image = img.vk_data;
        view_create_info.viewType = vk::VkImageViewType::VK_IMAGE_VIEW_TYPE_2D;
        view_create_info.format = img.format;
        view_create_info.components.r = vk::VkComponentSwizzle::VK_COMPONENT_SWIZZLE_R;
        view_create_info.components.g = vk::VkComponentSwizzle::VK_COMPONENT_SWIZZLE_G;
        view_create_info.components.b = vk::VkComponentSwizzle::VK_COMPONENT_SWIZZLE_B;
        view_create_info.components.a = vk::VkComponentSwizzle::VK_COMPONENT_SWIZZLE_A;
        view_create_info.subresourceRange.aspectMask =
            vk::VkImageAspectFlagBits::VK_IMAGE_ASPECT_COLOR_BIT as u32;
        view_create_info.subresourceRange.levelCount = img.mips_count as u32;
        view_create_info.subresourceRange.layerCount = 1;
        let mut vk_data = 0 as vk::VkImageView;
        vulkan_check!(vk::vkCreateImageView(
            dev.vk_data,
            &view_create_info,
            null(),
            &mut vk_data,
        ));
        vk_data
    }
}

impl Drop for View {
    fn drop(&mut self) {
        unsafe {
            let img = vxresult!(self.image.read());
            vk::vkDestroyImageView(img.logical_device.vk_data, self.vk_data, null());
        }
    }
}
