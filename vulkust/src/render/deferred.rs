use super::buffer::DynamicBuffer;
use super::command::Buffer as CmdBuffer;
use super::descriptor::Set as DescriptorSet;
use super::gapi::GraphicApiEngine;
use super::config::Configurations;
use super::pipeline::{Pipeline, PipelineType};
use super::shadower::Shadower;
use super::resolver::Resolver;
use std::mem::size_of;
use std::sync::Arc;

#[repr(C)]
#[cfg_attr(debug_mode, derive(Debug))]
struct Uniform {
    pixel_x_step: f32,
    pixel_y_step: f32,
}

impl Uniform {
    pub fn new(window_width: f32, window_height: f32) -> Self {
        Uniform {
            pixel_x_step: 2f32 / window_width,
            pixel_y_step: 2f32 / window_height,
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
    pub(crate) fn new(gapi_engine: &GraphicApiEngine, resolver: &Resolver, shadower: &Shadower, config: &Configurations) -> Self {
        let resolver_framebuffer = resolver.get_framebuffer();
        let (w, h) = resolver_framebuffer.get_dimensions();
        let uniform = Uniform::new(w as f32, h as f32);
        let uniform_buffer = vxresult!(gapi_engine.get_buffer_manager().write())
            .create_dynamic_buffer(size_of::<Uniform>() as isize);
        let mut textures = Vec::with_capacity(resolver.get_output_textures().len() + 2);
        for t in resolver.get_output_textures() {
            textures.push(t.clone());
        }
        textures.push(shadower.get_shadow_accumulator_strength_texture().clone());
        textures.push(shadower.get_shadow_accumulator_flagbits_texture().clone());
        let descriptor_set = vxresult!(gapi_engine.get_descriptor_manager().write())
            .create_deferred_set(&uniform_buffer, textures);
        let descriptor_set = Arc::new(descriptor_set);
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
