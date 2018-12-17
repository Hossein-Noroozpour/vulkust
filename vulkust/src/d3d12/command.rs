use super::buffer::{Buffer as BufBuffer, Static as StaticBuffer};
use super::descriptor::Set as DescriptorSet;
use super::device::Device;
use super::framebuffer::Framebuffer;
use super::pipeline::Pipeline;
use std::mem::{transmute, zeroed};
use std::ptr::null_mut;
use std::sync::{Arc, RwLock};
use winapi;
use winapi::Interface;

pub struct Pool {
    device: Arc<Device>,
    pool: &'static mut winapi::um::d3d12::ID3D12CommandAllocator,
}

impl Pool {
    pub(super) fn new(device: Arc<Device>) -> Self {
        let mut pool: &'static mut winapi::um::d3d12::ID3D12CommandAllocator = unsafe { zeroed() };
        ThrowIfFailed!(device.get_data().CreateCommandAllocator(
            winapi::um::d3d12::D3D12_COMMAND_LIST_TYPE_DIRECT,
            &winapi::um::d3d12::ID3D12CommandAllocator::uuidof(),
            transmute(&mut pool)
        ));
        Self { device, pool }
    }

    pub(super) fn get_device(&self) -> &Arc<Device> {
        return &self.device;
    }

    pub(super) fn get_data(&self) -> &winapi::um::d3d12::ID3D12CommandAllocator {
        return &*self.pool;
    }
}

#[cfg(debug_mode)]
impl std::fmt::Debug for Pool {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "Directx12-Pool");
    }
}

unsafe impl Send for Pool {}
unsafe impl Sync for Pool {}

pub struct Buffer {
    pool: Arc<Pool>,
    command_list: &'static mut winapi::um::d3d12::ID3D12CommandList,
}

impl Buffer {
    pub(crate) fn new_primary(pool: Arc<Pool>) -> Self {
        let mut command_list: &'static mut winapi::um::d3d12::ID3D12CommandList =
            unsafe { zeroed() };
        ThrowIfFailed!(pool.get_device().get_data().CreateCommandList(
            0,
            winapi::um::d3d12::D3D12_COMMAND_LIST_TYPE_DIRECT,
            transmute(pool.get_data()),
            null_mut(),
            &winapi::um::d3d12::ID3D12CommandList::uuidof(),
            transmute(&mut command_list)
        ));
        Self { pool, command_list }
    }

    pub(crate) fn new_secondary(pool: Arc<Pool>) -> Self {
        let mut command_list: &'static mut winapi::um::d3d12::ID3D12CommandList =
            unsafe { zeroed() };
        ThrowIfFailed!(pool.get_device().get_data().CreateCommandList(
            0,
            winapi::um::d3d12::D3D12_COMMAND_LIST_TYPE_BUNDLE,
            transmute(pool.get_data()),
            null_mut(),
            &winapi::um::d3d12::ID3D12CommandList::uuidof(),
            transmute(&mut command_list)
        ));
        Self { pool, command_list }
    }

    pub(crate) fn get_has_render_record(&self) -> bool {
        vxunimplemented!();
    }

    pub(crate) fn exe_cmd(&mut self, _other: &Self) {
        vxunimplemented!();
    }

    pub(crate) fn exe_cmds(&mut self, _others: &[&Self]) {
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

    pub(crate) fn bind_pipeline(&mut self, _p: &Pipeline) {
        vxunimplemented!();
    }

    pub(crate) fn bind_vertex_buffer(&mut self, _buffer: &Arc<RwLock<BufBuffer>>) {
        vxunimplemented!();
    }

    pub(crate) fn bind_index_buffer(&mut self, _buffer: &Arc<RwLock<BufBuffer>>) {
        vxunimplemented!();
    }

    pub(crate) fn draw_index(&mut self, _indices_count: u32) {
        vxunimplemented!();
    }

    pub(crate) fn draw(&mut self, _vertices_count: u32) {
        vxunimplemented!();
    }

    pub(crate) fn bind_gbuff_scene_descriptor(
        &mut self,
        _descriptor_set: &DescriptorSet,
        _buffer: &BufBuffer,
    ) {
        vxunimplemented!();
    }

    pub(crate) fn bind_gbuff_model_descriptor(
        &mut self,
        _descriptor_set: &DescriptorSet,
        _buffer: &BufBuffer,
    ) {
        vxunimplemented!();
    }

    pub(crate) fn bind_gbuff_material_descriptor(
        &mut self,
        _descriptor_set: &DescriptorSet,
        _buffer: &BufBuffer,
    ) {
        vxunimplemented!();
    }

    pub(crate) fn render_gbuff(
        &mut self,
        _vertex_buffer: &StaticBuffer,
        _index_buffer: &StaticBuffer,
        _indices_count: u32,
    ) {
        vxunimplemented!();
    }

    pub(crate) fn render_deferred(&mut self) {
        vxunimplemented!();
    }

    pub(crate) fn render_ssao(&mut self) {
        vxunimplemented!();
    }

    pub(crate) fn render_shadow_accumulator_directional(&mut self) {
        vxunimplemented!();
    }

    pub(crate) fn bind_deferred_scene_descriptor(
        &mut self,
        _descriptor_set: &DescriptorSet,
        _buffer: &BufBuffer,
    ) {
        vxunimplemented!();
    }

    pub(crate) fn bind_deferred_deferred_descriptor(
        &mut self,
        _descriptor_set: &DescriptorSet,
        _buffer: &BufBuffer,
    ) {
        vxunimplemented!();
    }

    pub(crate) fn bind_ssao_scene_descriptor(
        &mut self,
        _descriptor_set: &DescriptorSet,
        _buffer: &BufBuffer,
    ) {
        vxunimplemented!();
    }

    pub(crate) fn bind_ssao_ssao_descriptor(
        &mut self,
        _descriptor_set: &DescriptorSet,
        _buffer: &BufBuffer,
    ) {
        vxunimplemented!();
    }

    pub(crate) fn bind_shadow_mapper_light_descriptor(
        &mut self,
        _descriptor_set: &DescriptorSet,
        _buffer: &BufBuffer,
    ) {
        vxunimplemented!();
    }

    pub(crate) fn bind_shadow_mapper_material_descriptor(
        &mut self,
        _descriptor_set: &DescriptorSet,
        _buffer: &BufBuffer,
    ) {
        vxunimplemented!();
    }

    pub(crate) fn bind_shadow_accumulator_directional_descriptor(
        &mut self,
        _descriptor_set: &DescriptorSet,
        _buffer: &BufBuffer,
    ) {
        vxunimplemented!();
    }

    pub(crate) fn render_shadow_mapper(
        &mut self,
        _vertex_buffer: &StaticBuffer,
        _index_buffer: &StaticBuffer,
        _indices_count: u32,
    ) {
        vxunimplemented!();
    }
}

#[cfg(debug_mode)]
impl std::fmt::Debug for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "Directx12-Buffer");
    }
}

unsafe impl Send for Buffer {}
unsafe impl Sync for Buffer {}
