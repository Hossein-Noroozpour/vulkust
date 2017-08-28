use std::cell::DebugCell;
use std::collections::BTreeMap;
use std::default::Default;
use std::ffi::CString;
use std::ptr::{null, null_mut};
use std::sync::{Arc, Weak};
use super::super::core::application::ApplicationTrait as CoreAppTrait;
use super::super::core::asset::manager::Manager as AssetManager;
use super::super::render::shader::{
    Id as ShaderId, Shader, BindingStage, ResourceType,
    shader_id_to_vertex_attributes,
    shader_id_resources,
};
use super::super::render::vertex::Attribute as VertexAttribute;
use super::super::system::vulkan as vk;
use super::super::util::Cacher;
use super::descriptor::{Pool as DescriptorPool, Set as DescriptorSet};
use super::device::logical::Logical as LogicalDevice;
use super::engine::Engine;
use super::render_pass::RenderPass;

pub struct Layout {
    pub logical_device: Arc<LogicalDevice>,
    pub descriptor_set_layout: vk::VkDescriptorSetLayout,
    pub vk_data: vk::VkPipelineLayout,
}
impl Layout {
    pub fn new(logical_device: Arc<LogicalDevice>, sid: ShaderId) -> Self {
        let mut descriptor_set_layout = null_mut();
        let mut vk_data = 0 as vk::VkPipelineLayout;
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
        vulkan_check!(vk::vkCreateDescriptorSetLayout(
            logical_device.vk_data,
            &descriptor_layout,
            null(),
            &mut descriptor_set_layout,
        ));
        let mut pipeline_layout_create_info = vk::VkPipelineLayoutCreateInfo::default();
        pipeline_layout_create_info.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO;
        pipeline_layout_create_info.setLayoutCount = 1;
        pipeline_layout_create_info.pSetLayouts = &descriptor_set_layout;
        vulkan_check!(vk::vkCreatePipelineLayout(
            logical_device.vk_data,
            &pipeline_layout_create_info,
            null(),
            &mut vk_data,
        ));
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
            vk::vkDestroyPipelineLayout(self.logical_device.vk_data, self.vk_data, null());
            vk::vkDestroyDescriptorSetLayout(
                self.logical_device.vk_data,
                self.descriptor_set_layout,
                null(),
            );
        }
    }
}

pub struct Cache {
    pub logical_device: Arc<LogicalDevice>,
    pub vk_data: vk::VkPipelineCache,
}

impl Cache {
    pub fn new(logical_device: Arc<LogicalDevice>) -> Self {
        let mut vk_data = 0 as vk::VkPipelineCache;
        let mut pipeline_cache_create_info = vk::VkPipelineCacheCreateInfo::default();
        pipeline_cache_create_info.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO;
        vulkan_check!(vk::vkCreatePipelineCache(
            logical_device.vk_data,
            &pipeline_cache_create_info,
            null(),
            &mut vk_data,
        ));
        Cache {
            logical_device: logical_device,
            vk_data: vk_data,
        }
    }
}

impl Drop for Cache {
    fn drop(&mut self) {
        unsafe {
            vk::vkDestroyPipelineCache(self.logical_device.vk_data, self.vk_data, null());
        }
    }
}

pub struct Pipeline {
    pub cache: Arc<Cache>,
    pub descriptor_pool: Arc<DescriptorPool>,
    pub descriptor_set: DescriptorSet,
    pub layout: Layout,
    pub shader: Arc<Shader>,
    pub render_pass: Arc<RenderPass>,
    pub vk_data: vk::VkPipeline,
}

