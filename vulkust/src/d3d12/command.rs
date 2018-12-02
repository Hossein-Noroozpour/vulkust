use super::buffer::{Buffer as BufBuffer, Static as StaticBuffer};
use super::descriptor::Set as DescriptorSet;
use super::framebuffer::Framebuffer;
use super::pipeline::Pipeline;
use std::sync::{Arc, RwLock};

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct Pool {}

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct Buffer {}

impl Buffer {
    pub(crate) fn new_primary(_pool: Arc<Pool>) -> Self {
        vxunimplemented!();
    }

    pub(crate) fn new_secondary(_pool: Arc<Pool>) -> Self {
        vxunimplemented!();
    }

    fn new(_pool: Arc<Pool>, _is_secondary: bool) -> Self {
        vxunimplemented!();
    }

    pub(crate) fn get_has_render_record(&self) -> bool {
        vxunimplemented!();
    }

    pub(crate) fn exe_cmd(&mut self, _other: &Self) {
        vxunimplemented!();
    }

    pub(crate) fn begin(&mut self) {
        vxunimplemented!();
    }

    pub(crate) fn begin_secondary(&mut self, _framebuffer: &Framebuffer) {
        vxunimplemented!();
    }

    pub(crate) fn end_render_pass(&mut self) {
        vxunimplemented!();
    }

    pub(crate) fn end(&mut self) {
        vxunimplemented!();
    }

    pub(crate) fn bind_pipeline(&mut self, p: &Pipeline) {
        vxunimplemented!();
    }

    pub(crate) fn bind_vertex_buffer(&mut self, buffer: &Arc<RwLock<BufBuffer>>) {
        vxunimplemented!();
    }

    pub(crate) fn bind_index_buffer(&mut self, buffer: &Arc<RwLock<BufBuffer>>) {
        vxunimplemented!();
    }

    pub(crate) fn draw_index(&mut self, indices_count: u32) {
        vxunimplemented!();
    }

    pub(crate) fn draw(&mut self, vertices_count: u32) {
        vxunimplemented!();
    }

    pub(crate) fn bind_gbuff_scene_descriptor(
        &mut self,
        descriptor_set: &DescriptorSet,
        buffer: &BufBuffer,
    ) {
        vxunimplemented!();
    }

    pub(crate) fn bind_gbuff_model_descriptor(
        &mut self,
        descriptor_set: &DescriptorSet,
        buffer: &BufBuffer,
    ) {
        vxunimplemented!();
    }

    pub(crate) fn bind_gbuff_material_descriptor(
        &mut self,
        descriptor_set: &DescriptorSet,
        buffer: &BufBuffer,
    ) {
        vxunimplemented!();
    }

    pub(crate) fn render_gbuff(
        &mut self,
        vertex_buffer: &StaticBuffer,
        index_buffer: &StaticBuffer,
        indices_count: u32,
    ) {
        vxunimplemented!();
    }

    pub(crate) fn render_deferred(&mut self) {
        vxunimplemented!();
    }

    pub(crate) fn render_shadow_accumulator_directional(&mut self) {
        vxunimplemented!();
    }

    pub(crate) fn bind_deferred_scene_descriptor(
        &mut self,
        descriptor_set: &DescriptorSet,
        buffer: &BufBuffer,
    ) {
        vxunimplemented!();
    }

    pub(crate) fn bind_deferred_deferred_descriptor(
        &mut self,
        descriptor_set: &DescriptorSet,
        buffer: &BufBuffer,
    ) {
        vxunimplemented!();
    }

    pub(crate) fn bind_shadow_mapper_light_descriptor(
        &mut self,
        descriptor_set: &DescriptorSet,
        buffer: &BufBuffer,
    ) {
        vxunimplemented!();
    }

    pub(crate) fn bind_shadow_mapper_material_descriptor(
        &mut self,
        descriptor_set: &DescriptorSet,
        buffer: &BufBuffer,
    ) {
        vxunimplemented!();
    }

    pub(crate) fn bind_shadow_accumulator_directional_descriptor(
        &mut self,
        descriptor_set: &DescriptorSet,
        buffer: &BufBuffer,
    ) {
        vxunimplemented!();
    }

    pub(crate) fn render_shadow_mapper(
        &mut self,
        vertex_buffer: &StaticBuffer,
        index_buffer: &StaticBuffer,
        indices_count: u32,
    ) {
        vxunimplemented!();
    }
}
