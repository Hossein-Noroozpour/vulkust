use std::mem::{transmute, zeroed};
use std::ptr::null_mut;
use winapi;
use winapi::Interface;

pub(crate) struct Device {
    adapter: &'static mut winapi::shared::dxgi::IDXGIAdapter1,
    device: &'static mut winapi::um::d3d12::ID3D12Device,
    factory: &'static mut winapi::shared::dxgi1_4::IDXGIFactory4,
    queue: &'static mut winapi::um::d3d12::ID3D12CommandQueue,
}

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
            != unsafe { factory.EnumAdapters1(adapter_index, transmute(&mut adapter)) }
        {
            let mut desc: winapi::shared::dxgi::DXGI_ADAPTER_DESC1 = unsafe { zeroed() };
            unsafe {
                adapter.GetDesc1(&mut desc);
            }
            if desc.Flags & winapi::shared::dxgi::DXGI_ADAPTER_FLAG_SOFTWARE
                == winapi::shared::dxgi::DXGI_ADAPTER_FLAG_SOFTWARE
            {
                adapter_index += 1;
                continue;
            }
            if winapi::shared::winerror::SUCCEEDED(unsafe {
                winapi::um::d3d12::D3D12CreateDevice(
                    transmute(&mut *adapter),
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
            ThrowIfFailed!(factory.EnumWarpAdapter(
                &winapi::shared::dxgi::IDXGIAdapter1::uuidof(),
                transmute(&mut adapter)
            ));
            #[cfg(debug_mode)]
            vx_log_i!("Warning: Warp device created instead of Hardware device this is going to impact the performance seriously");
        }
        let mut device: &'static mut winapi::um::d3d12::ID3D12Device = unsafe { transmute(0usize) };
        ThrowIfFailed!(winapi::um::d3d12::D3D12CreateDevice(
            transmute(&mut *adapter),
            winapi::um::d3dcommon::D3D_FEATURE_LEVEL_11_0,
            &winapi::um::d3d12::ID3D12Device::uuidof(),
            transmute(&mut device)
        ));
        let mut queue_desc: winapi::um::d3d12::D3D12_COMMAND_QUEUE_DESC = unsafe { zeroed() };
        queue_desc.Flags = winapi::um::d3d12::D3D12_COMMAND_QUEUE_FLAG_NONE;
        queue_desc.Type = winapi::um::d3d12::D3D12_COMMAND_LIST_TYPE_DIRECT;
        let mut queue: &'static mut winapi::um::d3d12::ID3D12CommandQueue =
            unsafe { transmute(0usize) };
        ThrowIfFailed!(device.CreateCommandQueue(
            &queue_desc,
            &winapi::um::d3d12::ID3D12CommandQueue::uuidof(),
            transmute(&mut queue)
        ));
        Self {
            adapter,
            device,
            factory,
            queue,
        }
    }

    pub(super) fn get_data(&self) -> &winapi::um::d3d12::ID3D12Device {
        return &*self.device;
    }

    pub(super) fn get_factory(&self) -> &winapi::shared::dxgi1_4::IDXGIFactory4 {
        return &*self.factory;
    }

    pub(super) fn get_queue(&self) -> &winapi::um::d3d12::ID3D12CommandQueue {
        return &*self.queue;
    }
}

#[cfg(debug_mode)]
impl std::fmt::Debug for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "Directx12-Device");
    }
}

unsafe impl Send for Device {}
unsafe impl Sync for Device {}
