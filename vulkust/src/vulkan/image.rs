use super::super::render::image::AttachmentType;
use super::buffer::Manager as BufferManager;
use super::command::Buffer as CmdBuffer;
use super::device::logical::Logical as LogicalDevice;
use super::memory::{Location as MemeoryLocation, Manager as MemeoryManager, Memory};
use super::vulkan as vk;

use std::default::Default;
use std::ptr::null;
use std::sync::{Arc, RwLock};

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Image {
    pub logical_device: Arc<LogicalDevice>,
    pub layout: vk::VkImageLayout,
    pub format: vk::VkFormat,
    pub mips_count: u8,
    pub samples: vk::VkSampleCountFlagBits,
    pub usage: vk::VkImageUsageFlags,
    pub memory: Option<Arc<RwLock<Memory>>>,
    width: u32,
    height: u32,
    pub vk_data: vk::VkImage,
}

impl Image {
    pub fn new_with_info(
        info: &vk::VkImageCreateInfo,
        memory_mgr: &Arc<RwLock<MemeoryManager>>,
    ) -> Self {
        let logical_device = vxresult!(memory_mgr.read()).logical_device.clone();
        let mut vk_data = 0 as vk::VkImage;
        vulkan_check!(vk::vkCreateImage(
            logical_device.vk_data,
            info,
            null(),
            &mut vk_data,
        ));
        let mut mem_reqs = vk::VkMemoryRequirements::default();
        unsafe {
            vk::vkGetImageMemoryRequirements(logical_device.vk_data, vk_data, &mut mem_reqs);
        }
        vxlogi!("{:?}", &mem_reqs);
        let memory = vxresult!(memory_mgr.write()).allocate(&mem_reqs, MemeoryLocation::GPU);
        {
            let memory_r = vxresult!(memory.read());
            let root_memory = vxresult!(memory_r.root_memory.read());
            vulkan_check!(vk::vkBindImageMemory(
                logical_device.vk_data,
                vk_data,
                root_memory.vk_data,
                memory_r.info.offset as vk::VkDeviceSize,
            ));
        }
        Image {
            logical_device,
            vk_data,
            layout: info.initialLayout,
            format: info.format,
            mips_count: info.mipLevels as u8,
            samples: info.samples,
            usage: info.usage,
            width: info.extent.width,
            height: info.extent.height,
            memory: Some(memory),
        }
    }

    pub fn new_with_vk_data(
        logical_device: Arc<LogicalDevice>,
        vk_data: vk::VkImage,
        layout: vk::VkImageLayout,
        format: vk::VkFormat,
        usage: vk::VkImageUsageFlags,
        width: u32,
        height: u32,
        samples: vk::VkSampleCountFlagBits,
    ) -> Self {
        Image {
            logical_device,
            layout,
            format,
            mips_count: 1,
            vk_data,
            usage,
            samples,
            width,
            height,
            memory: None,
        }
    }

    pub(crate) fn new_2d_with_pixels(
        width: u32,
        height: u32,
        data: &[u8],
        buffmgr: &Arc<RwLock<BufferManager>>,
    ) -> Arc<RwLock<Self>> {
        let format = vk::VkFormat::VK_FORMAT_R8G8B8A8_UNORM;
        let mut image_info = vk::VkImageCreateInfo::default();
        image_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO;
        image_info.imageType = vk::VkImageType::VK_IMAGE_TYPE_2D;
        image_info.format = format;
        image_info.extent.width = width;
        image_info.extent.height = height;
        image_info.extent.depth = 1;
        image_info.mipLevels = 1;
        image_info.arrayLayers = 1;
        image_info.samples = vk::VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT;
        image_info.tiling = vk::VkImageTiling::VK_IMAGE_TILING_OPTIMAL;
        image_info.initialLayout = vk::VkImageLayout::VK_IMAGE_LAYOUT_UNDEFINED;
        image_info.sharingMode = vk::VkSharingMode::VK_SHARING_MODE_EXCLUSIVE;
        image_info.usage = vk::VkImageUsageFlagBits::VK_IMAGE_USAGE_TRANSFER_DST_BIT as u32
            | vk::VkImageUsageFlagBits::VK_IMAGE_USAGE_SAMPLED_BIT as u32;
        let memmgr = {
            let buffmgr = vxresult!(buffmgr.read());
            let memmgr = vxresult!(buffmgr.gpu_buffer.memory.read()).manager.clone();
            memmgr
        };
        let myself = Arc::new(RwLock::new(Self::new_with_info(&image_info, &memmgr)));
        vxresult!(buffmgr.write()).create_staging_image(&myself, data, &image_info);
        myself
    }

