use super::super::core::allocate::Object as AlcObject;
use super::super::render::config::Configurations;
use super::super::render::texture::Texture;
use super::buffer::{DynamicBuffer, Manager as BufferManager};
use super::device::logical::Logical as LogicalDevice;
use super::vulkan as vk;
use std::ptr::null;
use std::sync::{Arc, RwLock, Mutex};

#[cfg_attr(debug_mode, derive(Debug))]
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
        descriptor_pool_info.maxSets = buffers_count as u32 + 1; // todo I must find the exact number after everything got stable
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

#[cfg_attr(debug_mode, derive(Debug))]
pub struct SetLayout {
    pub logical_device: Arc<LogicalDevice>,
    pub vk_data: vk::VkDescriptorSetLayout,
}

impl SetLayout {
    pub fn new_gbuff(logical_device: Arc<LogicalDevice>) -> Self {
        let layout_bindings = Self::create_binding_info(7);
        return Self::new_with_bindings_info(logical_device, &layout_bindings);
    }

    pub fn new_buffer_only(logical_device: Arc<LogicalDevice>) -> Self {
        let layout_bindings = Self::create_binding_info(0);
        return Self::new_with_bindings_info(logical_device, &layout_bindings);
    }

    pub fn new_deferred(logical_device: Arc<LogicalDevice>) -> Self {
        let layout_bindings = Self::create_binding_info(4);
        return Self::new_with_bindings_info(logical_device, &layout_bindings);
    }

