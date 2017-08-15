use std::cell::RefCell;
use std::sync::Arc;
use std::default::Default;
use std::ptr::null;
use super::super::super::system::vulkan as vk;
use super::super::buffer::Manager as BufferManager;
use super::super::pipeline::layout::Layout as PipelineLayout;
use super::pool::Pool;
pub struct Set {
    pub pool: Arc<Pool>,
    pub pipeline_layout: Arc<PipelineLayout>,
    pub vk_data: vk::VkDescriptorSet,
}
impl Set {
    pub fn new(
        pool: Arc<Pool>,
        pipeline_layout: Arc<PipelineLayout>,
        buffer: &Arc<RefCell<BufferManager>>,
    ) -> Self {
        let mut alloc_info = vk::VkDescriptorSetAllocateInfo::default();
        alloc_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO;
        alloc_info.descriptorPool = pool.vk_data;
        alloc_info.descriptorSetCount = 1;
        alloc_info.pSetLayouts = &(pipeline_layout.descriptor_set_layout);
        let mut vk_data = 0 as vk::VkDescriptorSet;
        vulkan_check!(vk::vkAllocateDescriptorSets(
            pool.logical_device.vk_data,
            &alloc_info,
            &mut vk_data,
        ));
        let mut write_descriptor_set = vk::VkWriteDescriptorSet::default();
        write_descriptor_set.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
        write_descriptor_set.dstSet = vk_data;
        write_descriptor_set.descriptorCount = 1;
        write_descriptor_set.descriptorType =
            vk::VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER;
        write_descriptor_set.pBufferInfo = &(buffer.descriptor);
        write_descriptor_set.dstBinding = 0;
        unsafe {
            vk::vkUpdateDescriptorSets(
                pool.logical_device.vk_data,
                1,
                &write_descriptor_set,
                0,
                null(),
            );
        }
        Set {
            pool: pool,
            pipeline_layout: pipeline_layout,
            buffer: buffer,
            vk_data: vk_data,
        }
    }
}
// impl Drop for Set {
//     fn drop(&mut self) {
//         unsafe {
//         }
//     }
// }
