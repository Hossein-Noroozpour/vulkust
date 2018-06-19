pub mod view;

use super::buffer::Manager as BufferManager;
use super::device::logical::Logical as LogicalDevice;
use super::memory::{Location as MemeoryLocation, Manager as MemeoryManager, Memory};
use super::vulkan as vk;
use image;

use std::ptr::null;
use std::sync::{Arc, RwLock};

pub struct Image {
    pub logical_device: Arc<LogicalDevice>,
    pub vk_data: vk::VkImage,
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
            memory: Some(memory),
        }
    }

    pub fn new_with_vk_data(logical_device: Arc<LogicalDevice>, vk_image: vk::VkImage) -> Self {
        Image {
            logical_device,
            vk_data: vk_image,
            memory: None,
        }
    }

    pub fn new_with_file_name(file: &str, buffmgr: &Arc<RwLock<BufferManager>>) {
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
        image_info.usage = vk::VkImageUsageFlagBits::VK_IMAGE_USAGE_TRANSFER_DST_BIT as u32;
        let memmgr = {
            let buffmgr = vxresult!(buffmgr.read());
            let memmgr = vxresult!(buffmgr.gpu_buffer.memory.read()).manager.clone();
            memmgr
        };
        let myself = Arc::new(Self::new_with_info(&image_info, &memmgr));
        vxresult!(buffmgr.write()).create_staging_image(&myself, &img, &image_info);
        // let upbuff = buffmgr.write();
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