    fn new_with_bindings_info(
        logical_device: Arc<LogicalDevice>,
        layout_bindings: &Vec<vk::VkDescriptorSetLayoutBinding>,
    ) -> Self {
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

    fn create_binding_info(images_count: usize) -> Vec<vk::VkDescriptorSetLayoutBinding> {
        let mut layout_bindings =
            vec![vk::VkDescriptorSetLayoutBinding::default(); 1 + images_count];
        layout_bindings[0].binding = 0;
        layout_bindings[0].descriptorType =
            vk::VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC;
        layout_bindings[0].descriptorCount = 1;
        layout_bindings[0].stageFlags = vk::VkShaderStageFlagBits::VK_SHADER_STAGE_VERTEX_BIT
            as u32
            | vk::VkShaderStageFlagBits::VK_SHADER_STAGE_FRAGMENT_BIT as u32;
        for i in 1..(images_count + 1) {
            layout_bindings[i].binding = i as u32;
            layout_bindings[i].descriptorCount = 1;
            layout_bindings[i].descriptorType =
                vk::VkDescriptorType::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER;
            layout_bindings[i].stageFlags =
                vk::VkShaderStageFlagBits::VK_SHADER_STAGE_FRAGMENT_BIT as u32;
        }
        return layout_bindings;
    }
}

impl Drop for SetLayout {
    fn drop(&mut self) {
        unsafe {
            vk::vkDestroyDescriptorSetLayout(self.logical_device.vk_data, self.vk_data, null());
        }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Set {
    pub pool: Arc<Pool>,
    pub layout: Arc<SetLayout>,
    pub uniform: Arc<RwLock<DynamicBuffer>>,
    pub textures: Vec<Arc<RwLock<Texture>>>,
    pub vk_data: vk::VkDescriptorSet,
}

impl Set {
    pub fn new_buffer_only(
        pool: Arc<Pool>,
        layout: Arc<SetLayout>,
        uniform: Arc<Mutex<DynamicBuffer>>,
        buffer_manager: &Arc<RwLock<BufferManager>>,
    ) -> Self {
        Self::new(pool, layout, uniform, buffer_manager, Vec::new())
    }

    pub fn new_gbuff(
        pool: Arc<Pool>,
        layout: Arc<SetLayout>,
        uniform: Arc<Mutex<DynamicBuffer>>,
        buffer_manager: &Arc<RwLock<BufferManager>>,
        textures: Vec<Arc<RwLock<Texture>>>,
    ) -> Self {
        #[cfg(debug_mode)]
        {
            if textures.len() != 7 {
                vxlogf!("For gbuffer filler descriptor you need 7 textures.");
            }
        }
        Self::new(pool, layout, uniform, buffer_manager, textures)
    }

    pub fn new_deferred(
        pool: Arc<Pool>,
        layout: Arc<SetLayout>,
        uniform: Arc<Mutex<DynamicBuffer>>,
        buffer_manager: &Arc<RwLock<BufferManager>>,
        textures: Vec<Arc<RwLock<Texture>>>,
    ) -> Self {
        #[cfg(debug_mode)]
        {
            if textures.len() != 4 {
                vxlogf!("For deferred descriptor you need 4 textures.");
            }
        }
        Self::new(pool, layout, uniform, buffer_manager, textures)
    }

    fn create_buffer_info(
        uniform: &DynamicBuffer,
        buffer_manager: &Arc<RwLock<BufferManager>>,
    ) -> vk::VkDescriptorBufferInfo {
        let mut buff_info = vk::VkDescriptorBufferInfo::default();
        buff_info.buffer = vxresult!(buffer_manager.read()).cpu_buffer.vk_data;
        buff_info.range = vxresult!(uniform.buffers[0].0.read()).get_size() as vk::VkDeviceSize;
        // for offset: it is dynamic uniform buffer, it will be fill later
        return buff_info;
    }

    fn allocate_set(pool: &Arc<Pool>, layout: &Arc<SetLayout>) -> vk::VkDescriptorSet {
        let mut alloc_info = vk::VkDescriptorSetAllocateInfo::default();
        alloc_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO;
        alloc_info.descriptorPool = pool.vk_data;
        alloc_info.descriptorSetCount = 1;
        alloc_info.pSetLayouts = &layout.vk_data;
        let mut vk_data = 0 as vk::VkDescriptorSet;
        vulkan_check!(vk::vkAllocateDescriptorSets(
            pool.logical_device.vk_data,
            &alloc_info,
            &mut vk_data,
        ));
        return vk_data;
    }

    fn new(
        pool: Arc<Pool>,
        layout: Arc<SetLayout>,
        uniform: Arc<RwLock<DynamicBuffer>>,
        buffer_manager: &Arc<RwLock<BufferManager>>,
        textures: Vec<Arc<RwLock<Texture>>>,
    ) -> Self {
        let vk_data = Self::allocate_set(&pool, &layout);
        let buff_info = Self::create_buffer_info(&*vxresult!(uniform.read()), buffer_manager);
        let mut infos = vec![vk::VkWriteDescriptorSet::default(); 1 + textures.len()];
        infos[0].sType = vk::VkStructureType::VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
        infos[0].dstSet = vk_data;
        infos[0].descriptorCount = 1;
        infos[0].descriptorType = vk::VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC;
        infos[0].pBufferInfo = &buff_info;
        infos[0].dstBinding = 0;
        let mut last_info_i = 1;
        let mut last_img_info_i = 0;
        let mut img_infos = vec![vk::VkDescriptorImageInfo::default(); textures.len()];
        for texture in &textures {
            let texture = vxresult!(texture.read());
            img_infos[last_img_info_i].imageLayout =
                vk::VkImageLayout::VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL;
            img_infos[last_img_info_i].imageView = texture.get_image_view().vk_data;
            img_infos[last_img_info_i].sampler = texture.get_sampler().vk_data;
            infos[last_info_i].sType = vk::VkStructureType::VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
            infos[last_info_i].dstSet = vk_data;
            infos[last_info_i].descriptorCount = 1;
            infos[last_info_i].descriptorType =
                vk::VkDescriptorType::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER;
            infos[last_info_i].pImageInfo = &(img_infos[last_img_info_i]);
            infos[last_info_i].dstBinding = last_info_i as u32;
            last_info_i += 1;
            last_img_info_i += 1;
        }
        unsafe {
            vk::vkUpdateDescriptorSets(
                pool.logical_device.vk_data,
                infos.len() as u32,
                infos.as_ptr(),
                0,
                null(),
            );
        }
        Set {
            pool,
            layout,
            uniform,
            textures,
            vk_data,
        }
    }
}

impl Drop for Set {
    fn drop(&mut self) {}
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Manager {
    pub buffer_manager: Arc<RwLock<BufferManager>>,
    pub gbuff_set_layout: Arc<SetLayout>,
    pub buffer_only_set_layout: Arc<SetLayout>,
    pub deferred_set_layout: Arc<SetLayout>,
    pub pool: Arc<Pool>,
}

// todo it can in future cache the sets based on their buffer id and size and texture ids and samplers

impl Manager {
    pub fn new(
        buffer_manager: &Arc<RwLock<BufferManager>>,
        logical_device: &Arc<LogicalDevice>,
        conf: &Configurations,
    ) -> Self {
        let pool = Arc::new(Pool::new(logical_device.clone(), conf));
        let gbuff_set_layout = Arc::new(SetLayout::new_gbuff(logical_device.clone()));
        let buffer_only_set_layout = Arc::new(SetLayout::new_buffer_only(logical_device.clone()));
        let deferred_set_layout = Arc::new(SetLayout::new_deferred(logical_device.clone()));
        let buffer_manager = buffer_manager.clone();
        Manager {
            buffer_manager,
            gbuff_set_layout,
            buffer_only_set_layout,
            deferred_set_layout,
            pool,
        }
    }

    pub fn create_gbuff_set(
        &mut self,
        uniform: Arc<Mutex<DynamicBuffer>>,
        textures: Vec<Arc<RwLock<Texture>>>,
    ) -> Set {
        Set::new_gbuff(
            self.pool.clone(),
            self.gbuff_set_layout.clone(),
            uniform,
            &self.buffer_manager,
            textures,
        )
    }

    pub fn create_buffer_only_set(&mut self, uniform: Arc<RwLock<DynamicBuffer>>) -> Set {
        Set::new_buffer_only(
            self.pool.clone(),
            self.buffer_only_set_layout.clone(),
            uniform,
            &self.buffer_manager,
        )
    }

    pub fn create_deferred_set(
        &mut self,
        uniform: Arc<RwLock<DynamicBuffer>>,
        textures: Vec<Arc<RwLock<Texture>>>,
    ) -> Set {
        Set::new_deferred(
            self.pool.clone(),
            self.deferred_set_layout.clone(),
            uniform,
            &self.buffer_manager,
            textures,
        )
    }
}
