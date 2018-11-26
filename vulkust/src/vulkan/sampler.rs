use super::super::render::sampler::Filter;
use super::device::Logical as LogicalDevice;
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
        return Self::new_with_filter(logical_device, Filter::Linear);
    }

    pub(crate) fn new_with_filter(logical_device: Arc<LogicalDevice>, f: Filter) -> Self {
        let filter = Self::convert_filter(f);
        let mut info = vk::VkSamplerCreateInfo::default();
        info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO;
        info.borderColor = vk::VkBorderColor::VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK;
        info.unnormalizedCoordinates = vk::VK_FALSE;
        info.compareOp = vk::VkCompareOp::VK_COMPARE_OP_ALWAYS;
        info.magFilter = filter;
        info.minFilter = filter;
        info.addressModeU = vk::VkSamplerAddressMode::VK_SAMPLER_ADDRESS_MODE_REPEAT;
        info.addressModeV = info.addressModeU;
        info.addressModeW = info.addressModeU;
        match f {
            Filter::Nearest => {
                info.anisotropyEnable = vk::VK_FALSE;
                info.maxAnisotropy = 0f32;
                info.compareEnable = vk::VK_FALSE;
                info.mipmapMode = vk::VkSamplerMipmapMode::VK_SAMPLER_MIPMAP_MODE_NEAREST;
                info.mipLodBias = 0f32;
                info.minLod = 0f32;
                info.maxLod = 0f32;
            }
            _ => {
                info.anisotropyEnable = vk::VK_TRUE;
                info.maxAnisotropy = logical_device
                    .get_physical()
                    .get_properties()
                    .limits
                    .maxSamplerAnisotropy;
                info.compareEnable = vk::VK_FALSE;
                info.mipmapMode = vk::VkSamplerMipmapMode::VK_SAMPLER_MIPMAP_MODE_LINEAR;
                info.mipLodBias = 0f32;
                info.minLod = 0f32;
                info.maxLod = 0f32;
            }
        }
        let mut vk_data = 0 as vk::VkSampler;
        vulkan_check!(vk::vkCreateSampler(
            logical_device.get_data(),
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

    pub(super) fn convert_filter(f: Filter) -> vk::VkFilter {
        match f {
            Filter::Nearest => vk::VkFilter::VK_FILTER_NEAREST,
            Filter::Linear => vk::VkFilter::VK_FILTER_LINEAR,
            Filter::Cube => vk::VkFilter::VK_FILTER_CUBIC_IMG,
        }
    }
}

impl Drop for Sampler {
    fn drop(&mut self) {
        unsafe {
            vk::vkDestroySampler(self.logical_device.get_data(), self.vk_data, null());
        }
    }
}
