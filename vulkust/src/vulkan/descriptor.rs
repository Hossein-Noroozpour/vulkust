use std::cell::RefCell;
use std::collections::BTreeMap;
use std::default::Default;
use std::ptr::null;
use std::sync::{Arc, Weak};
use super::super::render::shader::Id as ShaderId;
use super::super::system::vulkan as vk;
use super::buffer::Manager as BufferManager;
use super::device::logical::Logical as LogicalDevice;
use super::pipeline::layout::Layout as PipelineLayout;

pub struct Pool {
    pub logical_device: Arc<LogicalDevice>,
    pub vk_data: vk::VkDescriptorPool,
}

impl Pool {
    pub fn new(logical_device: Arc<LogicalDevice>) -> Self {
        let mut type_counts = [vk::VkDescriptorPoolSize::default(); 1];
        type_counts[0].type_ = vk::VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER;
        type_counts[0].descriptorCount = 1;
        let mut descriptor_pool_info = vk::VkDescriptorPoolCreateInfo::default();
        descriptor_pool_info.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO;
        descriptor_pool_info.poolSizeCount = type_counts.len() as u32;
        descriptor_pool_info.pPoolSizes = type_counts.as_ptr();
        descriptor_pool_info.maxSets = 1;
        let mut vk_data = 0 as vk::VkDescriptorPool;
        vulkan_check!(vk::vkCreateDescriptorPool(
            logical_device.vk_data,
            &descriptor_pool_info,
            null(),
            &mut vk_data,
        ));
        Pool {
            logical_device: logical_device,
            vk_data: vk_data,
        }
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        unsafe {
            vk::vkDestroyDescriptorPool(self.logical_device.vk_data, self.vk_data, null());
        }
    }
}

pub struct Set {
    pub vk_data: vk::VkDescriptorSet,
}

impl Set {
    fn new(
        pool: &Arc<Pool>,
        pipeline_layout: &Arc<PipelineLayout>,
        buffer_info: &vk::VkDescriptorBufferInfo,
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
        write_descriptor_set.pBufferInfo = buffer_info;
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
            vk_data: vk_data,
        }
    }
}

pub struct Manager {
    cached: BTreeMap<ShaderId, Weak<Set>>,
    pool: Arc<Pool>,
    pipeline_layout: Arc<PipelineLayout>,
    buffer: vk::VkBuffer,
}

impl Manager {
    pub fn new(
        pool: Arc<Pool>,
        pipeline_layout: Arc<PipelineLayout>,
        buffer_manager: &BufferManager) -> Self {
        Manager {
            cached: BTreeMap::new(),
            pool: pool,
            pipeline_layout: pipeline_layout,
            buffer: buffer_manager.get_buffer(),
        }
    }

    pub fn get(&mut self, id: ShaderId) -> Arc<Set> {
        match self.cached.get(&id) {
            Some(res) => match res.upgrade() {
                Some(res) => {
                    return res;
                }
                None => {}
            },
            None => {}
        }
        let mut buff_info = vk::VkDescriptorBufferInfo::default();
        buff_info.buffer = self.buffer;
        let set = Arc::new(Set::new(&self.pool, &self.pipeline_layout, &buff_info));
        self.cached.insert(id, Arc::downgrade(&set));
        return set;
    }
}