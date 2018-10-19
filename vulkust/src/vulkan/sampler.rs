use super::device::logical::Logical as LogicalDevice;
use super::vulkan as vk;
use std::ptr::null;
use std::sync::Arc;

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Sampler {
    logical_device: Arc<LogicalDevice>,
    vk_data: vk::VkSampler,
}

impl Sampler {
    pub(crate) fn new(logical_device: Arc<LogicalDevice>) -> Self {
        let mut info = vk::VkSamplerCreateInfo::default();
        info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO;
        info.magFilter = vk::VkFilter::VK_FILTER_LINEAR;
        info.minFilter = vk::VkFilter::VK_FILTER_LINEAR;
        info.addressModeU = vk::VkSamplerAddressMode::VK_SAMPLER_ADDRESS_MODE_REPEAT;
        info.addressModeV = info.addressModeU;
        info.addressModeW = info.addressModeU;
        info.anisotropyEnable = vk::VK_TRUE;
        info.maxAnisotropy = logical_device
            .physical_device
            .properties
            .limits
            .maxSamplerAnisotropy;
        info.borderColor = vk::VkBorderColor::VK_BORDER_COLOR_INT_OPAQUE_BLACK;
        info.unnormalizedCoordinates = vk::VK_FALSE;
        info.compareEnable = vk::VK_FALSE;
        info.compareOp = vk::VkCompareOp::VK_COMPARE_OP_ALWAYS;
        info.mipmapMode = vk::VkSamplerMipmapMode::VK_SAMPLER_MIPMAP_MODE_LINEAR;
        info.mipLodBias = 0f32;
        info.minLod = 0f32;
        info.maxLod = 0f32;
        let mut vk_data = 0 as vk::VkSampler;
        vulkan_check!(vk::vkCreateSampler(
            logical_device.vk_data,
            &info,
            null(),
            &mut vk_data
        ));
        Sampler {
            logical_device,
            vk_data,
        }
    }

    pub(crate) fn get_data(&self) -> vk::VkSampler {
        return self.vk_data;
    }
}

impl Drop for Sampler {
    fn drop(&mut self) {
        unsafe {
            vk::vkDestroySampler(self.logical_device.vk_data, self.vk_data, null());
        }
    }
}
