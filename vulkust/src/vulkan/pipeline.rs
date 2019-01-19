use super::super::render::config::Configurations;
use super::super::render::pipeline::PipelineType;
use super::descriptor::{Manager as DescriptorManager, SetLayout as DescriptorSetLayout};
use super::device::Logical as LogicalDevice;
use super::render_pass::RenderPass;
use super::shader::Module;
use ash::version::DeviceV1_0;
use ash::vk;
use std::collections::BTreeMap;
use std::ffi::CString;
use std::mem::{size_of, transmute};
use std::sync::{Arc, RwLock, Weak};

macro_rules! include_shader {
    ($name:expr) => {
        include_bytes!(concat!(env!("OUT_DIR"), "/vulkan/shaders/", $name, ".spv"))
    };
}

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct Layout {
    descriptor_set_layouts: Vec<Arc<DescriptorSetLayout>>,
    vk_data: vk::PipelineLayout,
}

impl Layout {
    pub fn new_gbuff(descriptor_manager: &Arc<RwLock<DescriptorManager>>) -> Self {
        let descriptor_manager = vxresult!(descriptor_manager.read());
        let gbuff_descriptor_set_layout = descriptor_manager.get_gbuff_set_layout().clone();
        let buffer_only_descriptor_set_layout =
            descriptor_manager.get_buffer_only_set_layout().clone();
        let layout = [
            *buffer_only_descriptor_set_layout.get_data(),
            *buffer_only_descriptor_set_layout.get_data(),
            *gbuff_descriptor_set_layout.get_data(),
        ];
        let descriptor_set_layouts = vec![
            gbuff_descriptor_set_layout,
            buffer_only_descriptor_set_layout,
        ];
        Self::new(&layout, descriptor_set_layouts)
    }

    pub fn new_unlit(descriptor_manager: &Arc<RwLock<DescriptorManager>>) -> Self {
        let descriptor_manager = vxresult!(descriptor_manager.read());
        let unlit_descriptor_set_layout = descriptor_manager.get_gbuff_set_layout().clone();
        let buffer_only_descriptor_set_layout =
            descriptor_manager.get_buffer_only_set_layout().clone();
        let layout = [
            *buffer_only_descriptor_set_layout.get_data(),
            *unlit_descriptor_set_layout.get_data(),
        ];
        let descriptor_set_layouts = vec![
            unlit_descriptor_set_layout,
            buffer_only_descriptor_set_layout,
        ];
        Self::new(&layout, descriptor_set_layouts)
    }