    pub(crate) fn change_layout(&mut self, cmd: &mut CmdBuffer, new_layout: vk::VkImageLayout) {
        let mut dst_stage = vk::VkPipelineStageFlagBits::VK_PIPELINE_STAGE_TRANSFER_BIT as u32;
        let mut barrier = vk::VkImageMemoryBarrier::default();
        barrier.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER;
        barrier.srcQueueFamilyIndex = vk::VK_QUEUE_FAMILY_IGNORED;
        barrier.dstQueueFamilyIndex = vk::VK_QUEUE_FAMILY_IGNORED;
        barrier.oldLayout = self.layout;
        barrier.newLayout = new_layout;
        barrier.image = self.vk_data;
        barrier.subresourceRange.aspectMask =
            vk::VkImageAspectFlagBits::VK_IMAGE_ASPECT_COLOR_BIT as u32;
        barrier.subresourceRange.layerCount = 1;
        barrier.subresourceRange.levelCount = self.mips_count as u32;
        barrier.srcAccessMask = match self.layout {
            vk::VkImageLayout::VK_IMAGE_LAYOUT_UNDEFINED => 0u32,
            vk::VkImageLayout::VK_IMAGE_LAYOUT_PREINITIALIZED => {
                vk::VkAccessFlagBits::VK_ACCESS_HOST_WRITE_BIT as u32
            }
            vk::VkImageLayout::VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL => {
                vk::VkAccessFlagBits::VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT as u32
            }
            vk::VkImageLayout::VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL => {
                vk::VkAccessFlagBits::VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT as u32
            }
            vk::VkImageLayout::VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL => {
                vk::VkAccessFlagBits::VK_ACCESS_TRANSFER_READ_BIT as u32
            }
            vk::VkImageLayout::VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL => {
                vk::VkAccessFlagBits::VK_ACCESS_TRANSFER_WRITE_BIT as u32
            }
            vk::VkImageLayout::VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL => {
                vk::VkAccessFlagBits::VK_ACCESS_SHADER_READ_BIT as u32
            }
            _ => vxunexpected!(),
        };
        barrier.dstAccessMask = match new_layout {
            vk::VkImageLayout::VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL => {
                vk::VkAccessFlagBits::VK_ACCESS_TRANSFER_WRITE_BIT as u32
            }
            vk::VkImageLayout::VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL => {
                vk::VkAccessFlagBits::VK_ACCESS_TRANSFER_READ_BIT as u32
            }
            vk::VkImageLayout::VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL => {
                vk::VkAccessFlagBits::VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT as u32
            }
            vk::VkImageLayout::VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL => {
                vk::VkAccessFlagBits::VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT as u32
            }
            vk::VkImageLayout::VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL => {
                dst_stage =
                    vk::VkPipelineStageFlagBits::VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT as u32;
                if barrier.srcAccessMask == 0u32 {
                    barrier.srcAccessMask = vk::VkAccessFlagBits::VK_ACCESS_HOST_WRITE_BIT as u32
                        | vk::VkAccessFlagBits::VK_ACCESS_TRANSFER_WRITE_BIT as u32;
                }
                vk::VkAccessFlagBits::VK_ACCESS_SHADER_READ_BIT as u32
            }
            _ => vxunexpected!(),
        };
        let src_stage = vk::VkPipelineStageFlagBits::VK_PIPELINE_STAGE_TRANSFER_BIT as u32;
        cmd.pipeline_image_barrier(src_stage, dst_stage, 0, &barrier);
        self.layout = new_layout;
    }

    pub fn get_dimensions(&self) -> (u32, u32) {
        return (self.width, self.height);
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        if self.memory.is_some() {
            unsafe {
                vk::vkDestroyImage(self.logical_device.vk_data, self.vk_data, null());
            }
        }
    }
}

unsafe impl Send for Image {}

unsafe impl Sync for Image {}

