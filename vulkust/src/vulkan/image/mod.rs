pub mod view;

use super::super::system::vulkan::{
    VkImage,
    VkResult,
    VkFormat,
    VkExtent3D,
    VkImageType,
    vkFreeMemory,
    vkCreateImage,
    VkImageTiling,
    VkDeviceMemory,
    vkDestroyImage,
    VkStructureType,
    vkAllocateMemory,
    VkImageCreateInfo,
    vkBindImageMemory,
    VkMemoryRequirements,
    VkMemoryAllocateInfo,
    VkImageUsageFlagBits,
    VkSampleCountFlagBits,
    VkAllocationCallbacks,
    VkMemoryPropertyFlagBits,
    vkGetImageMemoryRequirements,
};

use super::device::Device;

use std::default::Default;
use std::sync::{
    Arc,
    RwLock,
};

pub struct Image {
    pub device: Arc<RwLock<Device>>,
    pub vk_image: VkImage,
    pub vk_format: VkFormat,
    pub vk_mem: VkDeviceMemory,
}

impl Image {
    pub fn new_depth_with_format(
        device: Arc<RwLock<Device>>, width: u32, height: u32) -> Self {
        let format = device.read().unwrap().vk_depth_format;
        Image::new_with_format_samples_tiling_usage(
            device, format, width, height, VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT,
            VkImageTiling::VK_IMAGE_TILING_OPTIMAL,
            (VkImageUsageFlagBits::VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT as u32) |
                (VkImageUsageFlagBits::VK_IMAGE_USAGE_TRANSFER_SRC_BIT as u32))
    }

    pub fn new_with_format_samples_tiling_usage(
        device: Arc<RwLock<Device>>, format: VkFormat, width: u32, height: u32,
        samples: VkSampleCountFlagBits, tiling: VkImageTiling,
        usage: u32) -> Self {
        let dev = device.read().unwrap();
        let mut image_ci = VkImageCreateInfo::default();
        image_ci.sType = VkStructureType::VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO;
        image_ci.imageType = VkImageType::VK_IMAGE_TYPE_2D;
        image_ci.format = format;
        image_ci.extent = VkExtent3D {
            width: width,
            height: height,
            depth: 1,
        };
        image_ci.mipLevels = 1;
        image_ci.arrayLayers = 1;
        image_ci.samples = samples;
        image_ci.tiling = tiling;
        image_ci.usage = usage;
        let mut vk_image = 0 as VkImage;
        vulkan_check!(vkCreateImage(
            dev.vk_device, &image_ci as *const VkImageCreateInfo,
            0 as *const VkAllocationCallbacks, &mut vk_image as *mut VkImage));
        let mut vk_mem_req = VkMemoryRequirements::default();
        unsafe {
            vkGetImageMemoryRequirements(
                dev.vk_device, vk_image, &mut vk_mem_req as *mut VkMemoryRequirements);
        }
        let mut mem_alloc = VkMemoryAllocateInfo::default();
        mem_alloc.sType = VkStructureType::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
        mem_alloc.allocationSize = vk_mem_req.size;
        mem_alloc.memoryTypeIndex = dev.choose_heap_from_flags(
            &vk_mem_req, VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as u32,
            VkMemoryPropertyFlagBits::VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT as u32); // !!!!
        let mut vk_mem = 0 as VkDeviceMemory;
        vulkan_check!(vkAllocateMemory(
            dev.vk_device, &mem_alloc as *const VkMemoryAllocateInfo,
            0 as *const VkAllocationCallbacks, &mut vk_mem as *mut VkDeviceMemory));
        vulkan_check!(vkBindImageMemory(dev.vk_device, vk_image, vk_mem, 0));
        Image {
            device: device.clone(),
            vk_image: vk_image,
            vk_format: format,
            vk_mem: vk_mem,
        }
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        let dev = self.device.read().unwrap();
        unsafe {
            vkFreeMemory(dev.vk_device, self.vk_mem,  0 as *const VkAllocationCallbacks);
            vkDestroyImage(dev.vk_device, self.vk_image, 0 as *const VkAllocationCallbacks);
        }
    }
}