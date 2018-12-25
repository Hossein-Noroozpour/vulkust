use super::super::core::types::Real;
use super::command::Buffer as CmdBuffer;
use super::config::Configurations;
use super::descriptor::Set as DescriptorSet;
use super::framebuffer::Framebuffer;
use super::g_buffer_filler::GBufferFiller;
use super::gapi::GraphicApiEngine;
use super::image::{
    AttachmentType, Format as ImageFormat, Layout as ImageLayout, View as ImageView,
};
use super::light::ShadowAccumulatorDirectionalUniform;
use super::pipeline::{Pipeline, PipelineType};
use super::render_pass::RenderPass;
use super::texture::{Manager as TextureManager, Texture};
use std::mem::size_of;
use std::sync::{Arc, RwLock};

use cgmath;

const SHADOW_MAP_FMT: ImageFormat = ImageFormat::DepthFloat;
const SHADOW_ACCUMULATOR_FLAGBITS_FMT: ImageFormat = ImageFormat::FlagBits8;

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Shadower {
    //---------------------------------------
    shadow_map_buffers: Vec<Arc<ImageView>>,
    shadow_map_render_pass: Arc<RenderPass>,
    shadow_map_framebuffers: Vec<Arc<Framebuffer>>,
    shadow_map_pipeline: Arc<Pipeline>,
    shadow_map_descriptor_set: Arc<DescriptorSet>,
    //---------------------------------------
    shadow_accumulator_flagbits_buffer: Arc<ImageView>,
    shadow_accumulator_flagbits_texture: Arc<RwLock<Texture>>,
    shadow_accumulator_directional_pipeline: Arc<Pipeline>,
    shadow_accumulator_directional_descriptor_set: Arc<DescriptorSet>,
    shadow_accumulator_render_pass: Arc<RenderPass>,
    shadow_accumulator_framebuffer: Arc<Framebuffer>,
    clear_shadow_accumulator_render_pass: Arc<RenderPass>,
    clear_shadow_accumulator_framebuffer: Arc<Framebuffer>,
}

