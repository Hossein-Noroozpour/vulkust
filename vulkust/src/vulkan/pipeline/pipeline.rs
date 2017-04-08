use std::sync::Arc;
use std::ffi::CString;
use std::ptr::null;
use std::fs::File;
use std::io::Read;
use std::mem::transmute;
use super::super::super::system::vulkan as vk;
use super::super::device::logical::Logical as LogicalDevice;
use super::super::render_pass::RenderPass;
use super::layout::Layout;
use super::cache::Cache;

pub struct Pipeline {
    pub layout: Arc<Layout>,
    pub render_pass: Arc<RenderPass>,
    pub cache: Arc<Cache>,
    pub vk_data: vk::VkPipeline,
}

impl Pipeline {
    fn new(layout: Arc<Layout>, render_pass: Arc<RenderPass>, pipeline_cache: Arc<Cache>) -> Self {
		let mut pipeline_create_info = vk::VkGraphicsPipelineCreateInfo::default();
		pipeline_create_info.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO;
		pipeline_create_info.layout = layout.vk_data;
		pipeline_create_info.renderPass = render_pass.vk_data;
		let mut input_assembly_state = vk::VkPipelineInputAssemblyStateCreateInfo::default();
		input_assembly_state.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO;
		input_assembly_state.topology =
            vk::VkPrimitiveTopology::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST;
		let mut rasterization_state = vk::VkPipelineRasterizationStateCreateInfo::default();
		rasterization_state.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO;
		rasterization_state.polygonMode = vk::VkPolygonMode::VK_POLYGON_MODE_FILL;
		rasterization_state.cullMode = vk::VkCullModeFlagBits::VK_CULL_MODE_NONE as u32;
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
            vk::VkDynamicState::VK_DYNAMIC_STATE_SCISSOR];
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
		vertex_input_binding.stride = 24;
		vertex_input_binding.inputRate = vk::VkVertexInputRate::VK_VERTEX_INPUT_RATE_VERTEX;
		let mut vertex_attribute = [vk::VkVertexInputAttributeDescription::default(); 2];
		vertex_attribute[0].format = vk::VkFormat::VK_FORMAT_R32G32B32_SFLOAT;
		vertex_attribute[1].location = 1;
		vertex_attribute[1].format = vk::VkFormat::VK_FORMAT_R32G32B32_SFLOAT;
		vertex_attribute[1].offset = 12;
		let mut vertex_input_state = vk::VkPipelineVertexInputStateCreateInfo::default();
		vertex_input_state.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO;
		vertex_input_state.vertexBindingDescriptionCount = 1;
		vertex_input_state.pVertexBindingDescriptions = &vertex_input_binding;
		vertex_input_state.vertexAttributeDescriptionCount = 2;
		vertex_input_state.pVertexAttributeDescriptions = vertex_attribute.as_ptr();
        let stage_name = CString::new("main").unwrap();
		let mut shader_stages = [vk::VkPipelineShaderStageCreateInfo::default(); 2];
		shader_stages[0].sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO;
		shader_stages[0].stage = vk::VkShaderStageFlagBits::VK_SHADER_STAGE_VERTEX_BIT;
		shader_stages[0].module = load_shader(
            "shaders/triangle.vert.spv", &pipeline_cache.logical_device);
		shader_stages[0].pName = stage_name.as_ptr();
		shader_stages[1].sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO;
		shader_stages[1].stage = vk::VkShaderStageFlagBits::VK_SHADER_STAGE_FRAGMENT_BIT;
		shader_stages[1].module = load_shader(
            "shaders/triangle.frag.spv", &pipeline_cache.logical_device);
		shader_stages[1].pName = stage_name.as_ptr();
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
            pipeline_cache.logical_device.vk_data, pipeline_cache.vk_data, 1,
            &pipeline_create_info, null(),&mut vk_data));
		unsafe {
            vk::vkDestroyShaderModule(
                pipeline_cache.logical_device.vk_data, shader_stages[0].module, null());
		    vk::vkDestroyShaderModule(
                pipeline_cache.logical_device.vk_data, shader_stages[1].module, null());
        }
        Pipeline {
            layout: layout,
            render_pass: render_pass,
            cache: pipeline_cache,
            vk_data: vk_data,
        }
    }
}

fn load_shader(s: &str, logical_device: &Arc<LogicalDevice>) -> vk::VkShaderModule {
    let mut shader_file = File::open(s).unwrap();
    let mut file_buffer = Vec::new();
    let shader_size = shader_file.read_to_end(&mut file_buffer).unwrap();
    let mut module_create_info = vk::VkShaderModuleCreateInfo::default();
    module_create_info.sType = vk::VkStructureType::VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO;
    module_create_info.codeSize = shader_size;
    module_create_info.pCode = unsafe {transmute(file_buffer.as_ptr())};
    let mut shader_module = 0 as vk::VkShaderModule;
    vulkan_check!(vk::vkCreateShaderModule(
        logical_device.vk_data, &module_create_info, null(), &mut shader_module));
    return shader_module;
}
