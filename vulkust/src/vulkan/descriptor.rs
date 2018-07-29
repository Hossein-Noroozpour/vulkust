use super::buffer::Manager as BufferManager;
use super::device::logical::Logical as LogicalDevice;
use super::image::View as ImageView;
use super::sampler::Sampler;
use super::vulkan as vk;
use super::super::render::config::Configurations;
use std::ptr::null;
use std::sync::{Arc, RwLock};

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Pool {
    pub logical_device: Arc<LogicalDevice>,
    pub vk_data: vk::VkDescriptorPool,
}

impl Pool {
    pub fn new(logical_device: Arc<LogicalDevice>, conf: &Configurations) -> Self {
        let buffers_count = conf.max_number_mesh + conf.max_number_models + conf.max_number_scene;
        let mut type_counts = [vk::VkDescriptorPoolSize::default(); 2];
        type_counts[0].type_ = vk::VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC;
        type_counts[0].descriptorCount = buffers_count as u32;
        type_counts[1].type_ = vk::VkDescriptorType::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER;
        type_counts[1].descriptorCount = conf.max_number_texture as u32;
        let mut descriptor_pool_info = vk::VkDescriptorPoolCreateInfo::default();
        descriptor_pool_info.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO;
        descriptor_pool_info.poolSizeCount = type_counts.len() as u32;
        descriptor_pool_info.pPoolSizes = type_counts.as_ptr();
        descriptor_pool_info.maxSets = buffers_count as u32 + 1; // todo find a better solution for this
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

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct SetLayout {
    pub logical_device: Arc<LogicalDevice>,
    pub vk_data: vk::VkDescriptorSetLayout,
}

impl SetLayout {
    fn new(logical_device: Arc<LogicalDevice>) -> Self {
        let mut layout_bindings = vec![vk::VkDescriptorSetLayoutBinding::default(); 2];
        layout_bindings[0].binding = 0;
        layout_bindings[0].descriptorType =
            vk::VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC;
        layout_bindings[0].descriptorCount = 1;
        layout_bindings[0].stageFlags =
            vk::VkShaderStageFlagBits::VK_SHADER_STAGE_VERTEX_BIT as u32;
        layout_bindings[1].binding = 1;
        layout_bindings[1].descriptorType =
            vk::VkDescriptorType::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER;
        layout_bindings[1].descriptorCount = 1;
        layout_bindings[1].stageFlags =
            vk::VkShaderStageFlagBits::VK_SHADER_STAGE_FRAGMENT_BIT as u32;
        let mut descriptor_layout = vk::VkDescriptorSetLayoutCreateInfo::default();
        descriptor_layout.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO;
        descriptor_layout.bindingCount = layout_bindings.len() as u32;
        descriptor_layout.pBindings = layout_bindings.as_ptr();
        let mut vk_data = 0 as vk::VkDescriptorSetLayout;
        vulkan_check!(vk::vkCreateDescriptorSetLayout(
            logical_device.vk_data,
            &descriptor_layout,
            null(),
            &mut vk_data,
        ));
        SetLayout {
            logical_device,
            vk_data,
        }
    }
}

impl Drop for SetLayout {
    fn drop(&mut self) {
        unsafe {
            vk::vkDestroyDescriptorSetLayout(self.logical_device.vk_data, self.vk_data, null());
        }
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Set {
    pub pool: Arc<Pool>,
    pub layout: Arc<SetLayout>,
    pub vk_data: vk::VkDescriptorSet,
}

impl Set {
    fn new(
        pool: &Arc<Pool>,
        layout: &Arc<SetLayout>,
        buffer_manager: &Arc<RwLock<BufferManager>>,
        image_view: &Arc<ImageView>,
        sampler: &Arc<Sampler>,
    ) -> Self {
        let layout = layout.clone();
        let logical_device: &Arc<LogicalDevice> = &pool.logical_device;
        let pool = pool.clone();
        let mut alloc_info = vk::VkDescriptorSetAllocateInfo::default();
        alloc_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO;
        alloc_info.descriptorPool = pool.vk_data;
        alloc_info.descriptorSetCount = 1;
        alloc_info.pSetLayouts = &layout.vk_data;
        let mut vk_data = 0 as vk::VkDescriptorSet;
        vulkan_check!(vk::vkAllocateDescriptorSets(
            logical_device.vk_data,
            &alloc_info,
            &mut vk_data,
        ));
        let buffer = vxresult!(buffer_manager.read()).cpu_buffer.vk_data;
        let mut buff_info = vk::VkDescriptorBufferInfo::default();
        buff_info.buffer = buffer;
        buff_info.range = 64; // todo: temporary
        let mut img_info = vk::VkDescriptorImageInfo::default();
        img_info.imageLayout = vk::VkImageLayout::VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL;
        img_info.imageView = image_view.vk_data;
        img_info.sampler = sampler.vk_data;
        let mut infos = vec![vk::VkWriteDescriptorSet::default(); 2];
        infos[0].sType = vk::VkStructureType::VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
        infos[0].dstSet = vk_data;
        infos[0].descriptorCount = 1;
        infos[0].descriptorType = vk::VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC;
        infos[0].pBufferInfo = &buff_info;
        infos[0].dstBinding = 0;
        infos[1].sType = vk::VkStructureType::VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
        infos[1].dstSet = vk_data;
        infos[1].descriptorCount = 1;
        infos[1].descriptorType = vk::VkDescriptorType::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER;
        infos[1].pImageInfo = &img_info;
        infos[1].dstBinding = 1;
        unsafe {
            vk::vkUpdateDescriptorSets(
                logical_device.vk_data,
                infos.len() as u32,
                infos.as_ptr(),
                0,
                null(),
            );
        }
        Set {
            pool,
            layout,
            vk_data,
        }
    }
}

impl Drop for Set {
    fn drop(&mut self) {}
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Manager {
    pub buffer_manager: Arc<RwLock<BufferManager>>,
    pub main_set_layout: Arc<SetLayout>,
    pub pool: Arc<Pool>,
}

impl Manager {
    pub fn new(
        buffer_manager: &Arc<RwLock<BufferManager>>,
        logical_device: &Arc<LogicalDevice>,
        conf: &Configurations,
    ) -> Self {
        let pool = Arc::new(Pool::new(logical_device.clone(), conf));
        let main_set_layout = Arc::new(SetLayout::new(logical_device.clone()));
        let buffer_manager = buffer_manager.clone();
        Manager {
            buffer_manager,
            main_set_layout,
            pool,
        }
    }

    pub fn create_main_set(&mut self, image_view: &Arc<ImageView>, sampler: &Arc<Sampler>) -> Set {
        Set::new(
            &self.pool,
            &self.main_set_layout,
            &self.buffer_manager,
            image_view,
            sampler,
        )
    }
}
