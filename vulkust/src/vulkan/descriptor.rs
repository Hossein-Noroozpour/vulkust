use std::default::Default;
use std::ptr::null;
use std::sync::Arc;
use super::super::render::shader::{
    BindingStage,
    Id as ShaderId, 
    ResourceType, 
    shader_id_resources,
    shader_uniform_size, 
};
use super::super::system::vulkan as vk;
use super::super::util::cache::Cacher;
use super::super::util::cell::DebugCell;
use super::buffer::Manager as BufferManager;
use super::device::logical::Logical as LogicalDevice;

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
    pool: Arc<DebugCell<Pool>>,
    pub layout: Vec<vk::VkDescriptorSetLayout>,
    pub vk_data: vk::VkDescriptorSet,
}

impl Set {
    fn new(
        sid: ShaderId,
        pool: Arc<DebugCell<Pool>>,
        buffer_info: vk::VkDescriptorBufferInfo,
    ) -> Self {
        let logical_device = pool.borrow().logical_device.clone();
        let shader_resources = shader_id_resources(sid);
        let mut layout_bindings = Vec::new();
        for r in shader_resources {
            let mut layout_binding = vk::VkDescriptorSetLayoutBinding::default();
            layout_binding.descriptorType = match r.2 { 
                ResourceType::Uniform => vk::VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,
            };
            layout_binding.descriptorCount = r.1;
            layout_binding.stageFlags = 0;
            for s in r.0 {
                match s {
                    BindingStage::Vertex =>
                        layout_binding.stageFlags |= 
                            vk::VkShaderStageFlagBits::VK_SHADER_STAGE_VERTEX_BIT as u32,
                    BindingStage::Fragment =>
                        layout_binding.stageFlags |= 
                            vk::VkShaderStageFlagBits::VK_SHADER_STAGE_FRAGMENT_BIT as u32,
                }
            }
            layout_bindings.push(layout_binding);
        }
        let mut descriptor_layout = vk::VkDescriptorSetLayoutCreateInfo::default();
        descriptor_layout.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO;
        descriptor_layout.bindingCount = layout_bindings.len() as u32;
        descriptor_layout.pBindings = layout_bindings.as_ptr();
        let mut descriptor_set_layout = 0 as vk::VkDescriptorSetLayout;
        vulkan_check!(vk::vkCreateDescriptorSetLayout(
            logical_device.vk_data,
            &descriptor_layout,
            null(),
            &mut descriptor_set_layout,
        ));
        let mut alloc_info = vk::VkDescriptorSetAllocateInfo::default();
        alloc_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO;
        alloc_info.descriptorPool = pool.borrow().vk_data;
        alloc_info.descriptorSetCount = 1;
        alloc_info.pSetLayouts = &descriptor_set_layout;
        let mut vk_data = 0 as vk::VkDescriptorSet;
        vulkan_check!(vk::vkAllocateDescriptorSets(
            logical_device.vk_data,
            &alloc_info,
            &mut vk_data,
        ));
        let mut write_descriptor_set = vk::VkWriteDescriptorSet::default();
        write_descriptor_set.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
        write_descriptor_set.dstSet = vk_data;
        write_descriptor_set.descriptorCount = 1;
        write_descriptor_set.descriptorType =
            vk::VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER;
        write_descriptor_set.pBufferInfo = &buffer_info;
        write_descriptor_set.dstBinding = 0;
        unsafe {
            vk::vkUpdateDescriptorSets(
                logical_device.vk_data,
                1,
                &write_descriptor_set,
                0,
                null(),
            );
        }
        Set {
            pool: pool,
            layout: vec![descriptor_set_layout; 1],
            vk_data: vk_data,
        }
    }
}

pub struct Manager {
    cached: Cacher<ShaderId, Set>,
    buffer_manager: Arc<DebugCell<BufferManager>>,
    pool: Arc<DebugCell<Pool>>,
}

impl Manager {
    pub fn new(
        buffer_manager: Arc<DebugCell<BufferManager>>) -> Self {
        let pool = Arc::new(DebugCell::new(Pool::new(buffer_manager.borrow().get_device().clone())));
        Manager {
            cached: Cacher::new(),
            pool: pool,
            buffer_manager: buffer_manager,
        }
    }

    pub fn get(&mut self, id: ShaderId) -> Arc<DebugCell<Set>> {
        let buffer = self.buffer_manager.borrow().get_buffer();
        let pool = self.pool.clone();
        self.cached.get(id, &|| {
            let mut buff_info = vk::VkDescriptorBufferInfo::default();
            buff_info.buffer = buffer;
            buff_info.range = shader_uniform_size(id) as vk::VkDeviceSize;
            Arc::new(DebugCell::new(Set::new(id, pool.clone(), buff_info)))
        })
    }
}