#[cfg_attr(debug_mode, derive(Debug))]
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
        Self::new_surface_attachment(
            logical_device,
            memory_mgr,
            depth_format,
            vk::VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT,
            AttachmentType::DepthStencilDisplay,
        )
    }

    pub fn new_with_vk_image(
        logical_device: Arc<LogicalDevice>,
        vk_image: vk::VkImage,
        format: vk::VkFormat,
        layout: vk::VkImageLayout,
        usage: vk::VkImageUsageFlags,
        width: u32,
        height: u32,
        samples: vk::VkSampleCountFlagBits,
    ) -> Self {
        Self::new_with_image(Arc::new(RwLock::new(Image::new_with_vk_data(
            logical_device,
            vk_image,
            layout,
            format,
            usage,
            width,
            height,
            samples,
        ))))
    }

    pub(crate) fn new_texture_2d_with_pixels(
        width: u32,
        height: u32,
        data: &[u8],
        buffmgr: &Arc<RwLock<BufferManager>>,
    ) -> Self {
        let image = Image::new_2d_with_pixels(width, height, data, buffmgr);
        Self::new_with_image(image)
    }

    pub fn new_with_image(image: Arc<RwLock<Image>>) -> Self {
        return Self::new_with_image_aspect(
            image,
            vk::VkImageAspectFlagBits::VK_IMAGE_ASPECT_COLOR_BIT as u32,
        );
    }

    pub fn new_with_image_aspect(image: Arc<RwLock<Image>>, aspect_mask: u32) -> Self {
        let mut vk_data = 0 as vk::VkImageView;
        {
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
            view_create_info.subresourceRange.aspectMask = aspect_mask;
            view_create_info.subresourceRange.levelCount = img.mips_count as u32;
            view_create_info.subresourceRange.layerCount = 1;
            vulkan_check!(vk::vkCreateImageView(
                dev.vk_data,
                &view_create_info,
                null(),
                &mut vk_data,
            ));
        }
        View { image, vk_data }
    }

    pub fn new_surface_attachment(
        logical_device: Arc<LogicalDevice>,
        memory_mgr: &Arc<RwLock<MemeoryManager>>,
        format: vk::VkFormat,
        samples: vk::VkSampleCountFlagBits,
        attachment_type: AttachmentType,
    ) -> Self {
        let surface_caps = logical_device.physical_device.surface_caps;
        return Self::new_attachment(
            logical_device,
            memory_mgr,
            format,
            samples,
            attachment_type,
            surface_caps.currentExtent.width,
            surface_caps.currentExtent.height,
        );
    }

    pub fn new_attachment(
        logical_device: Arc<LogicalDevice>,
        memory_mgr: &Arc<RwLock<MemeoryManager>>,
        format: vk::VkFormat,
        samples: vk::VkSampleCountFlagBits,
        attachment_type: AttachmentType,
        width: u32,
        height: u32,
    ) -> Self {
        let aspect_mask = match attachment_type {
            AttachmentType::ColorGBuffer
            | AttachmentType::ColorDisplay
            | AttachmentType::ResolverBuffer => {
                vk::VkImageAspectFlagBits::VK_IMAGE_ASPECT_COLOR_BIT as u32
            }
            AttachmentType::DepthGBuffer | AttachmentType::DepthShadowBuffer => {
                vk::VkImageAspectFlagBits::VK_IMAGE_ASPECT_DEPTH_BIT as u32
            }
            AttachmentType::DepthStencilDisplay => {
                vk::VkImageAspectFlagBits::VK_IMAGE_ASPECT_DEPTH_BIT as u32
                    | vk::VkImageAspectFlagBits::VK_IMAGE_ASPECT_STENCIL_BIT as u32
            }
        };
        let usage = match attachment_type {
            AttachmentType::ColorGBuffer => {
                vk::VkImageUsageFlagBits::VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT as u32
                    | vk::VkImageUsageFlagBits::VK_IMAGE_USAGE_SAMPLED_BIT as u32
            }
            AttachmentType::ColorDisplay | AttachmentType::ResolverBuffer => {
                vk::VkImageUsageFlagBits::VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT as u32
            }
            AttachmentType::DepthGBuffer => {
                vk::VkImageUsageFlagBits::VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT as u32
                    | vk::VkImageUsageFlagBits::VK_IMAGE_USAGE_SAMPLED_BIT as u32
            }
            AttachmentType::DepthStencilDisplay | AttachmentType::DepthShadowBuffer => {
                vk::VkImageUsageFlagBits::VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT as u32
            }
        };

        let mut image_info = vk::VkImageCreateInfo::default();
        image_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO;
        image_info.imageType = vk::VkImageType::VK_IMAGE_TYPE_2D;
        image_info.format = format;
        image_info.extent.width = width;
        image_info.extent.height = height;
        image_info.extent.depth = 1;
        image_info.mipLevels = 1;
        image_info.arrayLayers = 1;
        image_info.tiling = vk::VkImageTiling::VK_IMAGE_TILING_OPTIMAL;
        image_info.usage = usage;
        image_info.initialLayout = vk::VkImageLayout::VK_IMAGE_LAYOUT_UNDEFINED;
        image_info.samples = samples;
        let image = Arc::new(RwLock::new(Image::new_with_info(&image_info, memory_mgr)));
        return Self::new_with_image_aspect(image, aspect_mask);
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

unsafe impl Send for View {}

unsafe impl Sync for View {}