impl Shadower {
    pub(super) fn new(
        geng: &GraphicApiEngine,
        conf: &Configurations,
        g_buffer_filler: &GBufferFiller,
        texture_manager: &mut TextureManager,
    ) -> Self {
        let dev = geng.get_device();
        let memmgr = geng.get_memory_manager();
        let mut shadow_map_buffers = Vec::with_capacity(conf.get_max_shadow_maps_count() as usize);
        let mut shadow_map_textures = Vec::with_capacity(conf.get_max_shadow_maps_count() as usize);
        let sampler = geng.get_linear_repeat_sampler();
        let accumulator_sampler = geng.get_nearest_repeat_sampler();
        for _ in 0..conf.get_max_shadow_maps_count() {
            let buf = Arc::new(ImageView::new_attachment(
                memmgr,
                SHADOW_MAP_FMT,
                AttachmentType::DepthShadowBuffer,
                conf.get_shadow_map_aspect(),
                conf.get_shadow_map_aspect(),
            ));
            shadow_map_textures
                .push(texture_manager.create_2d_with_view_sampler(buf.clone(), sampler.clone()));
            shadow_map_buffers.push(buf);
        }
        let shadow_accumulator_flagbits_buffer = Arc::new(ImageView::new_surface_attachment(
            dev.clone(),
            memmgr,
            SHADOW_ACCUMULATOR_FLAGBITS_FMT,
            AttachmentType::ShadowAccumulator,
        ));
        let shadow_accumulator_flagbits_texture = texture_manager.create_2d_with_view_sampler(
            shadow_accumulator_flagbits_buffer.clone(),
            accumulator_sampler.clone(),
        );
        let shadow_accumulator_buffers = vec![shadow_accumulator_flagbits_buffer.clone()];
        let clear_shadow_accumulator_render_pass = Arc::new(RenderPass::new_with_layouts(
            shadow_accumulator_buffers.clone(),
            true,
            &[ImageLayout::Uninitialized],
            &[ImageLayout::ShaderReadOnly],
        ));
        let shadow_accumulator_render_pass = Arc::new(RenderPass::new_with_layouts(
            shadow_accumulator_buffers.clone(),
            false,
            &[ImageLayout::ShaderReadOnly],
            &[ImageLayout::ShaderReadOnly],
        ));
        let shadow_map_render_pass = Arc::new(RenderPass::new(
            vec![shadow_map_buffers[0].clone()],
            true,
            true,
        ));
        let clear_shadow_accumulator_framebuffer = Arc::new(Framebuffer::new(
            shadow_accumulator_buffers.clone(),
            clear_shadow_accumulator_render_pass.clone(),
        ));
        let shadow_accumulator_framebuffer = Arc::new(Framebuffer::new(
            shadow_accumulator_buffers.clone(),
            shadow_accumulator_render_pass.clone(),
        ));
        let mut shadow_map_framebuffers = Vec::with_capacity(shadow_map_buffers.len());
        for v in &shadow_map_buffers {
            shadow_map_framebuffers.push(Arc::new(Framebuffer::new(
                vec![v.clone()],
                shadow_map_render_pass.clone(),
            )));
        }
        let (shadow_mapper_uniform_buffer, shadow_accumulator_directional_uniform_buffer) = {
            let mut bufmgr = vxresult!(geng.get_buffer_manager().write());
            (
                bufmgr.create_dynamic_buffer(size_of::<ShadowMapperUniform>() as isize),
                bufmgr.create_dynamic_buffer(
                    size_of::<ShadowAccumulatorDirectionalUniform>() as isize
                ),
            )
        };
        let (shadow_map_descriptor_set, shadow_accumulator_directional_descriptor_set) = {
            let mut desmgr = vxresult!(geng.get_descriptor_manager().write());
            let gbufftex = g_buffer_filler.get_textures();
            (
                desmgr.create_buffer_only_set(&shadow_mapper_uniform_buffer),
                desmgr.create_shadow_accumulator_directional_set(
                    &shadow_accumulator_directional_uniform_buffer,
                    vec![
                        vec![gbufftex[0].clone()],
                        vec![gbufftex[1].clone()],
                        shadow_map_textures[0..conf.get_cascaded_shadows_count() as usize].to_vec(),
                    ],
                ),
            )
        };
        let (shadow_map_pipeline, shadow_accumulator_directional_pipeline) = {
            let mut pipmgr = vxresult!(geng.get_pipeline_manager().write());
            (
                pipmgr.create(
                    shadow_map_render_pass.clone(),
                    PipelineType::ShadowMapper,
                    conf,
                ),
                pipmgr.create(
                    shadow_accumulator_render_pass.clone(),
                    PipelineType::ShadowAccumulatorDirectional,
                    conf,
                ),
            )
        };
        Self {
            shadow_map_buffers,
            shadow_map_render_pass,
            shadow_map_framebuffers,
            shadow_map_pipeline,
            shadow_map_descriptor_set,
            //---------------------------------------
            shadow_accumulator_flagbits_buffer,
            shadow_accumulator_flagbits_texture,
            shadow_accumulator_directional_pipeline,
            shadow_accumulator_directional_descriptor_set,
            shadow_accumulator_render_pass,
            shadow_accumulator_framebuffer,
            clear_shadow_accumulator_render_pass,
            clear_shadow_accumulator_framebuffer,
        }
    }

    pub(super) fn begin_secondary_shadow_mappers(&self, cmds: &mut [CmdBuffer]) {
        let cmds_len = cmds.len();
        for i in 0..cmds_len {
            cmds[i].begin_secondary(&self.shadow_map_framebuffers[i]);
            cmds[i].bind_pipeline(&self.shadow_map_pipeline);
        }
    }

    pub(super) fn begin_shadow_map_primary(&self, cmd: &mut CmdBuffer, map_index: usize) {
        self.shadow_map_framebuffers[map_index].begin(cmd);
    }

    pub(crate) fn get_shadow_map_descriptor_set(&self) -> &DescriptorSet {
        return &self.shadow_map_descriptor_set;
    }

    pub(crate) fn get_shadow_accumulator_directional_descriptor_set(&self) -> &DescriptorSet {
        return &self.shadow_accumulator_directional_descriptor_set;
    }

    pub(crate) fn get_accumulator_framebuffer(&self) -> &Arc<Framebuffer> {
        return &self.shadow_accumulator_framebuffer;
    }

    pub(crate) fn get_shadow_accumulator_directional_pipeline(&self) -> &Arc<Pipeline> {
        return &self.shadow_accumulator_directional_pipeline;
    }

    pub(crate) fn clear_shadow_accumulator(&self, cmd: &mut CmdBuffer) {
        cmd.begin();
        self.clear_shadow_accumulator_framebuffer.begin(cmd);
        cmd.end_render_pass();
        cmd.end();
    }

    pub(super) fn get_shadow_accumulator_flagbits_texture(&self) -> &Arc<RwLock<Texture>> {
        return &self.shadow_accumulator_flagbits_texture;
    }
}

unsafe impl Send for Shadower {}
unsafe impl Sync for Shadower {}

#[repr(C)]
struct ShadowMapperUniform {
    mvp: cgmath::Matrix4<Real>,
}
