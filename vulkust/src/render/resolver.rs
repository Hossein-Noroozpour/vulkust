use super::super::core::types::Real;
use super::buffer::DynamicBuffer;
use super::command::Buffer as CmdBuffer;
use super::descriptor::Set as DescriptorSet;
use super::framebuffer::Framebuffer;
use super::g_buffer_filler::GBufferFiller;
use super::gapi::GraphicApiEngine;
use super::image::{AttachmentType, Format, View as ImageView};
use super::pipeline::{Pipeline, PipelineType};
use super::render_pass::RenderPass;
use super::scene::Manager as SceneManager;
use super::texture::Texture;
use std::mem::size_of;
use std::sync::{Arc, RwLock};

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
pub(super) struct Resolver {
    buffers: Vec<Arc<ImageView>>,
    render_pass: Arc<RenderPass>,
    framebuffer: Arc<Framebuffer>,
    uniform: Uniform,
    uniform_buffer: DynamicBuffer,
    textures: Vec<Arc<RwLock<Texture>>>,
    descriptor_set: Arc<DescriptorSet>,
    pipeline: Arc<Pipeline>,
}

fn convert_format(f: Format) -> Format {
    match f {
        Format::DepthFloat => return Format::Float,
        c @ _ => return c,
    }
}

impl Resolver {
    pub(super) fn new(
        eng: &GraphicApiEngine,
        g_buffer_filler: &GBufferFiller,
        scene_manager: &SceneManager,
    ) -> Self {
        let dev = eng.get_device();
        let memmgr = eng.get_memory_manager();
        let sampler = eng.get_linear_repeat_sampler();
        let mut texture_manager = vxresult!(scene_manager.texture_manager.write());
        let g_buffers = g_buffer_filler.get_buffers();

        let mut buffers = Vec::new();
        let mut textures = Vec::new();
        for v in g_buffers {
            let img = vxresult!(v.get_image().read());
            let format = convert_format(img.get_format());
            buffers.push(Arc::new(ImageView::new_surface_attachment(
                dev.clone(),
                memmgr,
                format,
                1,
                AttachmentType::ResolverBuffer,
            )));
            textures.push(texture_manager.create_2d_with_view_sampler(v.clone(), sampler.clone()));
        }
        textures.shrink_to_fit();
        buffers.shrink_to_fit();

        let render_pass = Arc::new(RenderPass::new(buffers.clone(), true, true));
        let framebuffer = Arc::new(Framebuffer::new(buffers.clone(), render_pass.clone()));
        let (w, h) = vxresult!(buffers[0].get_image().read()).get_dimensions();
        let s = eng.get_samples_count();
        let uniform = Uniform::new(s as i32, w as i32, h as i32);
        let uniform_buffer = 
            vxresult!(eng.get_buffer_manager().write())
                .create_dynamic_buffer(size_of::<Uniform>() as isize);
        let descriptor_set = vxresult!(eng.get_descriptor_manager().write())
            .create_resolver_set(&uniform_buffer, textures.clone());
        let descriptor_set = Arc::new(descriptor_set);
        let mut pipmgr = vxresult!(eng.get_pipeline_manager().write());
        let pipeline = pipmgr.create(render_pass.clone(), PipelineType::Resolver);
        Self {
            buffers,
            render_pass,
            framebuffer,
            uniform,
            uniform_buffer,
            descriptor_set,
            textures,
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

    pub(super) fn get_buffers(&self) -> &Vec<Arc<ImageView>> {
        return &self.buffers;
    }
}
