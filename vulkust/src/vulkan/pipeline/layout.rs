use std::sync::Arc;
use std::ptr::null;
use std::default::Default;
use super::super::super::system::vulkan as vk;
use super::super::device::logical::Logical as LogicalDevice;
pub struct Layout {
    pub logical_device: Arc<LogicalDevice>,
    // TODO: make it a different module
    pub descriptor_set_layout: vk::VkDescriptorSetLayout,
    pub vk_data: vk::VkPipelineLayout,
}
impl Layout {
    pub fn new(logical_device: Arc<LogicalDevice>) -> Self {
        let mut descriptor_set_layout = 0 as vk::VkDescriptorSetLayout;
        let mut vk_data = 0 as vk::VkPipelineLayout;
        let mut layout_binding = vk::VkDescriptorSetLayoutBinding::default();
		layout_binding.descriptorType = vk::VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER;
		layout_binding.descriptorCount = 1;
		layout_binding.stageFlags = vk::VkShaderStageFlagBits::VK_SHADER_STAGE_VERTEX_BIT as u32;
		let mut descriptor_layout = vk::VkDescriptorSetLayoutCreateInfo::default();
		descriptor_layout.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO;
		descriptor_layout.bindingCount = 1;
		descriptor_layout.pBindings = &layout_binding;
        vulkan_check!(vk::vkCreateDescriptorSetLayout(
            logical_device.vk_data, &descriptor_layout, null(), &mut descriptor_set_layout));
		let mut pipeline_layout_create_info = vk::VkPipelineLayoutCreateInfo::default();
		pipeline_layout_create_info.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO;
		pipeline_layout_create_info.setLayoutCount = 1;
		pipeline_layout_create_info.pSetLayouts = &descriptor_set_layout;
        vulkan_check!(vk::vkCreatePipelineLayout(
            logical_device.vk_data, &pipeline_layout_create_info, null(), &mut vk_data));
        Layout {
            logical_device: logical_device,
            descriptor_set_layout: descriptor_set_layout,
            vk_data: vk_data,
        }
    }
}
impl Drop for Layout {
    fn drop(&mut self) {
        unsafe {
            vk::vkDestroyPipelineLayout(
                self.logical_device.vk_data, self.vk_data, null());
            vk::vkDestroyDescriptorSetLayout(
                self.logical_device.vk_data, self.descriptor_set_layout, null());
        }
    }
}
