use super::descriptor::{Manager as DescriptorManager, SetLayout as DescriptorSetLayout};
use super::device::logical::Logical as LogicalDevice;
use super::render_pass::RenderPass;
use super::shader::Module;
use super::vulkan as vk;
use std::ffi::CString;
use std::ptr::null;
use std::sync::{Arc, RwLock};

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Layout {
    pub descriptor_set_layout: Arc<DescriptorSetLayout>,
    pub vk_data: vk::VkPipelineLayout,
}

impl Layout {
    pub fn new(
        logical_device: Arc<LogicalDevice>,
        descriptor_manager: &Arc<RwLock<DescriptorManager>>,
    ) -> Self {
        let mut vk_data = 0 as vk::VkPipelineLayout;
        let descriptor_set_layout = vxresult!(descriptor_manager.read()).main_set_layout.clone();
        let mut pipeline_layout_create_info = vk::VkPipelineLayoutCreateInfo::default();
        pipeline_layout_create_info.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO;
        pipeline_layout_create_info.setLayoutCount = 1;
        pipeline_layout_create_info.pSetLayouts = &descriptor_set_layout.vk_data;
        vulkan_check!(vk::vkCreatePipelineLayout(
            logical_device.vk_data,
            &pipeline_layout_create_info,
            null(),
            &mut vk_data,
        ));
        Layout {
            descriptor_set_layout,
            vk_data,
        }
    }
}

impl Drop for Layout {
    fn drop(&mut self) {
        unsafe {
            vk::vkDestroyPipelineLayout(
                self.descriptor_set_layout.logical_device.vk_data,
                self.vk_data,
                null(),
            );
        }
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Cache {
    pub logical_device: Arc<LogicalDevice>,
    pub vk_data: vk::VkPipelineCache,
}

impl Cache {
    pub fn new(logical_device: &Arc<LogicalDevice>) -> Self {
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
            logical_device: logical_device.clone(),
            vk_data,
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

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Pipeline {
    pub cache: Arc<Cache>,
    pub layout: Layout,
    pub shader: Vec<Module>,
    pub render_pass: Arc<RenderPass>,
    pub vk_data: vk::VkPipeline,
}

impl Pipeline {
    fn new(
        descriptor_manager: &Arc<RwLock<DescriptorManager>>,
        render_pass: Arc<RenderPass>,
        cache: Arc<Cache>,
    ) -> Self {
        let device = vxresult!(descriptor_manager.read())
            .pool
            .logical_device
            .clone();
        let vert_bytes = include_bytes!(concat!(env!("OUT_DIR"), "/vulkan/shaders/main.vert.spv"));
        let frag_bytes = include_bytes!(concat!(env!("OUT_DIR"), "/vulkan/shaders/main.frag.spv"));
        let vertex_shader = Module::new(vert_bytes, device.clone());
        let fragment_shader = Module::new(frag_bytes, device.clone());
        let shader = vec![vertex_shader, fragment_shader];
        let layout = Layout::new(device.clone(), descriptor_manager);
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
        rasterization_state.frontFace = vk::VkFrontFace::VK_FRONT_FACE_CLOCKWISE;
        rasterization_state.lineWidth = 1.0f32;
        let mut blend_attachment_state =
            vec![vk::VkPipelineColorBlendAttachmentState::default(); 1];
        blend_attachment_state[0].colorWriteMask = 0xF;
        let mut color_blend_state = vk::VkPipelineColorBlendStateCreateInfo::default();
        color_blend_state.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO;
        color_blend_state.attachmentCount = blend_attachment_state.len() as u32;
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
        vertex_input_binding.stride = 3 * 4 + 3 * 4 + 3 * 4 + 2 * 4; // temporary
        vertex_input_binding.inputRate = vk::VkVertexInputRate::VK_VERTEX_INPUT_RATE_VERTEX;
        let mut vertex_attributes = vec![vk::VkVertexInputAttributeDescription::default(); 4];
        vertex_attributes[0].format = vk::VkFormat::VK_FORMAT_R32G32B32_SFLOAT;
        vertex_attributes[1].location = 1;
        vertex_attributes[1].offset = 3 * 4;
        vertex_attributes[1].format = vk::VkFormat::VK_FORMAT_R32G32B32_SFLOAT;
        vertex_attributes[2].location = 2;
        vertex_attributes[2].offset = 3 * 4 + 3 * 4;
        vertex_attributes[2].format = vk::VkFormat::VK_FORMAT_R32G32B32_SFLOAT;
        vertex_attributes[3].location = 3;
        vertex_attributes[3].offset = 3 * 4 + 3 * 4 + 3 * 4;
        vertex_attributes[3].format = vk::VkFormat::VK_FORMAT_R32G32_SFLOAT;
        let mut vertex_input_state = vk::VkPipelineVertexInputStateCreateInfo::default();
        vertex_input_state.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO;
        vertex_input_state.vertexBindingDescriptionCount = 1;
        vertex_input_state.pVertexBindingDescriptions = &vertex_input_binding;
        vertex_input_state.vertexAttributeDescriptionCount = vertex_attributes.len() as u32;
        vertex_input_state.pVertexAttributeDescriptions = vertex_attributes.as_ptr();
        let stage_name = CString::new("main").unwrap();
        let stages_count = shader.len();
        let mut shader_stages = vec![vk::VkPipelineShaderStageCreateInfo::default(); stages_count];
        for i in 0..stages_count {
            shader_stages[i].sType =
                vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO;
            shader_stages[i].stage = match i {
                0 => vk::VkShaderStageFlagBits::VK_SHADER_STAGE_VERTEX_BIT,
                1 => vk::VkShaderStageFlagBits::VK_SHADER_STAGE_FRAGMENT_BIT,
                n @ _ => {
                    vxlogf!("Stage {} is not implemented yet!", n);
                }
            };
            shader_stages[i].module = shader[i].vk_data;
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
            device.vk_data,
            cache.vk_data,
            1,
            &pipeline_create_info,
            null(),
            &mut vk_data,
        ));
        Pipeline {
            cache,
            layout,
            shader,
            render_pass,
            vk_data,
        }
    }
}

impl Drop for Pipeline {
    fn drop(&mut self) {
        unsafe {
            vk::vkDestroyPipeline(self.cache.logical_device.vk_data, self.vk_data, null());
        }
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Manager {
    pub cache: Arc<Cache>,
    pub main_pipeline: Arc<Pipeline>,
    pub descriptor_manager: Arc<RwLock<DescriptorManager>>,
    pub render_pass: Arc<RenderPass>,
}

impl Manager {
    pub fn new(
        logical_device: &Arc<LogicalDevice>,
        descriptor_manager: &Arc<RwLock<DescriptorManager>>,
        render_pass: &Arc<RenderPass>,
    ) -> Self {
        let cache = Arc::new(Cache::new(logical_device));
        let descriptor_manager = descriptor_manager.clone();
        let main_pipeline = Arc::new(Pipeline::new(
            &descriptor_manager,
            render_pass.clone(),
            cache.clone(),
        ));
        let render_pass = render_pass.clone();
        Manager {
            render_pass,
            cache,
            descriptor_manager,
            main_pipeline,
        }
    }
}