impl Pipeline {
    fn new(
        manager: &Manager, sid: ShaderId
    ) -> Self
    {
        let device = manager.cache.logical_device.clone();
        let shader = manager.asset_manager.borrow_mut().get_shader(sid, device);
        let vertex_attrs = shader_id_to_vertex_attributes(sid);
        let layout = Layout::new(device, sid);
        let render_pass = manager.render_pass.clone();
        let pipeline_cache = manager.cache.clone();
        let mut input_assembly_state = vk::VkPipelineInputAssemblyStateCreateInfo::default();
        input_assembly_state.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO;
        input_assembly_state.topology =
            vk::VkPrimitiveTopology::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST;
        let mut rasterization_state = vk::VkPipelineRasterizationStateCreateInfo::default();
        rasterization_state.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO;
        rasterization_state.polygonMode = vk::VkPolygonMode::VK_POLYGON_MODE_FILL;
        rasterization_state.cullMode = vk::VkCullModeFlagBits::VK_CULL_MODE_FRONT_BIT as u32;
        rasterization_state.frontFace = vk::VkFrontFace::VK_FRONT_FACE_COUNTER_CLOCKWISE;
        rasterization_state.lineWidth = 1.0f32;
        let mut blend_attachment_state =
            vec![vk::VkPipelineColorBlendAttachmentState::default(); 1];
        blend_attachment_state[0].colorWriteMask = 0xF;
        let mut color_blend_state = vk::VkPipelineColorBlendStateCreateInfo::default();
        color_blend_state.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO;
        color_blend_state.attachmentCount = 1;
        color_blend_state.pAttachments = blend_attachment_state.as_ptr();
        let mut viewport_state = vk::VkPipelineViewportStateCreateInfo::default();
        viewport_state.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO;
        viewport_state.viewportCount = 1;
        viewport_state.scissorCount = 1;
        let dynamic_state_enables = [
            vk::VkDynamicState::VK_DYNAMIC_STATE_VIEWPORT,
            vk::VkDynamicState::VK_DYNAMIC_STATE_SCISSOR,
        ];
        let mut dynamic_state = vk::VkPipelineDynamicStateCreateInfo::default();
        dynamic_state.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO;
        dynamic_state.pDynamicStates = dynamic_state_enables.as_ptr();
        dynamic_state.dynamicStateCount = dynamic_state_enables.len() as u32;
        let mut depth_stencil_state = vk::VkPipelineDepthStencilStateCreateInfo::default();
        depth_stencil_state.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO;
        depth_stencil_state.depthTestEnable = 1;
        depth_stencil_state.depthWriteEnable = 1;
        depth_stencil_state.depthCompareOp = vk::VkCompareOp::VK_COMPARE_OP_LESS_OR_EQUAL;
        depth_stencil_state.depthBoundsTestEnable = 0;
        depth_stencil_state.back.failOp = vk::VkStencilOp::VK_STENCIL_OP_KEEP;
        depth_stencil_state.back.passOp = vk::VkStencilOp::VK_STENCIL_OP_KEEP;
        depth_stencil_state.back.compareOp = vk::VkCompareOp::VK_COMPARE_OP_ALWAYS;
        depth_stencil_state.stencilTestEnable = 0;
        depth_stencil_state.front = depth_stencil_state.back;
        let mut multisample_state = vk::VkPipelineMultisampleStateCreateInfo::default();
        multisample_state.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO;
        multisample_state.rasterizationSamples = vk::VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT;
        let mut vertex_input_binding = vk::VkVertexInputBindingDescription::default();
        vertex_input_binding.stride = 0;
        vertex_input_binding.inputRate = vk::VkVertexInputRate::VK_VERTEX_INPUT_RATE_VERTEX;
        let mut vertex_attributes =
            vec![vk::VkVertexInputAttributeDescription::default(); vertex_attrs.len()];
        for i in 0..vertex_attrs.len() {
            vertex_attributes[i].location = i as u32;
            vertex_attributes[i].offset = vertex_input_binding.stride;
            vertex_attributes[i].format = match vertex_attrs[i].clone() {
                VertexAttribute::Vec3F32 => {
                    vertex_input_binding.stride += 12;
                    vk::VkFormat::VK_FORMAT_R32G32B32_SFLOAT
                }
                VertexAttribute::Vec2F32 => {
                    vertex_input_binding.stride += 8;
                    vk::VkFormat::VK_FORMAT_R32G32_SFLOAT
                }
            }
        }
        let mut vertex_input_state = vk::VkPipelineVertexInputStateCreateInfo::default();
        vertex_input_state.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO;
        vertex_input_state.vertexBindingDescriptionCount = 1;
        vertex_input_state.pVertexBindingDescriptions = &vertex_input_binding;
        vertex_input_state.vertexAttributeDescriptionCount = vertex_attributes.len() as u32;
        vertex_input_state.pVertexAttributeDescriptions = vertex_attributes.as_ptr();
        let stage_name = CString::new("main").unwrap();
        let shader = material.get_shader();
        let stages_count = shader.get_stages_count();
        let mut shader_stages = vec![vk::VkPipelineShaderStageCreateInfo::default(); stages_count];
        for i in 0..stages_count {
            shader_stages[i].sType =
                vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO;
            shader_stages[i].stage = match i {
                0 => vk::VkShaderStageFlagBits::VK_SHADER_STAGE_VERTEX_BIT,
                1 => vk::VkShaderStageFlagBits::VK_SHADER_STAGE_FRAGMENT_BIT,
                n @ _ => {
                    logf!("Stage {} is not implemented yet!", n);
                }
            };
            shader_stages[i].module = shader.get_stage(i).module;
            shader_stages[i].pName = stage_name.as_ptr();
        }
        let mut pipeline_create_info = vk::VkGraphicsPipelineCreateInfo::default();
        pipeline_create_info.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO;
        pipeline_create_info.layout = layout.vk_data;
        pipeline_create_info.renderPass = render_pass.vk_data;
        pipeline_create_info.stageCount = shader_stages.len() as u32;
        pipeline_create_info.pStages = shader_stages.as_ptr();
        pipeline_create_info.pVertexInputState = &vertex_input_state;
        pipeline_create_info.pInputAssemblyState = &input_assembly_state;
        pipeline_create_info.pRasterizationState = &rasterization_state;
        pipeline_create_info.pColorBlendState = &color_blend_state;
        pipeline_create_info.pMultisampleState = &multisample_state;
        pipeline_create_info.pViewportState = &viewport_state;
        pipeline_create_info.pDepthStencilState = &depth_stencil_state;
        pipeline_create_info.renderPass = render_pass.vk_data;
        pipeline_create_info.pDynamicState = &dynamic_state;
        let mut vk_data = 0 as vk::VkPipeline;
        vulkan_check!(vk::vkCreateGraphicsPipelines(
            pipeline_cache.logical_device.vk_data,
            pipeline_cache.vk_data,
            1,
            &pipeline_create_info,
            null(),
            &mut vk_data,
        ));
        Pipeline {
            layout: layout.clone(),
            render_pass: render_pass.clone(),
            cache: pipeline_cache.clone(),
            vk_data: vk_data,
        }
    }
}

