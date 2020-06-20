use super::buffer::{Buffer as BufBuffer, Static as StaticBuffer};
use super::descriptor::Set as DescriptorSet;
use super::framebuffer::Framebuffer;
use super::pipeline::Pipeline;
use std::sync::{Arc, RwLock};

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Pool {}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Buffer {}

impl Buffer {
    pub(crate) fn new_primary(_pool: Arc<Pool>) -> Self {
        vx_unimplemented!();
    }

    pub(crate) fn new_secondary(_pool: Arc<Pool>) -> Self {
        vx_unimplemented!();
    }

    fn new(_pool: Arc<Pool>, _is_secondary: bool) -> Self {
        vx_unimplemented!();
    }

    pub(crate) fn get_has_render_record(&self) -> bool {
        vx_unimplemented!();
    }

    pub(crate) fn exe_cmd(&mut self, _other: &Self) {
        vx_unimplemented!();
    }

    pub(crate) fn exe_cmds(&mut self, _others: &[&Self]) {
        vx_unimplemented!();
    }

    pub(crate) fn begin(&mut self) {
        vx_unimplemented!();
    }

    pub(crate) fn begin_secondary(&mut self, _framebuffer: &Framebuffer) {
        vx_unimplemented!();
    }

    pub(crate) fn end_render_pass(&mut self) {
        vx_unimplemented!();
    }

    pub(crate) fn end(&mut self) {
        vx_unimplemented!();
    }

    pub(crate) fn bind_pipeline(&mut self, _p: &Pipeline) {
        vx_unimplemented!();
    }

    pub(crate) fn bind_vertex_buffer(&mut self, _buffer: &Arc<RwLock<BufBuffer>>) {
        vx_unimplemented!();
    }

    pub(crate) fn bind_index_buffer(&mut self, _buffer: &Arc<RwLock<BufBuffer>>) {
        vx_unimplemented!();
    }

    pub(crate) fn draw_index(&mut self, _indices_count: u32) {
        vx_unimplemented!();
    }

    pub(crate) fn draw(&mut self, _vertices_count: u32) {
        vx_unimplemented!();
    }

    pub(crate) fn bind_gbuff_scene_descriptor(
        &mut self,
        _descriptor_set: &DescriptorSet,
        _buffer: &BufBuffer,
    ) {
        vx_unimplemented!();
    }

    pub(crate) fn bind_gbuff_model_descriptor(
        &mut self,
        _descriptor_set: &DescriptorSet,
        _buffer: &BufBuffer,
    ) {
        vx_unimplemented!();
    }

    pub(crate) fn bind_gbuff_material_descriptor(
        &mut self,
        _descriptor_set: &DescriptorSet,
        _buffer: &BufBuffer,
    ) {
        vx_unimplemented!();
    }

    pub(crate) fn render_gbuff(
        &mut self,
        _vertex_buffer: &StaticBuffer,
        _index_buffer: &StaticBuffer,
        _indices_count: u32,
    ) {
        vx_unimplemented!();
    }

    pub(crate) fn render_deferred(&mut self) {
        vx_unimplemented!();
    }

    pub(crate) fn render_ssao(&mut self) {
        vx_unimplemented!();
    }

    pub(crate) fn render_shadow_accumulator_directional(&mut self) {
        vx_unimplemented!();
    }

    pub(crate) fn bind_deferred_scene_descriptor(
        &mut self,
        _descriptor_set: &DescriptorSet,
        _buffer: &BufBuffer,
    ) {
        vx_unimplemented!();
    }

    pub(crate) fn bind_deferred_deferred_descriptor(
        &mut self,
        _descriptor_set: &DescriptorSet,
        _buffer: &BufBuffer,
    ) {
        vx_unimplemented!();
    }

    pub(crate) fn bind_ssao_scene_descriptor(
        &mut self,
        _descriptor_set: &DescriptorSet,
        _buffer: &BufBuffer,
    ) {
        vx_unimplemented!();
    }

    pub(crate) fn bind_ssao_ssao_descriptor(
        &mut self,
        _descriptor_set: &DescriptorSet,
        _buffer: &BufBuffer,
    ) {
        vx_unimplemented!();
    }

    pub(crate) fn bind_shadow_mapper_light_descriptor(
        &mut self,
        _descriptor_set: &DescriptorSet,
        _buffer: &BufBuffer,
    ) {
        vx_unimplemented!();
    }

    pub(crate) fn bind_shadow_mapper_material_descriptor(
        &mut self,
        _descriptor_set: &DescriptorSet,
        _buffer: &BufBuffer,
    ) {
        vx_unimplemented!();
    }

    pub(crate) fn bind_shadow_accumulator_directional_descriptor(
        &mut self,
        _descriptor_set: &DescriptorSet,
        _buffer: &BufBuffer,
    ) {
        vx_unimplemented!();
    }

    pub(crate) fn render_shadow_mapper(
        &mut self,
        _vertex_buffer: &StaticBuffer,
        _index_buffer: &StaticBuffer,
        _indices_count: u32,
    ) {
        vx_unimplemented!();
    }
}
