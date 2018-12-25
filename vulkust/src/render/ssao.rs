use super::super::core::types::Real;
use super::buffer::Dynamic as DynamicBuffer;
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

use cgmath;
use cgmath::InnerSpace;
use rand;
use rand::distributions::{Distribution as RandDis, Uniform as RandUni};

const MAX_SSAO_SAMPLES_COUNT: usize = 128;

#[repr(C)]
struct Uniform {
    sample_vectors: [cgmath::Vector4<Real>; MAX_SSAO_SAMPLES_COUNT],
}

#[cfg(debug_mode)]
impl std::fmt::Debug for Uniform {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "SSAO Uniform");
    }
}

impl Uniform {
    pub fn new() -> Self {
        let r1 = RandUni::from(-1f32..1f32);
        let r2 = RandUni::from(0f32..1f32);
        let mut rng = rand::thread_rng();
        let mut sample_vectors = [cgmath::Vector4::new(0.0, 0.0, 0.0, 0.0); MAX_SSAO_SAMPLES_COUNT];
        let mut sum_weight = 0.0;
        for i in 0..MAX_SSAO_SAMPLES_COUNT {
            let v = cgmath::Vector3::new(
                r1.sample(&mut rng),
                r1.sample(&mut rng),
                r2.sample(&mut rng),
            );
            let sv = &mut sample_vectors[i];
            sv.x = v.x;
            sv.y = v.y;
            sv.z = v.z;
            sv.w = 2.4 - v.magnitude();
            sum_weight += sv.w;
        }
        let coef = -1.0 / sum_weight;
        for i in 0..MAX_SSAO_SAMPLES_COUNT {
            sample_vectors[i].w *= coef;
        }
        Uniform { sample_vectors }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct SSAO {
    uniform: Uniform,
    uniform_buffer: DynamicBuffer,
    render_pass: Arc<RenderPass>,
    framebuffer: Arc<Framebuffer>,
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
        let dev = eng.get_device();
        let memmgr = eng.get_memory_manager();
        let buffers = vec![Arc::new(ImageView::new_surface_attachment(
            dev.clone(),
            memmgr,
            Format::Float,
            AttachmentType::ColorGBuffer,
        ))];
        let uniform = Uniform::new();
        let uniform_buffer = vxresult!(eng.get_buffer_manager().write())
            .create_dynamic_buffer(size_of::<Uniform>() as isize);
        let mut textures = Vec::with_capacity(3); // position, normal, depth
        textures.push(g_buffer_filler.get_position_texture().clone());
        textures.push(g_buffer_filler.get_normal_texture().clone());
        textures.push(g_buffer_filler.get_depth_texture().clone());
        let descriptor_set = vxresult!(eng.get_descriptor_manager().write())
            .create_ssao_set(&uniform_buffer, textures);
        let sampler = eng.get_linear_repeat_sampler();
        let mut textures = Vec::with_capacity(buffers.len());
        for b in &buffers {
            textures.push(texmgr.create_2d_with_view_sampler(b.clone(), sampler.clone()));
        }
        let render_pass = Arc::new(RenderPass::new(buffers.clone(), true, true));
        let framebuffer = Arc::new(Framebuffer::new(buffers, render_pass.clone()));
        let pipeline = vxresult!(eng.get_pipeline_manager().write()).create(
            render_pass.clone(),
            PipelineType::SSAO,
            config,
        );
        Self {
            uniform,
            uniform_buffer,
            descriptor_set,
            pipeline,
            render_pass,
            framebuffer,
            textures,
        }
    }

    pub(super) fn begin_secondary(&self, cmd: &mut CmdBuffer) {
        cmd.begin_secondary(&self.framebuffer);
        cmd.bind_pipeline(&self.pipeline);
    }

    pub(super) fn end_secondary(&self, cmd: &mut CmdBuffer, frame_number: usize) {
        let buffer = self.uniform_buffer.get_buffer(frame_number);
        let buffer = vxresult!(buffer.read());
        cmd.bind_ssao_ssao_descriptor(&*self.descriptor_set, &*buffer);
        cmd.render_ssao();
        cmd.end();
    }

    pub(super) fn record_primary(&self, pricmd: &mut CmdBuffer, seccmd: &CmdBuffer) {
        pricmd.begin();
        self.framebuffer.begin(pricmd);
        pricmd.exe_cmd(seccmd);
        pricmd.end_render_pass();
        pricmd.end();
    }

    pub(crate) fn update(&mut self, frame_number: usize) {
        self.uniform_buffer.update(&self.uniform, frame_number);
    }

    pub(crate) fn get_ambient_occlusion_texture(&self) -> &Arc<RwLock<Texture>> {
        return &self.textures[0];
    }
}
