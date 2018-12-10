use super::super::core::types::Real;
use super::buffer::Dynamic as DynamicBuffer;
use super::command::Buffer as CmdBuffer;
use super::config::Configurations;
use super::descriptor::Set as DescriptorSet;
use super::g_buffer_filler::GBufferFiller;
use super::gapi::GraphicApiEngine;
use super::pipeline::{Pipeline, PipelineType};
use super::texture::{Manager as TextureManager, Texture};
use super::image::{AttachmentType, View as ImageView, Format};
use std::mem::size_of;
use std::sync::{Arc, RwLock};

use math;

#[repr(C)]
#[cfg_attr(debug_mode, derive(Debug))]
struct Uniform {
    reserved: math::Vector4<Real>,
}

impl Uniform {
    pub fn new() -> Self {
        Uniform {
            reserved: math::Vector4::new(0.0, 0.0, 0.0, 0.0),
        }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct SSAO {
    uniform: Uniform,
    uniform_buffer: DynamicBuffer,
    descriptor_set: Arc<DescriptorSet>,
    pipeline: Arc<Pipeline>,
    textures: Vec<Arc<RwLock<Texture>>>,
}

impl SSAO {
    pub(crate) fn new(
        eng: &GraphicApiEngine,
        texmgr: &mut TextureManager,
        g_buffer_filler: &GBufferFiller,
        config: &Configurations,
    ) -> Self {
        let gbuff_framebuffer = g_buffer_filler.get_framebuffer();
        let dev = eng.get_device();
        let memmgr = eng.get_memory_manager();
        let buffers = vec![
            Arc::new(ImageView::new_surface_attachment(
                dev.clone(),
                memmgr,
                Format::Float,
                AttachmentType::ColorGBuffer,
            )),
        ];
        let (w, h) = gbuff_framebuffer.get_dimensions();
        let uniform = Uniform::new();
        let uniform_buffer = vxresult!(eng.get_buffer_manager().write())
            .create_dynamic_buffer(size_of::<Uniform>() as isize);
        let mut textures = Vec::with_capacity(3); // position, normal, depth
        textures.push(g_buffer_filler.get_position_texture().clone());
        textures.push(g_buffer_filler.get_normal_texture().clone());
        textures.push(g_buffer_filler.get_depth_texture().clone());
        let descriptor_set = vxresult!(eng.get_descriptor_manager().write())
            .create_deferred_set(&uniform_buffer, textures);
        let render_pass = eng.get_render_pass();
        let pipeline = vxresult!(eng.get_pipeline_manager().write()).create(
            render_pass.clone(), PipelineType::SSAO, config);
        let sampler = eng.get_linear_repeat_sampler();
        let mut textures = Vec::with_capacity(buffers.len());
        for b in &buffers {
            textures.push(texmgr.create_2d_with_view_sampler(b.clone(), sampler.clone()));
        }
        Self {
            uniform,
            uniform_buffer,
            descriptor_set,
            pipeline,
            textures,
        }
    }

    pub(crate) fn render(&self, cmd: &mut CmdBuffer, frame_number: usize) {
        let buffer = self.uniform_buffer.get_buffer(frame_number);
        let buffer = vxresult!(buffer.read());
        cmd.bind_pipeline(&self.pipeline);
        cmd.bind_ssao_ssao_descriptor(&*self.descriptor_set, &*buffer);
    }

    pub(crate) fn update(&mut self, frame_number: usize) {
        self.uniform_buffer.update(&self.uniform, frame_number);
    }
}
