use std::ptr::null;
use std::sync::{Arc, RwLock};
use super::vulkan as vk;
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
    pub pool: Arc<Pool>,
    pub layout: Vec<vk::VkDescriptorSetLayout>,
    pub vk_data: vk::VkDescriptorSet,
}

impl Set {
    fn new(
        pool: Arc<Pool>,
        buffer_info: vk::VkDescriptorBufferInfo,
    ) -> Self {
        let logical_device = pool.logical_device.clone();
        let mut layout_binding = vk::VkDescriptorSetLayoutBinding::default();
        layout_binding.descriptorType = vk::VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER;
        layout_binding.descriptorCount = 1;
        layout_binding.stageFlags = vk::VkShaderStageFlagBits::VK_SHADER_STAGE_VERTEX_BIT as u32;
        let layout_bindings = vec![layout_binding];
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
        alloc_info.descriptorPool = pool.vk_data;
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
    pub main_set: Set,
    pub buffer_manager: Arc<RwLock<BufferManager>>,
    pub pool: Arc<Pool>,
}

impl Manager {
    pub fn new(
        buffer_manager: Arc<RwLock<BufferManager>>,
        logical_device: Arc<LogicalDevice>,
    ) -> Self {
        let pool = Arc::new(Pool::new(logical_device));
        let buffer = vxresult!(buffer_manager.read()).cpu_buffer.vk_data;
        let mut buff_info = vk::VkDescriptorBufferInfo::default();
        buff_info.buffer = buffer;
        buff_info.range = 64; // temporary
        let main_set = Set::new(pool.clone(), buff_info);
        Manager {
            main_set,
            pool,
            buffer_manager,
        }
    }
}