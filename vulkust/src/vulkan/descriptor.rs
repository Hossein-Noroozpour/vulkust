use super::super::core::allocate::Object as AlcObject;
use super::super::core::types::Id;
use super::super::render::config::Configurations;
#[cfg(debug_mode)]
use super::super::render::config::MAX_DIRECTIONAL_CASCADES_COUNT;
use super::super::render::texture::Texture;
use super::buffer::Dynamic as DynamicBuffer;
use super::device::Logical as LogicalDevice;
use super::vulkan as vk;
use std::collections::BTreeMap;
use std::ptr::null;
use std::sync::{Arc, RwLock, Weak};

const SSAO_TEX_COUNT: usize = 3;
const DEFERRED_TEX_COUNT: usize = 6;
const GBUFF_TEX_COUNT: usize = 7;

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Pool {
    pub logical_device: Arc<LogicalDevice>,
    pub vk_data: vk::VkDescriptorPool,
}

impl Pool {
    pub fn new(logical_device: Arc<LogicalDevice>, conf: &Configurations) -> Self {
        let buffers_count =
            conf.get_max_meshes_count() + conf.get_max_models_count() + conf.get_max_scenes_count();
        let mut type_counts = [vk::VkDescriptorPoolSize::default(); 2];
        type_counts[0].type_ = vk::VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC;
        type_counts[0].descriptorCount = buffers_count as u32;
        type_counts[1].type_ = vk::VkDescriptorType::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER;
        type_counts[1].descriptorCount = conf.get_max_textures_count() as u32;
        let mut descriptor_pool_info = vk::VkDescriptorPoolCreateInfo::default();
        descriptor_pool_info.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO;
        descriptor_pool_info.poolSizeCount = type_counts.len() as u32;
        descriptor_pool_info.pPoolSizes = type_counts.as_ptr();
        descriptor_pool_info.maxSets = buffers_count as u32 + 1; // todo I must find the exact number after everything got stable
        let mut vk_data = 0 as vk::VkDescriptorPool;
        vulkan_check!(vk::vkCreateDescriptorPool(
            logical_device.get_data(),
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
            vk::vkDestroyDescriptorPool(self.logical_device.get_data(), self.vk_data, null());
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
        let layout_bindings = Self::create_binding_info(&[1; GBUFF_TEX_COUNT]);
        return Self::new_with_bindings_info(logical_device, &layout_bindings);
    }

    pub fn new_buffer_only(logical_device: Arc<LogicalDevice>) -> Self {
        let layout_bindings = Self::create_binding_info(&[]);
        return Self::new_with_bindings_info(logical_device, &layout_bindings);
    }

    pub fn new_deferred(logical_device: Arc<LogicalDevice>) -> Self {
        let layout_bindings = Self::create_binding_info(&[1; DEFERRED_TEX_COUNT]);
        return Self::new_with_bindings_info(logical_device, &layout_bindings);
    }

    pub fn new_ssao(logical_device: Arc<LogicalDevice>) -> Self {
        let layout_bindings = Self::create_binding_info(&[1; SSAO_TEX_COUNT]);
        return Self::new_with_bindings_info(logical_device, &layout_bindings);
    }

    pub fn new_shadow_accumulator_directional(
        logical_device: Arc<LogicalDevice>,
        conf: &Configurations,
    ) -> Self {
        let layout_bindings =
            Self::create_binding_info(&[1, 1, conf.get_cascaded_shadows_count() as u32]);
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
            logical_device.get_data(),
            &descriptor_layout,
            null(),
            &mut vk_data,
        ));
        SetLayout {
            logical_device,
            vk_data,
        }
    }

    fn create_binding_info(images: &[u32]) -> Vec<vk::VkDescriptorSetLayoutBinding> {
        let images_count = images.len();
        let mut layout_bindings =
            vec![vk::VkDescriptorSetLayoutBinding::default(); 1 + images_count];
        layout_bindings[0].binding = 0;
        layout_bindings[0].descriptorType =
            vk::VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC;
        layout_bindings[0].descriptorCount = 1;
        layout_bindings[0].stageFlags = vk::VkShaderStageFlagBits::VK_SHADER_STAGE_VERTEX_BIT
            as u32
            | vk::VkShaderStageFlagBits::VK_SHADER_STAGE_FRAGMENT_BIT as u32;
        let mut binding_index = 0;
        for image_count in images {
            binding_index += 1;
            layout_bindings[binding_index].binding = binding_index as u32;
            layout_bindings[binding_index].descriptorCount = *image_count;
            layout_bindings[binding_index].descriptorType =
                vk::VkDescriptorType::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER;
            layout_bindings[binding_index].stageFlags =
                vk::VkShaderStageFlagBits::VK_SHADER_STAGE_FRAGMENT_BIT as u32;
        }
        return layout_bindings;
    }
}

