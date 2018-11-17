use super::super::core::types::Real;
use super::buffer::DynamicBuffer;
use super::command::Buffer as CmdBuffer;
use super::config::Configurations;
use super::descriptor::Set as DescriptorSet;
use super::framebuffer::Framebuffer;
use super::g_buffer_filler::GBufferFiller;
use super::gapi::GraphicApiEngine;
use super::image::{AttachmentType, Format, View as ImageView};
use super::pipeline::{Pipeline, PipelineType};
use super::render_pass::RenderPass;
use super::texture::{Manager as TextureManager, Texture};
use std::mem::size_of;
use std::sync::{Arc, RwLock};

fn convert_format_to_resolver_format(f: Format) -> Format {
    match f {
        Format::DepthFloat => Format::Float,
        _ => f,
    }
}

#[repr(C)]
#[cfg_attr(debug_mode, derive(Debug))]
struct Uniform {
    inverse_samples_count: Real,
    samples_count: i32,
    window_height: i32,
    window_width: i32,
}

impl Uniform {
    pub fn new(samples_count: i32, window_width: i32, window_height: i32) -> Self {
        Uniform {
            inverse_samples_count: 1f32 / (samples_count as f32),
            samples_count,
            window_height,
            window_width,
        }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Resolver {
    buffers: Vec<Arc<ImageView>>,
    render_pass: Arc<RenderPass>,
    framebuffer: Arc<Framebuffer>,
    uniform: Uniform,
    uniform_buffer: DynamicBuffer,
    textures: Vec<Arc<RwLock<Texture>>>,
    output_textures: Vec<Arc<RwLock<Texture>>>,
    descriptor_set: Arc<DescriptorSet>,
    pipeline: Arc<Pipeline>,
}

impl Resolver {
    pub(super) fn new(
        eng: &GraphicApiEngine,
        g_buffer_filler: &GBufferFiller,
        texture_manager: &mut TextureManager,
        config: &Configurations,
    ) -> Self {
        let dev = eng.get_device();
        let memmgr = eng.get_memory_manager();
        let sampler = eng.get_linear_repeat_sampler();
        let g_buffers = g_buffer_filler.get_buffers();

        let mut buffers = Vec::with_capacity(g_buffers.len());
        let mut textures = Vec::with_capacity(g_buffers.len());
        let mut output_textures = Vec::with_capacity(g_buffers.len());
        for v in g_buffers {
            let img = vxresult!(v.get_image().read());
            let format = convert_format_to_resolver_format(img.get_format());
            let buffer = Arc::new(ImageView::new_surface_attachment(
                dev.clone(),
                memmgr,
                format,
                1,
                AttachmentType::ResolverBuffer,
            ));
            output_textures
                .push(texture_manager.create_2d_with_view_sampler(buffer.clone(), sampler.clone()));
            buffers.push(buffer);
            textures.push(texture_manager.create_2d_with_view_sampler(v.clone(), sampler.clone()));
        }

        let render_pass = Arc::new(RenderPass::new(buffers.clone(), true, true));
        let framebuffer = Arc::new(Framebuffer::new(buffers.clone(), render_pass.clone()));
        let (w, h) = vxresult!(buffers[0].get_image().read()).get_dimensions();
        let s = eng.get_samples_count();
        let uniform = Uniform::new(s as i32, w as i32, h as i32);
        let uniform_buffer = vxresult!(eng.get_buffer_manager().write())
            .create_dynamic_buffer(size_of::<Uniform>() as isize);
        let descriptor_set = vxresult!(eng.get_descriptor_manager().write())
            .create_resolver_set(&uniform_buffer, textures.clone());
        let mut pipmgr = vxresult!(eng.get_pipeline_manager().write());
        let pipeline = pipmgr.create(render_pass.clone(), PipelineType::Resolver, config);
        Self {
            buffers,
            render_pass,
            framebuffer,
            uniform,
            uniform_buffer,
            descriptor_set,
            textures,
            output_textures,
            pipeline,
        }
    }

    pub(super) fn begin_primary(&self, cmd: &mut CmdBuffer) {
        cmd.begin();
        self.framebuffer.begin(cmd);
    }

    pub(super) fn update(&mut self, frame_number: usize) {
        self.uniform_buffer.update(&self.uniform, frame_number);
    }

    pub(super) fn begin_secondary(&self, cmd: &mut CmdBuffer, frame_number: usize) {
        cmd.begin_secondary(&self.framebuffer);
        let buffer = self.uniform_buffer.get_buffer(frame_number);
        let buffer = vxresult!(buffer.read());
        cmd.bind_pipeline(&self.pipeline);
        cmd.bind_resolver_descriptor(&*self.descriptor_set, &*buffer);
        cmd.render_resolver();
        cmd.end();
    }

    pub(super) fn get_framebuffer(&self) -> &Arc<Framebuffer> {
        return &self.framebuffer;
    }

    pub(super) fn _get_buffers(&self) -> &Vec<Arc<ImageView>> {
        return &self.buffers;
    }

    pub(super) fn get_output_textures(&self) -> &Vec<Arc<RwLock<Texture>>> {
        return &self.output_textures;
    }
}
