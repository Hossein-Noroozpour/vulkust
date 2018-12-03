use std::mem::{transmute, transmute_copy, zeroed};
use std::ptr::null_mut;
use winapi;
use winapi::Interface;

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct Device {}

impl Device {
    pub(super) fn new() -> Self {
        let mut dxgi_factory_flags: winapi::shared::minwindef::UINT = 0;
        #[cfg(debug_mode)]
        {
            let mut debug_controller: &'static mut winapi::um::d3d12sdklayers::ID3D12Debug =
                unsafe { transmute(0usize) };
            if winapi::shared::winerror::SUCCEEDED(unsafe {
                winapi::um::d3d12::D3D12GetDebugInterface(
                    &winapi::um::d3d12sdklayers::ID3D12Debug::uuidof(),
                    transmute(&mut debug_controller),
                )
            }) {
                unsafe {
                    debug_controller.EnableDebugLayer();
                }
                dxgi_factory_flags |= winapi::shared::dxgi1_3::DXGI_CREATE_FACTORY_DEBUG;
            }
        }
        let mut factory: &'static mut winapi::shared::dxgi1_4::IDXGIFactory4 =
            unsafe { transmute(0usize) };
        ThrowIfFailed!(winapi::shared::dxgi1_3::CreateDXGIFactory2(
            dxgi_factory_flags,
            &winapi::shared::dxgi1_4::IDXGIFactory4::uuidof(),
            transmute(&mut factory)
        ));
        let mut adapter: &'static mut winapi::shared::dxgi::IDXGIAdapter1 =
            unsafe { transmute(0usize) };
        let mut adapter_index: winapi::shared::minwindef::UINT = 0;
        let mut adapter_found = false;
        while winapi::shared::winerror::DXGI_ERROR_NOT_FOUND
            != unsafe { factory.EnumAdapters1(adapter_index, transmute_copy(&mut adapter)) }
        {
            let mut desc: winapi::shared::dxgi::DXGI_ADAPTER_DESC1 = unsafe { zeroed() };
            unsafe {
                adapter.GetDesc1(&mut desc);
            }
            if desc.Flags & winapi::shared::dxgi::DXGI_ADAPTER_FLAG_SOFTWARE
                == winapi::shared::dxgi::DXGI_ADAPTER_FLAG_SOFTWARE
            {
                continue;
            }
            if winapi::shared::winerror::SUCCEEDED(unsafe {
                winapi::um::d3d12::D3D12CreateDevice(
                    transmute_copy(adapter),
                    winapi::um::d3dcommon::D3D_FEATURE_LEVEL_11_0,
                    &winapi::um::d3d12::ID3D12Device::uuidof(),
                    null_mut(),
                )
            }) {
                adapter_found = true;
                break;
            }
            adapter_index += 1;
        }
        if !adapter_found {
            // todo
        }
        let mut device: &'static mut winapi::um::d3d12::ID3D12Device = unsafe { transmute(0usize) };
        ThrowIfFailed!(winapi::um::d3d12::D3D12CreateDevice(
            transmute(adapter),
            winapi::um::d3dcommon::D3D_FEATURE_LEVEL_11_0,
            &winapi::um::d3d12::ID3D12Device::uuidof(),
            transmute(&mut device)
        ));

        // if (m_useWarpDevice)
        // {
        //     ComPtr<IDXGIAdapter> warpAdapter;
        //     ThrowIfFailed(factory->EnumWarpAdapter(IID_PPV_ARGS(&warpAdapter)));

        //     ThrowIfFailed(D3D12CreateDevice(
        //         warpAdapter.Get(),
        //         D3D_FEATURE_LEVEL_11_0,
        //         IID_PPV_ARGS(&m_device)
        //         ));
        // }
        // else
        // {
        //
        Self {}
    }
}