impl Drop for Pipeline {
    fn drop(&mut self) {
        unsafe {
            vk::vkDestroyPipeline(self.layout.logical_device.vk_data, self.vk_data, null());
        }
    }
}

pub struct Manager {
    pub cache: Arc<Cache>,
    pub cached: Cacher<ShaderId, Pipeline>,
    pub descriptor_pool: Arc<DescriptorPool>,
    pub render_pass: Arc<RenderPass>,
    pub asset_manager: Arc<DebugCell<AssetManager>>,
}

impl Manager {
    pub fn new<CoreApp>(
        engine: &Engine<CoreApp>
    ) -> Self 
    where
        CoreApp: CoreAppTrait {
        let device = engine.logical_device.as_ref().unwrap().clone();
        Manager {
            render_pass: engine.render_pass.as_ref().unwrap().clone(),
            cache: Arc::new(Cache::new(device.clone())),
            descriptor_pool: Arc::new(DescriptorPool::new(device)),
            cached: Cacher::new(),
            asset_manager: engine.os_app.asset_manager.clone(),
        }
    }

    pub fn get(&mut self, id: ShaderId) -> Arc<Pipeline> {
        self.cached.get(id, &|| { self.create_pipeline(id) } )
    }

    fn create_pipeline(&self, id: ShaderId) -> Pipeline {
        Pipeline::new(self, id)
    }
}
