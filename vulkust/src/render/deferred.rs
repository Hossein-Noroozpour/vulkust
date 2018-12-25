use super::super::core::types::Real;
use super::buffer::Dynamic as DynamicBuffer;
use super::command::Buffer as CmdBuffer;
use super::config::Configurations;
use super::descriptor::Set as DescriptorSet;
use super::g_buffer_filler::GBufferFiller;
use super::gapi::GraphicApiEngine;
use super::pipeline::{Pipeline, PipelineType};
use super::shadower::Shadower;
use super::ssao::SSAO;
use super::texture::Manager as TextureManager;
use std::mem::size_of;
use std::sync::Arc;

use cgmath;

#[repr(C)]
#[cfg_attr(debug_mode, derive(Debug))]
struct Uniform {
    pixel_step: cgmath::Vector4<Real>,
}

impl Uniform {
    pub fn new(window_width: Real, window_height: Real) -> Self {
        Uniform {
            pixel_step: cgmath::Vector4::new(1f32 / window_width, 1f32 / window_height, 0.0, 0.0),
        }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Deferred {
    uniform: Uniform,
    uniform_buffer: DynamicBuffer,
    descriptor_set: Arc<DescriptorSet>,
    pipeline: Arc<Pipeline>,
}

impl Deferred {
    pub(crate) fn new(
        gapi_engine: &GraphicApiEngine,
        g_buffer_filler: &GBufferFiller,
        shadower: &Shadower,
        ssao: Option<&SSAO>,
        config: &Configurations,
        texmgr: &mut TextureManager,
    ) -> Self {
        let gbuff_framebuffer = g_buffer_filler.get_framebuffer();
        let (w, h) = gbuff_framebuffer.get_dimensions();
        let uniform = Uniform::new(w as f32, h as f32);
        let uniform_buffer = vxresult!(gapi_engine.get_buffer_manager().write())
            .create_dynamic_buffer(size_of::<Uniform>() as isize);
        let mut textures = Vec::with_capacity(g_buffer_filler.get_textures().len() + 2);
        for t in g_buffer_filler.get_textures() {
            textures.push(t.clone());
        }
        if let Some(ssao) = ssao {
            textures.push(ssao.get_ambient_occlusion_texture().clone());
        } else {
            textures.push(texmgr.create_2d_with_pixels(2, 2, gapi_engine, &[255u8; 2 * 2 * 4]));
        }
        textures.push(shadower.get_shadow_accumulator_flagbits_texture().clone());
        let descriptor_set = vxresult!(gapi_engine.get_descriptor_manager().write())
            .create_deferred_set(&uniform_buffer, textures);
        let mut pipmgr = vxresult!(gapi_engine.get_pipeline_manager().write());
        let render_pass = gapi_engine.get_render_pass();
        let pipeline = pipmgr.create(render_pass.clone(), PipelineType::Deferred, config);
        Deferred {
            uniform,
            uniform_buffer,
            descriptor_set,
            pipeline,
        }
    }

    pub(crate) fn render(&self, cmd: &mut CmdBuffer, frame_number: usize) {
        let buffer = self.uniform_buffer.get_buffer(frame_number);
        let buffer = vxresult!(buffer.read());
        cmd.bind_pipeline(&self.pipeline);
        cmd.bind_deferred_deferred_descriptor(&*self.descriptor_set, &*buffer);
    }

    pub(crate) fn update(&mut self, frame_number: usize) {
        self.uniform_buffer.update(&self.uniform, frame_number);
    }
}