impl Drop for SetLayout {
    fn drop(&mut self) {
        unsafe {
            vk::vkDestroyDescriptorSetLayout(self.logical_device.get_data(), self.vk_data, null());
        }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct Set {
    pub pool: Arc<Pool>,
    pub layout: Arc<SetLayout>,
    pub texturess: Vec<Vec<Arc<RwLock<Texture>>>>,
    pub vk_data: vk::VkDescriptorSet,
}

impl Set {
    pub fn new_buffer_only(
        pool: Arc<Pool>,
        layout: Arc<SetLayout>,
        uniform: &DynamicBuffer,
    ) -> Self {
        Self::new(pool, layout, uniform, Vec::new())
    }

    pub fn new_gbuff(
        pool: Arc<Pool>,
        layout: Arc<SetLayout>,
        uniform: &DynamicBuffer,
        textures: Vec<Arc<RwLock<Texture>>>,
    ) -> Self {
        let mut texturess = Vec::new();
        for t in textures {
            texturess.push(vec![t]);
        }
        Self::new(pool, layout, uniform, texturess)
    }

    pub fn new_deferred(
        pool: Arc<Pool>,
        layout: Arc<SetLayout>,
        uniform: &DynamicBuffer,
        textures: Vec<Arc<RwLock<Texture>>>,
    ) -> Self {
        #[cfg(debug_mode)]
        {
            if textures.len() != DEFERRED_TEX_COUNT {
                vxlogf!(
                    "For deferred descriptor you need {} textures.",
                    DEFERRED_TEX_COUNT
                );
            }
        }
        let mut texturess = Vec::new();
        for t in textures {
            texturess.push(vec![t]);
        }
        Self::new(pool, layout, uniform, texturess)
    }

    pub fn new_ssao(
        pool: Arc<Pool>,
        layout: Arc<SetLayout>,
        uniform: &DynamicBuffer,
        textures: Vec<Arc<RwLock<Texture>>>,
    ) -> Self {
        #[cfg(debug_mode)]
        {
            if textures.len() != SSAO_TEX_COUNT {
                vxlogf!("For SSAO descriptor you need {} textures.", SSAO_TEX_COUNT);
            }
        }
        let mut texturess = Vec::new();
        for t in textures {
            texturess.push(vec![t]);
        }
        Self::new(pool, layout, uniform, texturess)
    }

    pub fn new_shadow_accumulator_directional(
        pool: Arc<Pool>,
        layout: Arc<SetLayout>,
        uniform: &DynamicBuffer,
        texturess: Vec<Vec<Arc<RwLock<Texture>>>>,
    ) -> Self {
        #[cfg(debug_mode)]
        {
            if texturess.len() != 3 {
                vxlogf!("For shadow accumulator directional descriptor you need 3 textures.");
            }
            if texturess[0].len() != 1
                || texturess[1].len() != 1
                || texturess[2].len() < 1
                || texturess[2].len() > MAX_DIRECTIONAL_CASCADES_COUNT as usize
            {
                vxlogf!("Wrong number of textures for shadow accumulator directional descriptor.");
            }
        }
        Self::new(pool, layout, uniform, texturess)
    }

    fn create_buffer_info(uniform: &DynamicBuffer) -> vk::VkDescriptorBufferInfo {
        let buffer = vxresult!(uniform.get_buffer(0).read());
        let mut buff_info = vk::VkDescriptorBufferInfo::default();
        buff_info.buffer = buffer.get_data();
        buff_info.range = buffer.get_allocated_memory().get_size() as vk::VkDeviceSize;
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
            pool.logical_device.get_data(),
            &alloc_info,
            &mut vk_data,
        ));
        return vk_data;
    }

