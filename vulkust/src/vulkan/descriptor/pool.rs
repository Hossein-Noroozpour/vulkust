use std::default::Default;
use std::ptr::null;
use std::sync::Arc;
use super::super::super::system::vulkan as vk;
use super::super::device::logical::Logical as LogicalDevice;
pub struct Pool {
    logical_device: Arc<LogicalDevice>,
    vk_data: vk::VkDescriptorPool,
}
impl Pool {
    pub fn new(logical_device: Arc<LogicalDevice>) -> Self {
		let mut type_counts = [vk::VkDescriptorPoolSize::default(); 1];
		type_counts[0].type_ = vk::VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER;
		type_counts[0].descriptorCount = 1;
		let mut descriptor_pool_info = vk::VkDescriptorPoolCreateInfo::default();
		descriptor_pool_info.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO;
		descriptor_pool_info.poolSizeCount = 1;
		descriptor_pool_info.pPoolSizes = type_counts.as_ptr();
		descriptor_pool_info.maxSets = 1;
        let mut vk_data = 0 as vk::VkDescriptorPool;
		vulkan_check!(vk::vkCreateDescriptorPool(
            logical_device.vk_data, &descriptor_pool_info, null(), &mut vk_data));
        Pool {
            logical_device: logical_device,
            vk_data: vk_data,
        }
    }
}
