pub mod view;

use super::buffer::Manager as BufferManager;
use super::command::buffer::Buffer as CmdBuffer;
use super::device::logical::Logical as LogicalDevice;
use super::memory::{Location as MemeoryLocation, Manager as MemeoryManager, Memory};
use super::vulkan as vk;
use image;

use std::ptr::null;
use std::sync::{Arc, RwLock};

pub struct Image {
    pub logical_device: Arc<LogicalDevice>,
    pub vk_data: vk::VkImage,
    pub layout: vk::VkImageLayout,
    pub mips_count: u8,
    pub memory: Option<Arc<RwLock<Memory>>>,
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
            mips_count: info.mipLevels as u8,
            memory: Some(memory),
        }
    }

    pub fn new_with_vk_data(
        logical_device: Arc<LogicalDevice>, 
        vk_image: vk::VkImage, 
        layout: vk::VkImageLayout,
    ) -> Self {
        Image {
            logical_device,
            layout,
            mips_count: 0,
            vk_data: vk_image,
            memory: None,
        }
    }

    pub fn new_with_file_name(file: &str, buffmgr: &Arc<RwLock<BufferManager>>) -> Arc<RwLock<Self>> {
        let img = vxresult!(image::open(file)).to_rgba(); // todo: issue #8
        let (width, height) = img.dimensions();
        let img: Vec<u8> = img.into_raw();
        let mut image_info = vk::VkImageCreateInfo::default();
        image_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO;
        image_info.imageType = vk::VkImageType::VK_IMAGE_TYPE_2D;
        image_info.format = vk::VkFormat::VK_FORMAT_R8G8B8A8_UNORM; // todo: issue #8
        image_info.extent.width = width;
        image_info.extent.height = height;
        image_info.extent.depth = 1;
        image_info.mipLevels = 1;
        image_info.arrayLayers = 1;
        image_info.samples = vk::VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT;
        image_info.tiling = vk::VkImageTiling::VK_IMAGE_TILING_OPTIMAL;
        image_info.initialLayout = vk::VkImageLayout::VK_IMAGE_LAYOUT_UNDEFINED;
        image_info.sharingMode = vk::VkSharingMode::VK_SHARING_MODE_EXCLUSIVE;
        image_info.usage = vk::VkImageUsageFlagBits::VK_IMAGE_USAGE_TRANSFER_DST_BIT as u32 |
            vk::VkImageUsageFlagBits::VK_IMAGE_USAGE_SAMPLED_BIT as u32;
        let memmgr = {
            let buffmgr = vxresult!(buffmgr.read());
            let memmgr = vxresult!(buffmgr.gpu_buffer.memory.read()).manager.clone();
            memmgr
        };
        let myself = Arc::new(RwLock::new(Self::new_with_info(&image_info, &memmgr)));
        vxresult!(buffmgr.write()).create_staging_image(&myself, &img, &image_info);
        myself
    }

    pub fn change_layout(&mut self, cmd: &mut CmdBuffer, new_layout: vk::VkImageLayout) {
        let mut dst_stage = vk::VkPipelineStageFlagBits::VK_PIPELINE_STAGE_TRANSFER_BIT as u32;
        let mut barrier = vk::VkImageMemoryBarrier::default();
        barrier.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER;
		barrier.srcQueueFamilyIndex = vk::VK_QUEUE_FAMILY_IGNORED;
		barrier.dstQueueFamilyIndex = vk::VK_QUEUE_FAMILY_IGNORED;
		barrier.oldLayout = self.layout;
		barrier.newLayout = new_layout;
		barrier.image = self.vk_data;
		barrier.subresourceRange.aspectMask = vk::VkImageAspectFlagBits::VK_IMAGE_ASPECT_COLOR_BIT as u32;
        barrier.subresourceRange.layerCount = 1;
        barrier.subresourceRange.levelCount = self.mips_count as u32;
		barrier.srcAccessMask = match self.layout {
			vk::VkImageLayout::VK_IMAGE_LAYOUT_UNDEFINED => 
                0u32,
		    vk::VkImageLayout::VK_IMAGE_LAYOUT_PREINITIALIZED => 
                vk::VkAccessFlagBits::VK_ACCESS_HOST_WRITE_BIT as u32,
			vk::VkImageLayout::VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL => 
                vk::VkAccessFlagBits::VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT as u32,
			vk::VkImageLayout::VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL => 
                vk::VkAccessFlagBits::VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT as u32,
			vk::VkImageLayout::VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL => 
                vk::VkAccessFlagBits::VK_ACCESS_TRANSFER_READ_BIT as u32,
			vk::VkImageLayout::VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL => 
                vk::VkAccessFlagBits::VK_ACCESS_TRANSFER_WRITE_BIT as u32,
			vk::VkImageLayout::VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL => 
                vk::VkAccessFlagBits::VK_ACCESS_SHADER_READ_BIT as u32,
			_ => vxunexpected!(),
        };
        barrier.dstAccessMask = match new_layout {
			vk::VkImageLayout::VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL => 
                vk::VkAccessFlagBits::VK_ACCESS_TRANSFER_WRITE_BIT as u32,
			vk::VkImageLayout::VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL => 
                vk::VkAccessFlagBits::VK_ACCESS_TRANSFER_READ_BIT as u32,
			vk::VkImageLayout::VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL => 
                vk::VkAccessFlagBits::VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT as u32,
			vk::VkImageLayout::VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL => 
                vk::VkAccessFlagBits::VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT as u32,
			vk::VkImageLayout::VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL => {
                dst_stage = vk::VkPipelineStageFlagBits::VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT as u32;
				if barrier.srcAccessMask == 0u32 {
					barrier.srcAccessMask = 
                        vk::VkAccessFlagBits::VK_ACCESS_HOST_WRITE_BIT as u32 | 
                        vk::VkAccessFlagBits::VK_ACCESS_TRANSFER_WRITE_BIT as u32;
				}
                vk::VkAccessFlagBits::VK_ACCESS_SHADER_READ_BIT as u32
            },
			_ => vxunexpected!(),
		};
        let src_stage = vk::VkPipelineStageFlagBits::VK_PIPELINE_STAGE_TRANSFER_BIT as u32;
        cmd.pipeline_image_barrier(src_stage, dst_stage, 0, &barrier);
        self.layout = new_layout;
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