    fn new(
        pool: Arc<Pool>,
        layout: Arc<SetLayout>,
        uniform: &DynamicBuffer,
        texturess: Vec<Vec<Arc<RwLock<Texture>>>>,
    ) -> Self {
        let vk_data = Self::allocate_set(&pool, &layout);
        let buff_info = Self::create_buffer_info(uniform);
        let mut infos = vec![vk::VkWriteDescriptorSet::default(); 1 + texturess.len()];
        infos[0].sType = vk::VkStructureType::VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
        infos[0].dstSet = vk_data;
        infos[0].descriptorCount = 1;
        infos[0].descriptorType = vk::VkDescriptorType::VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC;
        infos[0].pBufferInfo = &buff_info;
        infos[0].dstBinding = 0;
        let mut img_infoss = Vec::new();
        for textures in &texturess {
            let mut img_infos = Vec::new();
            for texture in textures {
                let texture = vxresult!(texture.read());
                let mut img_info = vk::VkDescriptorImageInfo::default();
                img_info.imageLayout = vk::VkImageLayout::VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL;
                img_info.imageView = texture.get_image_view().get_data();
                img_info.sampler = texture.get_sampler().get_data();
                img_infos.push(img_info);
            }
            img_infoss.push(img_infos);
        }
        let mut last_info_i = 1;
        let mut last_img_info_i = 0;
        for _ in &texturess {
            infos[last_info_i].sType = vk::VkStructureType::VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
            infos[last_info_i].dstSet = vk_data;
            infos[last_info_i].descriptorCount = img_infoss[last_img_info_i].len() as u32;
            infos[last_info_i].descriptorType =
                vk::VkDescriptorType::VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER;
            infos[last_info_i].pImageInfo = img_infoss[last_img_info_i].as_ptr();
            infos[last_info_i].dstBinding = last_info_i as u32;
            last_info_i += 1;
            last_img_info_i += 1;
        }
        unsafe {
            vk::vkUpdateDescriptorSets(
                pool.logical_device.get_data(),
                infos.len() as u32,
                infos.as_ptr(),
                0,
                null(),
            );
        }
        Set {
            pool,
            layout,
            texturess,
            vk_data,
        }
    }
}

impl Drop for Set {
    fn drop(&mut self) {}
}

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct Manager {
    buffer_only_set_layout: Arc<SetLayout>,
    gbuff_set_layout: Arc<SetLayout>,
    deferred_set_layout: Arc<SetLayout>,
    ssao_set_layout: Arc<SetLayout>,
    shadow_accumulator_directional_set_layout: Arc<SetLayout>,
    buffer_only_sets: BTreeMap<usize, Weak<Set>>,
    gbuff_sets: BTreeMap<([Id; GBUFF_TEX_COUNT], usize), Weak<Set>>,
    deferred_set: Option<Arc<Set>>,
    ssao_set: Option<Arc<Set>>,
    shadow_accumulator_directional_set: Option<Arc<Set>>,
    pool: Arc<Pool>,
}

// todo it can in future cache the sets based on their buffer id and size and texture ids and samplers

impl Manager {
    pub(crate) fn new(logical_device: &Arc<LogicalDevice>, conf: &Configurations) -> Self {
        let pool = Arc::new(Pool::new(logical_device.clone(), conf));
        let gbuff_set_layout = Arc::new(SetLayout::new_gbuff(logical_device.clone()));
        let buffer_only_set_layout = Arc::new(SetLayout::new_buffer_only(logical_device.clone()));
        let deferred_set_layout = Arc::new(SetLayout::new_deferred(logical_device.clone()));
        let ssao_set_layout = Arc::new(SetLayout::new_ssao(logical_device.clone()));
        let shadow_accumulator_directional_set_layout = Arc::new(
            SetLayout::new_shadow_accumulator_directional(logical_device.clone(), conf),
        );
        Manager {
            gbuff_set_layout,
            buffer_only_set_layout,
            deferred_set_layout,
            ssao_set_layout,
            shadow_accumulator_directional_set_layout,
            buffer_only_sets: BTreeMap::new(),
            gbuff_sets: BTreeMap::new(),
            deferred_set: None,
            ssao_set: None,
            shadow_accumulator_directional_set: None,
            pool,
        }
    }

