use super::buffer::DynamicBuffer;
use super::command::Buffer as CmdBuffer;
use super::descriptor::Set as DescriptorSet;
use super::gapi::GraphicApiEngine;
use super::pipeline::{Pipeline, PipelineType};
use super::resolver::Resolver;
use super::scene::Manager as SceneManager;
use std::mem::size_of;
use std::sync::{Arc, RwLock};

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
pub(super) struct Deferred {
    uniform: Uniform,
    uniform_buffer: DynamicBuffer,
    descriptor_set: Arc<DescriptorSet>,
    pipeline: Arc<Pipeline>,
}

impl Deferred {
    pub fn new(
        gapi_engine: &GraphicApiEngine,
        scene_manager: &SceneManager,
        resolver: &Resolver,
    ) -> Self {
        let resolver_framebuffer = resolver.get_framebuffer();
        let (w, h) = resolver_framebuffer.get_dimensions();
        let uniform = Uniform::new(w as f32, h as f32);
        let uniform_buffer =
                vxresult!(gapi_engine.get_buffer_manager().write())
                    .create_dynamic_buffer(size_of::<Uniform>() as isize);
        let sampler = gapi_engine.get_linear_repeat_sampler();
        let mut texture_manager = vxresult!(scene_manager.texture_manager.write());
        let mut textures = Vec::new();
        let resolver_buffers = resolver.get_buffers();
        for v in resolver_buffers {
            textures.push(texture_manager.create_2d_with_view_sampler(v.clone(), sampler.clone()));
        }
        textures.shrink_to_fit();
        let descriptor_set = vxresult!(gapi_engine.get_descriptor_manager().write())
            .create_deferred_set(&uniform_buffer, textures.clone());
        let descriptor_set = Arc::new(descriptor_set);
        let mut pipmgr = vxresult!(gapi_engine.get_pipeline_manager().write());
        let render_pass = gapi_engine.get_render_pass();
        let pipeline = pipmgr.create(render_pass.clone(), PipelineType::Deferred);
        Deferred {
            uniform,
            uniform_buffer,
            descriptor_set,
            pipeline,
        }
    }

    pub fn render(&mut self, cmd: &mut CmdBuffer, frame_number: usize) {
        self.uniform_buffer.update(&self.uniform, frame_number);
        let buffer = self.uniform_buffer.get_buffer(frame_number);
        let buffer = vxresult!(buffer.read());
        cmd.bind_pipeline(&self.pipeline);
        cmd.bind_deferred_deferred_descriptor(&*self.descriptor_set, &*buffer);
    }
}
