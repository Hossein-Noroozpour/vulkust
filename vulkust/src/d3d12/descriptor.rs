use super::super::render::config::Configurations;
use super::super::render::texture::Texture;
use super::buffer::Dynamic as DynamicBuffer;
use super::device::Device;
use std::mem::{transmute, zeroed};
use std::sync::{Arc, RwLock};

use winapi;
use winapi::Interface;

pub(crate) struct Pool {
    device: Arc<Device>,
    dsv_heap: &'static mut winapi::um::d3d12::ID3D12DescriptorHeap,
    cbv_srv_uav_heap: &'static mut winapi::um::d3d12::ID3D12DescriptorHeap,
    sampler_heap: &'static mut winapi::um::d3d12::ID3D12DescriptorHeap,
}

impl Pool {
    fn new(device: Arc<Device>, conf: &Configurations) -> Self {
        let mut dsv_heap_desc: winapi::um::d3d12::D3D12_DESCRIPTOR_HEAP_DESC = unsafe { zeroed() };
        dsv_heap_desc.NumDescriptors =
            (1 /* gbuffer depth */ + conf.get_cascaded_shadows_count()) as _;
        dsv_heap_desc.Type = winapi::um::d3d12::D3D12_DESCRIPTOR_HEAP_TYPE_DSV;
        dsv_heap_desc.Flags = winapi::um::d3d12::D3D12_DESCRIPTOR_HEAP_FLAG_NONE;
        let mut dsv_heap: &'static mut winapi::um::d3d12::ID3D12DescriptorHeap =
            unsafe { zeroed() };
        ThrowIfFailed!(device.get_data().CreateDescriptorHeap(
            &dsv_heap_desc,
            &winapi::um::d3d12::ID3D12DescriptorHeap::uuidof(),
            transmute(&mut dsv_heap)
        ));

        let mut cbv_srv_uav_heap_desc: winapi::um::d3d12::D3D12_DESCRIPTOR_HEAP_DESC =
            unsafe { zeroed() };
        cbv_srv_uav_heap_desc.NumDescriptors = (conf.get_max_shadow_maker_lights_count() * 6
            + conf.get_max_textures_count()
            + conf.get_max_models_count()
            + conf.get_max_meshes_count()
            + conf.get_max_scenes_count()) as _;
        cbv_srv_uav_heap_desc.Type = winapi::um::d3d12::D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV;
        cbv_srv_uav_heap_desc.Flags = winapi::um::d3d12::D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE;
        let mut cbv_srv_uav_heap: &'static mut winapi::um::d3d12::ID3D12DescriptorHeap =
            unsafe { zeroed() };
        ThrowIfFailed!(device.get_data().CreateDescriptorHeap(
            &cbv_srv_uav_heap_desc,
            &winapi::um::d3d12::ID3D12DescriptorHeap::uuidof(),
            transmute(&mut cbv_srv_uav_heap)
        ));

        let mut sampler_heap_desc: winapi::um::d3d12::D3D12_DESCRIPTOR_HEAP_DESC =
            unsafe { zeroed() };
        sampler_heap_desc.NumDescriptors = 2; // One clamp and one wrap sampler.
        sampler_heap_desc.Type = winapi::um::d3d12::D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER;
        sampler_heap_desc.Flags = winapi::um::d3d12::D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE;
        let mut sampler_heap: &'static mut winapi::um::d3d12::ID3D12DescriptorHeap =
            unsafe { zeroed() };
        ThrowIfFailed!(device.get_data().CreateDescriptorHeap(
            &sampler_heap_desc,
            &winapi::um::d3d12::ID3D12DescriptorHeap::uuidof(),
            transmute(&mut sampler_heap)
        ));

        Self {
            device,
            dsv_heap,
            cbv_srv_uav_heap,
            sampler_heap,
        }
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct Set {}

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct Manager {}

impl Manager {
    pub(crate) fn create_gbuff_set(
        &mut self,
        _uniform: &DynamicBuffer,
        _textures: Vec<Arc<RwLock<Texture>>>,
    ) -> Arc<Set> {
        vx_unimplemented!();
    }

    pub(crate) fn create_buffer_only_set(&mut self, _uniform: &DynamicBuffer) -> Arc<Set> {
        vx_unimplemented!();
    }

    pub(crate) fn create_deferred_set(
        &mut self,
        _uniform: &DynamicBuffer,
        _textures: Vec<Arc<RwLock<Texture>>>,
    ) -> Arc<Set> {
        vx_unimplemented!();
    }

    pub(crate) fn create_ssao_set(
        &mut self,
        _uniform: &DynamicBuffer,
        _textures: Vec<Arc<RwLock<Texture>>>,
    ) -> Arc<Set> {
        vx_unimplemented!();
    }

    pub(crate) fn create_shadow_accumulator_directional_set(
        &mut self,
        _uniform: &DynamicBuffer,
        _texturess: Vec<Vec<Arc<RwLock<Texture>>>>,
    ) -> Arc<Set> {
        vx_unimplemented!();
    }
}