    pub fn new_shadow_mapper(descriptor_manager: &Arc<RwLock<DescriptorManager>>) -> Self {
        let descriptor_manager = vxresult!(descriptor_manager.read());
        let gbuff_descriptor_set_layout = descriptor_manager.get_gbuff_set_layout().clone();
        let buffer_only_descriptor_set_layout =
            descriptor_manager.get_buffer_only_set_layout().clone();
        let layout = [
            *buffer_only_descriptor_set_layout.get_data(),
            *gbuff_descriptor_set_layout.get_data(),
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
            .get_shadow_accumulator_directional_set_layout()
            .clone();
        let layout = [*shadow_accumulator_directional_descriptor_set_layout.get_data()];
        let descriptor_set_layouts = vec![shadow_accumulator_directional_descriptor_set_layout];
        Self::new(&layout, descriptor_set_layouts)
    }

    pub fn new_deferred(descriptor_manager: &Arc<RwLock<DescriptorManager>>) -> Self {
        let descriptor_manager = vxresult!(descriptor_manager.read());
        let deferred_descriptor_set_layout = descriptor_manager.get_deferred_set_layout().clone();
        let buffer_only_descriptor_set_layout =
            descriptor_manager.get_buffer_only_set_layout().clone();
        let layout = [
            *buffer_only_descriptor_set_layout.get_data(),
            *deferred_descriptor_set_layout.get_data(),
        ];
        let descriptor_set_layouts = vec![
            buffer_only_descriptor_set_layout,
            deferred_descriptor_set_layout,
        ];
        Self::new(&layout, descriptor_set_layouts)
    }

    pub fn new_ssao(descriptor_manager: &Arc<RwLock<DescriptorManager>>) -> Self {
        let descriptor_manager = vxresult!(descriptor_manager.read());
        let ssao_descriptor_set_layout = descriptor_manager.get_ssao_set_layout().clone();
        let buffer_only_descriptor_set_layout =
            descriptor_manager.get_buffer_only_set_layout().clone();
        let layout = [
            *buffer_only_descriptor_set_layout.get_data(),
            *ssao_descriptor_set_layout.get_data(),
        ];
        let descriptor_set_layouts = vec![
            buffer_only_descriptor_set_layout,
            ssao_descriptor_set_layout,
        ];
        Self::new(&layout, descriptor_set_layouts)
    }

    fn new(
        layout: &[vk::DescriptorSetLayout],
        descriptor_set_layouts: Vec<Arc<DescriptorSetLayout>>,
    ) -> Self {
        let mut pipeline_layout_create_info = vk::PipelineLayoutCreateInfo::default();
        pipeline_layout_create_info.set_layout_count = layout.len() as u32;
        pipeline_layout_create_info.p_set_layouts = layout.as_ptr();
        let vk_data = vxresult!(unsafe {
            descriptor_set_layouts[0]
                .get_logical_device()
                .get_data()
                .create_pipeline_layout(&pipeline_layout_create_info, None)
        });
        Self {
            descriptor_set_layouts,
            vk_data,
        }
    }

    pub(super) fn get_data(&self) -> &vk::PipelineLayout {
        return &self.vk_data;
    }
}

impl Drop for Layout {
    fn drop(&mut self) {
        unsafe {
            self.descriptor_set_layouts[0]
                .get_logical_device()
                .get_data()
                .destroy_pipeline_layout(self.vk_data, None);
        }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
struct Cache {
    logical_device: Arc<LogicalDevice>,
    vk_data: vk::PipelineCache,
}

impl Cache {
    fn new(logical_device: Arc<LogicalDevice>) -> Self {
        let pipeline_cache_create_info = vk::PipelineCacheCreateInfo::default();
        let vk_data = vxresult!(unsafe {
            logical_device
                .get_data()
                .create_pipeline_cache(&pipeline_cache_create_info, None)
        });
        Self {
            logical_device,
            vk_data,
        }
    }
}

impl Drop for Cache {
    fn drop(&mut self) {
        unsafe {
            self.logical_device
                .get_data()
                .destroy_pipeline_cache(self.vk_data, None);
        }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct Pipeline {
    cache: Arc<Cache>,
    layout: Layout,
    shaders: Vec<Module>,
    render_pass: Arc<RenderPass>,
    vk_data: vk::Pipeline,
}

impl Pipeline {
    fn new(
        descriptor_manager: &Arc<RwLock<DescriptorManager>>,
        render_pass: Arc<RenderPass>,
        cache: Arc<Cache>,
        pipeline_type: PipelineType,
        config: &Configurations,
    ) -> Self {
        let device = vxresult!(descriptor_manager.read())
            .get_pool()
            .get_logical_device()
            .clone();

        let vert_bytes: &'static [u8] = match pipeline_type {
            PipelineType::GBuffer => include_shader!("g-buffers-filler.vert"),
            PipelineType::Deferred => include_shader!("deferred.vert"),
            PipelineType::ShadowMapper => include_shader!("shadow-mapper.vert"),
            PipelineType::ShadowAccumulatorDirectional => {
                include_shader!("shadow-accumulator-directional.vert")
            }
            PipelineType::SSAO => include_shader!("ssao.vert"),
            PipelineType::Unlit => include_shader!("unlit.vert"),
            PipelineType::TransparentPBR => include_shader!("transparent_pbr.vert"),
        };
        let frag_bytes: &'static [u8] = match pipeline_type {
            PipelineType::GBuffer => include_shader!("g-buffers-filler.frag"),
            PipelineType::Deferred => include_shader!("deferred.frag"),
            PipelineType::ShadowMapper => include_shader!("shadow-mapper.frag"),
            PipelineType::ShadowAccumulatorDirectional => {
                include_shader!("shadow-accumulator-directional.frag")
            }
            PipelineType::SSAO => include_shader!("ssao.frag"),
            PipelineType::Unlit => include_shader!("unlit.frag"),
            PipelineType::TransparentPBR => include_shader!("transparent_pbr.frag"),
        };

        let vertex_shader = Module::new(vert_bytes, device.clone());
        let fragment_shader = Module::new(frag_bytes, device.clone());
        let shaders = vec![vertex_shader, fragment_shader];
        let layout = match pipeline_type {
            PipelineType::GBuffer | PipelineType::TransparentPBR => {
                Layout::new_gbuff(descriptor_manager)
            }
            PipelineType::Deferred => Layout::new_deferred(descriptor_manager),
            PipelineType::ShadowMapper => Layout::new_shadow_mapper(descriptor_manager),
            PipelineType::ShadowAccumulatorDirectional => {
                Layout::new_shadow_accumulator_directional(descriptor_manager)
            }
            PipelineType::SSAO => Layout::new_ssao(descriptor_manager),
            PipelineType::Unlit => Layout::new_unlit(descriptor_manager),
        };

        let mut input_assembly_state = vk::PipelineInputAssemblyStateCreateInfo::default();
        input_assembly_state.topology = vk::PrimitiveTopology::TRIANGLE_LIST;

        let mut rasterization_state = vk::PipelineRasterizationStateCreateInfo::default();
        rasterization_state.polygon_mode = vk::PolygonMode::FILL;
        rasterization_state.cull_mode = vk::CullModeFlags::FRONT;
        rasterization_state.front_face = vk::FrontFace::CLOCKWISE;
        rasterization_state.line_width = 1f32;

        let blend_attachment_state_size = render_pass.get_color_attachments().len();
        let mut blend_attachment_state =
            vec![vk::PipelineColorBlendAttachmentState::default(); blend_attachment_state_size];
        for i in 0..blend_attachment_state_size {
            match pipeline_type {
                PipelineType::Deferred | PipelineType::Unlit | PipelineType::TransparentPBR => {
                    blend_attachment_state[i].blend_enable = vk::TRUE;
                    blend_attachment_state[i].src_color_blend_factor = vk::BlendFactor::SRC_ALPHA;
                    blend_attachment_state[i].dst_color_blend_factor =
                        vk::BlendFactor::ONE_MINUS_SRC_ALPHA;
                    blend_attachment_state[i].color_blend_op = vk::BlendOp::ADD;
                    blend_attachment_state[i].src_alpha_blend_factor = vk::BlendFactor::ONE;
                    blend_attachment_state[i].dst_alpha_blend_factor = vk::BlendFactor::ZERO;
                    blend_attachment_state[i].alpha_blend_op = vk::BlendOp::ADD;
                    blend_attachment_state[i].color_write_mask = vk::ColorComponentFlags::R
                        | vk::ColorComponentFlags::G
                        | vk::ColorComponentFlags::B
                        | vk::ColorComponentFlags::A;
                }
                PipelineType::ShadowAccumulatorDirectional => {
                    blend_attachment_state[i].blend_enable = vk::TRUE;
                    blend_attachment_state[i].src_color_blend_factor = vk::BlendFactor::ONE;
                    blend_attachment_state[i].dst_color_blend_factor = vk::BlendFactor::ONE;
                    blend_attachment_state[i].color_blend_op = vk::BlendOp::ADD;
                    blend_attachment_state[i].color_write_mask = vk::ColorComponentFlags::R
                        | vk::ColorComponentFlags::G
                        | vk::ColorComponentFlags::B
                        | vk::ColorComponentFlags::A;
                }
                PipelineType::SSAO => {
                    blend_attachment_state[i].color_write_mask = vk::ColorComponentFlags::R;
                }
                _ => {
                    blend_attachment_state[i].color_write_mask = vk::ColorComponentFlags::R
                        | vk::ColorComponentFlags::G
                        | vk::ColorComponentFlags::B
                        | vk::ColorComponentFlags::A;
                }
            }
        }

        let mut color_blend_state = vk::PipelineColorBlendStateCreateInfo::default();
        color_blend_state.attachment_count = blend_attachment_state.len() as u32;
        color_blend_state.p_attachments = blend_attachment_state.as_ptr();

        let mut viewport_state = vk::PipelineViewportStateCreateInfo::default();
        viewport_state.viewport_count = 1;
        viewport_state.scissor_count = 1;

        let dynamic_state_enables = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];

        let mut dynamic_state = vk::PipelineDynamicStateCreateInfo::default();
        dynamic_state.p_dynamic_states = dynamic_state_enables.as_ptr();
        dynamic_state.dynamic_state_count = dynamic_state_enables.len() as u32;

        let mut depth_stencil_state = vk::PipelineDepthStencilStateCreateInfo::default();
        depth_stencil_state.depth_test_enable = vk::TRUE;
        depth_stencil_state.depth_write_enable = vk::TRUE;
        depth_stencil_state.depth_compare_op = vk::CompareOp::LESS_OR_EQUAL;
        depth_stencil_state.depth_bounds_test_enable = vk::FALSE;
        depth_stencil_state.back.fail_op = vk::StencilOp::KEEP;
        depth_stencil_state.back.pass_op = vk::StencilOp::KEEP;
        depth_stencil_state.back.compare_op = vk::CompareOp::ALWAYS;
        depth_stencil_state.stencil_test_enable = vk::FALSE;
        depth_stencil_state.front = depth_stencil_state.back;

        let mut multisample_state = vk::PipelineMultisampleStateCreateInfo::default();
        multisample_state.rasterization_samples = vk::SampleCountFlags::TYPE_1;

        let mut vertex_input_binding = vk::VertexInputBindingDescription::default();
        vertex_input_binding.stride = 48; // bytes of vertex
        vertex_input_binding.input_rate = vk::VertexInputRate::VERTEX;

        let mut vertex_attributes = vec![vk::VertexInputAttributeDescription::default(); 4];
        vertex_attributes[0].format = vk::Format::R32G32B32_SFLOAT;
        vertex_attributes[1].location = 1;
        vertex_attributes[1].offset = 12;
        vertex_attributes[1].format = vk::Format::R32G32B32_SFLOAT;
        vertex_attributes[2].location = 2;
        vertex_attributes[2].offset = 24;
        vertex_attributes[2].format = vk::Format::R32G32B32A32_SFLOAT;
        vertex_attributes[3].location = 3;
        vertex_attributes[3].offset = 40;
        vertex_attributes[3].format = vk::Format::R32G32_SFLOAT;

        let mut vertex_input_state = vk::PipelineVertexInputStateCreateInfo::default();
        match pipeline_type {
            PipelineType::GBuffer
            | PipelineType::ShadowMapper
            | PipelineType::Unlit
            | PipelineType::TransparentPBR => {
                vertex_input_state.vertex_binding_description_count = 1;
                vertex_input_state.p_vertex_binding_descriptions = &vertex_input_binding;
                vertex_input_state.vertex_attribute_description_count =
                    vertex_attributes.len() as u32;
                vertex_input_state.p_vertex_attribute_descriptions = vertex_attributes.as_ptr();
            }
            _ => {}
        }

        let cascades_count = config.get_cascaded_shadows_count() as u32;

        let mut specialization_map_entries = match pipeline_type {
            PipelineType::ShadowAccumulatorDirectional => {
                vec![vk::SpecializationMapEntry::default(); 1]
            }
            _ => Vec::new(),
        };

        let mut specialization_info = vk::SpecializationInfo::default();

        match pipeline_type {
            PipelineType::ShadowAccumulatorDirectional => {
                specialization_map_entries[0].constant_id = 0;
                specialization_map_entries[0].size = size_of::<u32>();
                specialization_map_entries[0].offset = 0;

                specialization_info.data_size = size_of::<u32>();
                specialization_info.map_entry_count = specialization_map_entries.len() as u32;
                specialization_info.p_map_entries = specialization_map_entries.as_ptr();
                specialization_info.p_data = unsafe { transmute(&cascades_count) };
            }
            _ => {}
        };

        let stage_name = CString::new("main").unwrap();
        let stages_count = shaders.len();
        let mut shader_stages = vec![vk::PipelineShaderStageCreateInfo::default(); stages_count];
        for i in 0..stages_count {
            shader_stages[i].p_name = stage_name.as_ptr();
            shader_stages[i].module = *shaders[i].get_data();
            match i {
                0 => {
                    shader_stages[i].stage = vk::ShaderStageFlags::VERTEX;
                }
                1 => {
                    shader_stages[i].stage = vk::ShaderStageFlags::FRAGMENT;
                }
                n @ _ => {
                    vxlogf!("Stage {} is not implemented yet!", n);
                }
            };
            match pipeline_type {
                PipelineType::ShadowAccumulatorDirectional => {
                    shader_stages[i].p_specialization_info = &specialization_info;
                }
                _ => {}
            }
        }

        let mut pipeline_create_info = vk::GraphicsPipelineCreateInfo::default();
        pipeline_create_info.layout = layout.vk_data;
        pipeline_create_info.render_pass = *render_pass.get_data();
        pipeline_create_info.stage_count = shader_stages.len() as u32;
        pipeline_create_info.p_stages = shader_stages.as_ptr();
        pipeline_create_info.p_vertex_input_state = &vertex_input_state;
        pipeline_create_info.p_input_assembly_state = &input_assembly_state;
        pipeline_create_info.p_rasterization_state = &rasterization_state;
        pipeline_create_info.p_color_blend_state = &color_blend_state;
        pipeline_create_info.p_multisample_state = &multisample_state;
        pipeline_create_info.p_viewport_state = &viewport_state;
        pipeline_create_info.p_depth_stencil_state = &depth_stencil_state;
        pipeline_create_info.p_dynamic_state = &dynamic_state;

        let vkdev = device.get_data();

        let vk_data = vxresult!(unsafe {
            vkdev.create_graphics_pipelines(cache.vk_data, &[pipeline_create_info], None)
        });
        let vk_data = vk_data[0];
        Self {
            cache,
            layout,
            shaders,
            render_pass,
            vk_data,
        }
    }

    pub(super) fn get_info_for_binding(&self) -> (vk::PipelineBindPoint, vk::Pipeline) {
        return (vk::PipelineBindPoint::GRAPHICS, self.vk_data);
    }

    pub(crate) fn get_layout(&self) -> &Layout {
        return &self.layout;
    }
}

impl Drop for Pipeline {
    fn drop(&mut self) {
        unsafe {
            self.cache
                .logical_device
                .get_data()
                .destroy_pipeline(self.vk_data, None);
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
    pub(super) fn new(
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
        config: &Configurations,
    ) -> Arc<Pipeline> {
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
            pipeline_type,
            config,
        ));
        self.pipelines.insert(id, Arc::downgrade(&p));
        return p;
    }
}
