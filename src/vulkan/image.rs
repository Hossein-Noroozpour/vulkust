use super::super::system::vulkan::{
    VkResult,
    VkFormat,
    VkExtent3D,
    VkImageType,
    VkImageTiling,
    VkStructureType,
    VkImageCreateInfo,
    VkMemoryAllocateInfo,
    VkImageUsageFlagBits,
    VkSampleCountFlagBits,
};

use std::default::Default;

pub struct Image {

}

impl Image {
    pub fn new_with_format(format: VkFormat, width: u32, height: u32) -> Self {
        Image::new_with_format_samples_tiling_usage(
            format, width, height, VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT,
            VkImageTiling::VK_IMAGE_TILING_OPTIMAL,
            (VkImageUsageFlagBits::VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT as u32) |
                (VkImageUsageFlagBits::VK_IMAGE_USAGE_TRANSFER_SRC_BIT as u32))
    }

    pub fn new_with_format_samples_tiling_usage(
        format: VkFormat, width: u32, height: u32, samples: VkSampleCountFlagBits,
        tiling: VkImageTiling, usage: VkImageUsageFlagBits) -> Self {
        let mut image = VkImageCreateInfo::default();
        image.sType = VkStructureType::VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO;
        image.imageType = VkImageType::VK_IMAGE_TYPE_2D;
        image.format = format;
        image.extent = VkExtent3D {
            width: width,
            height: height,
            depth: 1,
        };
        image.mipLevels = 1;
        image.arrayLayers = 1;
        image.samples = samples;
        image.tiling = tiling;
        image.usage = usage;

        let mem_alloc = VkMemoryAllocateInfo::default();

        mem_alloc.sType = VkStructureType::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
        VK_CHECK_RESULT(vkCreateImage(device, &image, nullptr, &depthStencil.image));







        VkImageViewCreateInfo depthStencilView = {};
        depthStencilView.sType = VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO;
        depthStencilView.pNext = NULL;
        depthStencilView.viewType = VK_IMAGE_VIEW_TYPE_2D;
        depthStencilView.format = depthFormat;
        depthStencilView.flags = 0;
        depthStencilView.subresourceRange = {};
        depthStencilView.subresourceRange.aspectMask = VK_IMAGE_ASPECT_DEPTH_BIT | VK_IMAGE_ASPECT_STENCIL_BIT;
        depthStencilView.subresourceRange.baseMipLevel = 0;
        depthStencilView.subresourceRange.levelCount = 1;
        depthStencilView.subresourceRange.baseArrayLayer = 0;
        depthStencilView.subresourceRange.layerCount = 1;

        VkMemoryRequirements memReqs;

        vkGetImageMemoryRequirements(device, depthStencil.image, &memReqs);
        mem_alloc.allocationSize = memReqs.size;
        mem_alloc.memoryTypeIndex = vulkanDevice->getMemoryType(memReqs.memoryTypeBits, VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT);
        VK_CHECK_RESULT(vkAllocateMemory(device, &mem_alloc, nullptr, &depthStencil.mem));
        VK_CHECK_RESULT(vkBindImageMemory(device, depthStencil.image, depthStencil.mem, 0));

        depthStencilView.image = depthStencil.image;
        VK_CHECK_RESULT(vkCreateImageView(device, &depthStencilView, nullptr, &depthStencil.view));
    }
}
