use super::super::render::pipeline::PipelineType;
use super::descriptor::{Manager as DescriptorManager, SetLayout as DescriptorSetLayout};
use super::device::logical::Logical as LogicalDevice;
use super::render_pass::RenderPass;
use super::shader::Module;
use super::vulkan as vk;
use std::collections::BTreeMap;
use std::ffi::CString;
use std::mem::transmute;
use std::ptr::null;
use std::sync::{Arc, RwLock, Weak};

macro_rules! include_shader {
    ($name:expr) => {
        include_bytes!(concat!(env!("OUT_DIR"), "/vulkan/shaders/", $name, ".spv"))
    };
}

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct Layout {
    pub descriptor_set_layouts: Vec<Arc<DescriptorSetLayout>>,
    pub vk_data: vk::VkPipelineLayout,
}

impl Layout {
    pub fn new_gbuff(descriptor_manager: &Arc<RwLock<DescriptorManager>>) -> Self {
        let descriptor_manager = vxresult!(descriptor_manager.read());
        let gbuff_descriptor_set_layout = descriptor_manager.gbuff_set_layout.clone();
        let buffer_only_descriptor_set_layout = descriptor_manager.buffer_only_set_layout.clone();
        let layout = [
            buffer_only_descriptor_set_layout.vk_data,
            buffer_only_descriptor_set_layout.vk_data,
            gbuff_descriptor_set_layout.vk_data,
        ];
        let descriptor_set_layouts = vec![
            gbuff_descriptor_set_layout,
            buffer_only_descriptor_set_layout,
        ];
        Self::new(&layout, descriptor_set_layouts)
    }

    pub fn new_shadow_mapper(descriptor_manager: &Arc<RwLock<DescriptorManager>>) -> Self {
        let descriptor_manager = vxresult!(descriptor_manager.read());
        let gbuff_descriptor_set_layout = descriptor_manager.gbuff_set_layout.clone();
        let buffer_only_descriptor_set_layout = descriptor_manager.buffer_only_set_layout.clone();
        let layout = [
            buffer_only_descriptor_set_layout.vk_data,
            gbuff_descriptor_set_layout.vk_data,
        ];
        let descriptor_set_layouts = vec![
            buffer_only_descriptor_set_layout,
            gbuff_descriptor_set_layout,
        ];
        Self::new(&layout, descriptor_set_layouts)
    }

    pub fn new_shadow_accumulator_directional(
        descriptor_manager: &Arc<RwLock<DescriptorManager>>,
    ) -> Self {
        let descriptor_manager = vxresult!(descriptor_manager.read());
        let shadow_accumulator_directional_descriptor_set_layout = descriptor_manager
            .shadow_accumulator_directional_set_layout
            .clone();
        let layout = [shadow_accumulator_directional_descriptor_set_layout.vk_data];
        let descriptor_set_layouts = vec![shadow_accumulator_directional_descriptor_set_layout];
        Self::new(&layout, descriptor_set_layouts)
    }

    pub fn new_resolver(descriptor_manager: &Arc<RwLock<DescriptorManager>>) -> Self {
        let descriptor_manager = vxresult!(descriptor_manager.read());
        let resolver_descriptor_set_layout = descriptor_manager.resolver_set_layout.clone();
        let layout = [resolver_descriptor_set_layout.vk_data];
        let descriptor_set_layouts = vec![resolver_descriptor_set_layout];
        Self::new(&layout, descriptor_set_layouts)
    }

    pub fn new_deferred(descriptor_manager: &Arc<RwLock<DescriptorManager>>) -> Self {
        let descriptor_manager = vxresult!(descriptor_manager.read());
        let deferred_descriptor_set_layout = descriptor_manager.deferred_set_layout.clone();
        let buffer_only_descriptor_set_layout = descriptor_manager.buffer_only_set_layout.clone();
        let layout = [
            buffer_only_descriptor_set_layout.vk_data,
            deferred_descriptor_set_layout.vk_data,
        ];
        let descriptor_set_layouts = vec![
            buffer_only_descriptor_set_layout,
            deferred_descriptor_set_layout,
        ];
        Self::new(&layout, descriptor_set_layouts)
    }