    pub(crate) fn create_gbuff_set(
        &mut self,
        uniform: &DynamicBuffer,
        textures: Vec<Arc<RwLock<Texture>>>,
    ) -> Arc<Set> {
        #[cfg(debug_mode)]
        {
            if textures.len() != GBUFF_TEX_COUNT {
                vxlogf!(
                    "For gbuffer filler descriptor you need {} textures.",
                    GBUFF_TEX_COUNT
                );
            }
        }
        let mut id = ([0 as Id; GBUFF_TEX_COUNT], 0usize);
        for i in 0..GBUFF_TEX_COUNT {
            id.0[i] = vxresult!(textures[i].read()).get_id();
        }
        id.1 = vxresult!(uniform.get_buffer(0).read())
            .get_allocated_memory()
            .get_size() as usize;
        if let Some(s) = self.gbuff_sets.get(&id) {
            if let Some(s) = s.upgrade() {
                return s;
            }
        }
        let s = Arc::new(Set::new_gbuff(
            self.pool.clone(),
            self.gbuff_set_layout.clone(),
            uniform,
            textures,
        ));
        self.gbuff_sets.insert(id, Arc::downgrade(&s));
        return s;
    }

    pub(crate) fn create_buffer_only_set(&mut self, uniform: &DynamicBuffer) -> Arc<Set> {
        let id = vxresult!(uniform.get_buffer(0).read())
            .get_allocated_memory()
            .get_size() as usize;
        if let Some(s) = self.buffer_only_sets.get(&id) {
            if let Some(s) = s.upgrade() {
                return s;
            }
        }
        let s = Arc::new(Set::new_buffer_only(
            self.pool.clone(),
            self.buffer_only_set_layout.clone(),
            uniform,
        ));
        self.buffer_only_sets.insert(id, Arc::downgrade(&s));
        return s;
    }

    pub(crate) fn create_deferred_set(
        &mut self,
        uniform: &DynamicBuffer,
        textures: Vec<Arc<RwLock<Texture>>>,
    ) -> Arc<Set> {
        if let Some(s) = &self.deferred_set {
            return s.clone();
        }
        let s = Arc::new(Set::new_deferred(
            self.pool.clone(),
            self.deferred_set_layout.clone(),
            uniform,
            textures,
        ));
        self.deferred_set = Some(s.clone());
        return s;
    }

    pub(crate) fn create_ssao_set(
        &mut self,
        uniform: &DynamicBuffer,
        textures: Vec<Arc<RwLock<Texture>>>,
    ) -> Arc<Set> {
        if let Some(s) = &self.ssao_set {
            return s.clone();
        }
        let s = Arc::new(Set::new_ssao(
            self.pool.clone(),
            self.ssao_set_layout.clone(),
            uniform,
            textures,
        ));
        self.ssao_set = Some(s.clone());
        return s;
    }

    pub(crate) fn create_shadow_accumulator_directional_set(
        &mut self,
        uniform: &DynamicBuffer,
        texturess: Vec<Vec<Arc<RwLock<Texture>>>>,
    ) -> Arc<Set> {
        if let Some(s) = &self.shadow_accumulator_directional_set {
            return s.clone();
        }
        let s = Arc::new(Set::new_shadow_accumulator_directional(
            self.pool.clone(),
            self.shadow_accumulator_directional_set_layout.clone(),
            uniform,
            texturess,
        ));
        self.shadow_accumulator_directional_set = Some(s.clone());
        return s;
    }

    pub(super) fn get_buffer_only_set_layout(&self) -> &Arc<SetLayout> {
        return &self.buffer_only_set_layout;
    }

    pub(super) fn get_gbuff_set_layout(&self) -> &Arc<SetLayout> {
        return &self.gbuff_set_layout;
    }

    pub(super) fn get_deferred_set_layout(&self) -> &Arc<SetLayout> {
        return &self.deferred_set_layout;
    }

    pub(super) fn get_ssao_set_layout(&self) -> &Arc<SetLayout> {
        return &self.ssao_set_layout;
    }

    pub(super) fn get_shadow_accumulator_directional_set_layout(&self) -> &Arc<SetLayout> {
        return &self.shadow_accumulator_directional_set_layout;
    }

    pub(super) fn get_pool(&self) -> &Arc<Pool> {
        return &self.pool;
    }
}