    fn new(
        layout: &[vk::VkDescriptorSetLayout],
        descriptor_set_layouts: Vec<Arc<DescriptorSetLayout>>,
    ) -> Self {
        let mut vk_data = 0 as vk::VkPipelineLayout;
        let vkdev = descriptor_set_layouts[0].logical_device.vk_data;
        let mut pipeline_layout_create_info = vk::VkPipelineLayoutCreateInfo::default();
        pipeline_layout_create_info.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO;
        pipeline_layout_create_info.setLayoutCount = layout.len() as u32;
        pipeline_layout_create_info.pSetLayouts = layout.as_ptr();
        vulkan_check!(vk::vkCreatePipelineLayout(
            vkdev,
            &pipeline_layout_create_info,
            null(),
            &mut vk_data,
        ));
        Layout {
            descriptor_set_layouts,
            vk_data,
        }
    }
}

impl Drop for Layout {
    fn drop(&mut self) {
        unsafe {
            vk::vkDestroyPipelineLayout(
                self.descriptor_set_layouts[0].logical_device.vk_data,
                self.vk_data,
                null(),
            );
        }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
struct Cache {
    logical_device: Arc<LogicalDevice>,
    vk_data: vk::VkPipelineCache,
}

impl Cache {
    fn new(logical_device: Arc<LogicalDevice>) -> Self {
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
            logical_device,
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

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Pipeline {
    cache: Arc<Cache>,
    layout: Layout,
    shaders: Vec<Module>,
    render_pass: Arc<RenderPass>,
    vk_data: vk::VkPipeline,
}

impl Pipeline {
    fn new(
        descriptor_manager: &Arc<RwLock<DescriptorManager>>,
        render_pass: Arc<RenderPass>,
        cache: Arc<Cache>,
        samples: vk::VkSampleCountFlagBits,
        pipeline_type: PipelineType,
    ) -> Self {
        let device = vxresult!(descriptor_manager.read())
            .pool
            .logical_device
            .clone();

        let vert_bytes: &'static [u8] = match pipeline_type {
            PipelineType::GBuffer => include_shader!("g-buffers-filler.vert"),
            PipelineType::Resolver => include_shader!("resolver.vert"),
            PipelineType::Deferred => include_shader!("deferred.vert"),
            PipelineType::ShadowMapper => include_shader!("shadow-mapper.vert"),
            PipelineType::ShadowAccumulatorDirectional => {
                include_shader!("shadow-accumulator-directional.vert")
            }
        };
        let frag_bytes: &'static [u8] = match pipeline_type {
            PipelineType::GBuffer => include_shader!("g-buffers-filler.frag"),
            PipelineType::Resolver => include_shader!("resolver.frag"),
            PipelineType::Deferred => include_shader!("deferred.frag"),
            PipelineType::ShadowMapper => include_shader!("shadow-mapper.frag"),
            PipelineType::ShadowAccumulatorDirectional => {
                include_shader!("shadow-accumulator-directional.frag")
            }
        };

        let vertex_shader = Module::new(vert_bytes, device.clone());
        let fragment_shader = Module::new(frag_bytes, device.clone());
        let shaders = vec![vertex_shader, fragment_shader];
        let layout = match pipeline_type {
            PipelineType::GBuffer => Layout::new_gbuff(descriptor_manager),
            PipelineType::Resolver => Layout::new_resolver(descriptor_manager),
            PipelineType::Deferred => Layout::new_deferred(descriptor_manager),
            PipelineType::ShadowMapper => Layout::new_shadow_mapper(descriptor_manager),
            PipelineType::ShadowAccumulatorDirectional => {
                Layout::new_shadow_accumulator_directional(descriptor_manager)
            }
        };

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
        rasterization_state.lineWidth = 1f32;

        let blend_attachment_state_size = render_pass.get_color_attachments().len();
        let mut blend_attachment_state =
            vec![vk::VkPipelineColorBlendAttachmentState::default(); blend_attachment_state_size];
        for i in 0..blend_attachment_state_size {
            match pipeline_type {
                PipelineType::Deferred => {
                    blend_attachment_state[i].blendEnable = vk::VK_TRUE;
                    blend_attachment_state[i].srcColorBlendFactor =
                        vk::VkBlendFactor::VK_BLEND_FACTOR_SRC_ALPHA;
                    blend_attachment_state[i].dstColorBlendFactor =
                        vk::VkBlendFactor::VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA;
                    blend_attachment_state[i].colorBlendOp = vk::VkBlendOp::VK_BLEND_OP_ADD;
                    blend_attachment_state[i].srcAlphaBlendFactor =
                        vk::VkBlendFactor::VK_BLEND_FACTOR_ONE;
                    blend_attachment_state[i].dstAlphaBlendFactor =
                        vk::VkBlendFactor::VK_BLEND_FACTOR_ZERO;
                    blend_attachment_state[i].alphaBlendOp = vk::VkBlendOp::VK_BLEND_OP_ADD;
                }
                PipelineType::ShadowAccumulatorDirectional => {
                    blend_attachment_state[i].blendEnable = vk::VK_TRUE;
                    blend_attachment_state[i].srcColorBlendFactor =
                        vk::VkBlendFactor::VK_BLEND_FACTOR_ONE;
                    blend_attachment_state[i].dstColorBlendFactor =
                        vk::VkBlendFactor::VK_BLEND_FACTOR_ONE;
                    blend_attachment_state[i].colorBlendOp = vk::VkBlendOp::VK_BLEND_OP_ADD;
                }
                _ => {}
            }
            blend_attachment_state[i].colorWriteMask =
                vk::VkColorComponentFlagBits::VK_COLOR_COMPONENT_R_BIT as vk::VkColorComponentFlags
                    | vk::VkColorComponentFlagBits::VK_COLOR_COMPONENT_G_BIT
                        as vk::VkColorComponentFlags
                    | vk::VkColorComponentFlagBits::VK_COLOR_COMPONENT_B_BIT
                        as vk::VkColorComponentFlags
                    | vk::VkColorComponentFlagBits::VK_COLOR_COMPONENT_A_BIT
                        as vk::VkColorComponentFlags;
        }

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
        depth_stencil_state.depthTestEnable = vk::VK_TRUE;
        depth_stencil_state.depthWriteEnable = vk::VK_TRUE;
        depth_stencil_state.depthCompareOp = vk::VkCompareOp::VK_COMPARE_OP_LESS_OR_EQUAL;
        depth_stencil_state.depthBoundsTestEnable = vk::VK_FALSE;
        depth_stencil_state.back.failOp = vk::VkStencilOp::VK_STENCIL_OP_KEEP;
        depth_stencil_state.back.passOp = vk::VkStencilOp::VK_STENCIL_OP_KEEP;
        depth_stencil_state.back.compareOp = vk::VkCompareOp::VK_COMPARE_OP_ALWAYS;
        depth_stencil_state.stencilTestEnable = vk::VK_FALSE;
        depth_stencil_state.front = depth_stencil_state.back;

        let mut multisample_state = vk::VkPipelineMultisampleStateCreateInfo::default();
        multisample_state.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO;
        if samples as u32 > vk::VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT as u32 {
            multisample_state.rasterizationSamples = samples;
            multisample_state.sampleShadingEnable = vk::VK_TRUE;
            multisample_state.minSampleShading = 0.25;
            multisample_state.alphaToCoverageEnable = vk::VK_TRUE;
        // multisample_state.alphaToOneEnable = vk::VK_TRUE;
        } else {
            multisample_state.rasterizationSamples =
                vk::VkSampleCountFlagBits::VK_SAMPLE_COUNT_1_BIT;
        }

        let mut vertex_input_binding = vk::VkVertexInputBindingDescription::default();
        vertex_input_binding.stride = 48; // bytes of vertex
        vertex_input_binding.inputRate = vk::VkVertexInputRate::VK_VERTEX_INPUT_RATE_VERTEX;

        let mut vertex_attributes = vec![vk::VkVertexInputAttributeDescription::default(); 4];
        vertex_attributes[0].format = vk::VkFormat::VK_FORMAT_R32G32B32_SFLOAT;
        vertex_attributes[1].location = 1;
        vertex_attributes[1].offset = 12;
        vertex_attributes[1].format = vk::VkFormat::VK_FORMAT_R32G32B32_SFLOAT;
        vertex_attributes[2].location = 2;
        vertex_attributes[2].offset = 24;
        vertex_attributes[2].format = vk::VkFormat::VK_FORMAT_R32G32B32A32_SFLOAT;
        vertex_attributes[3].location = 3;
        vertex_attributes[3].offset = 40;
        vertex_attributes[3].format = vk::VkFormat::VK_FORMAT_R32G32_SFLOAT;

        let mut vertex_input_state = vk::VkPipelineVertexInputStateCreateInfo::default();
        vertex_input_state.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO;
        match pipeline_type {
            PipelineType::GBuffer => {
                vertex_input_state.vertexBindingDescriptionCount = 1;
                vertex_input_state.pVertexBindingDescriptions = &vertex_input_binding;
                vertex_input_state.vertexAttributeDescriptionCount = vertex_attributes.len() as u32;
                vertex_input_state.pVertexAttributeDescriptions = vertex_attributes.as_ptr();
            }
            _ => {}
        }

        let stage_name = CString::new("main").unwrap();
        let stages_count = shaders.len();
        let mut shader_stages = vec![vk::VkPipelineShaderStageCreateInfo::default(); stages_count];
        for i in 0..stages_count {
            shader_stages[i].sType =
                vk::VkStructureType::VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO;
            shader_stages[i].pName = stage_name.as_ptr();
            shader_stages[i].module = shaders[i].vk_data;
            match i {
                0 => {
                    shader_stages[i].stage = vk::VkShaderStageFlagBits::VK_SHADER_STAGE_VERTEX_BIT;
                }
                1 => {
                    shader_stages[i].stage =
                        vk::VkShaderStageFlagBits::VK_SHADER_STAGE_FRAGMENT_BIT;
                }
                n @ _ => {
                    vxlogf!("Stage {} is not implemented yet!", n);
                }
            };
        }

        let mut pipeline_create_info = vk::VkGraphicsPipelineCreateInfo::default();
        pipeline_create_info.sType =
            vk::VkStructureType::VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO;
        pipeline_create_info.layout = layout.vk_data;
        pipeline_create_info.renderPass = render_pass.get_data();
        pipeline_create_info.stageCount = shader_stages.len() as u32;
        pipeline_create_info.pStages = shader_stages.as_ptr();
        pipeline_create_info.pVertexInputState = &vertex_input_state;
        pipeline_create_info.pInputAssemblyState = &input_assembly_state;
        pipeline_create_info.pRasterizationState = &rasterization_state;
        pipeline_create_info.pColorBlendState = &color_blend_state;
        pipeline_create_info.pMultisampleState = &multisample_state;
        pipeline_create_info.pViewportState = &viewport_state;
        pipeline_create_info.pDepthStencilState = &depth_stencil_state;
        pipeline_create_info.renderPass = render_pass.get_data();
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
            shaders,
            render_pass,
            vk_data,
        }
    }

    pub(super) fn get_info_for_binding(&self) -> (vk::VkPipelineBindPoint, vk::VkPipeline) {
        return (
            vk::VkPipelineBindPoint::VK_PIPELINE_BIND_POINT_GRAPHICS,
            self.vk_data,
        );
    }

    pub(crate) fn get_layout(&self) -> &Layout {
        return &self.layout;
    }
}

impl Drop for Pipeline {
    fn drop(&mut self) {
        unsafe {
            vk::vkDestroyPipeline(self.cache.logical_device.vk_data, self.vk_data, null());
        }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct Manager {
    cache: Arc<Cache>,
    descriptor_manager: Arc<RwLock<DescriptorManager>>,
    pipelines: BTreeMap<(usize, u8), Weak<Pipeline>>, // (renderpass, pipeline-type) -> pipeline
}

impl Manager {
    pub fn new(
        logical_device: Arc<LogicalDevice>,
        descriptor_manager: Arc<RwLock<DescriptorManager>>,
    ) -> Self {
        let cache = Arc::new(Cache::new(logical_device));
        Manager {
            cache,
            descriptor_manager,
            pipelines: BTreeMap::new(),
        }
    }

    pub(crate) fn create(
        &mut self,
        render_pass: Arc<RenderPass>,
        pipeline_type: PipelineType,
    ) -> Arc<Pipeline> {
        let samples = render_pass.get_vk_samples();
        let rpptr = unsafe { transmute(render_pass.get_data()) };
        let pt = pipeline_type as u8;
        let id = (rpptr, pt);
        if let Some(p) = self.pipelines.get(&id) {
            if let Some(p) = p.upgrade() {
                return p;
            }
        }
        let p = Arc::new(Pipeline::new(
            &self.descriptor_manager,
            render_pass,
            self.cache.clone(),
            samples,
            pipeline_type,
        ));
        self.pipelines.insert(id, Arc::downgrade(&p));
        return p;
    }
}
