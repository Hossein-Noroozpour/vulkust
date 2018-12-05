use super::super::render::config::Configurations;
use super::super::system::os::application::Application as OsApp;
use super::device::Device;
use std::mem::{transmute, zeroed};
use std::ptr::null_mut;
use std::sync::{Arc, RwLock};
use winapi;
use winapi::Interface;

pub(super) const BUFFER_COUNT: usize = 3;

pub(super) struct Swapchain {
    swapchain: &'static mut winapi::shared::dxgi1_4::IDXGISwapChain3,
    render_targets: [&'static mut winapi::um::d3d12::ID3D12Resource; BUFFER_COUNT],
    device: Arc<Device>,
}

impl Swapchain {
    pub(super) fn new(
        device: Arc<Device>,
        os_app: &Arc<RwLock<OsApp>>,
        conf: &Configurations,
    ) -> Self {
        let hwnd = vxresult!(os_app.read()).get_window();
        let mut swapchain_desc: winapi::shared::dxgi1_2::DXGI_SWAP_CHAIN_DESC1 =
            unsafe { zeroed() };
        swapchain_desc.BufferCount = BUFFER_COUNT as winapi::shared::minwindef::UINT;
        swapchain_desc.Width = conf.get_content_width();
        swapchain_desc.Height = conf.get_content_height();
        swapchain_desc.Format = winapi::shared::dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM;
        swapchain_desc.BufferUsage = winapi::shared::dxgitype::DXGI_USAGE_RENDER_TARGET_OUTPUT;
        swapchain_desc.SwapEffect = winapi::shared::dxgi::DXGI_SWAP_EFFECT_FLIP_DISCARD;
        swapchain_desc.SampleDesc.Count = 1;
        let mut swapchain: &'static mut winapi::shared::dxgi1_4::IDXGISwapChain3 =
            unsafe { zeroed() };
        ThrowIfFailed!(device.get_factory().CreateSwapChainForHwnd(
            transmute(&*device.get_queue()),
            hwnd,
            &swapchain_desc,
            null_mut(),
            null_mut(),
            transmute(&mut swapchain)
        ));
        // todo replace this with in future whenever winapi got updated DXGI_MWA_NO_ALT_ENTER
        ThrowIfFailed!(device.get_factory().MakeWindowAssociation(hwnd, 2));
        let mut rtv_heap_desc: winapi::um::d3d12::D3D12_DESCRIPTOR_HEAP_DESC = unsafe { zeroed() };
        rtv_heap_desc.NumDescriptors = BUFFER_COUNT as winapi::shared::minwindef::UINT;
        rtv_heap_desc.Type = winapi::um::d3d12::D3D12_DESCRIPTOR_HEAP_TYPE_RTV;
        rtv_heap_desc.Flags = winapi::um::d3d12::D3D12_DESCRIPTOR_HEAP_FLAG_NONE;
        let mut rtv_heap: &'static mut winapi::um::d3d12::ID3D12DescriptorHeap =
            unsafe { zeroed() };
        ThrowIfFailed!(device.get_data().CreateDescriptorHeap(
            &rtv_heap_desc,
            &winapi::um::d3d12::ID3D12DescriptorHeap::uuidof(),
            transmute(&mut rtv_heap)
        ));
        let rtv_descriptor_size = unsafe {
            device
                .get_data()
                .GetDescriptorHandleIncrementSize(winapi::um::d3d12::D3D12_DESCRIPTOR_HEAP_TYPE_RTV)
        };
        let mut rtv_handle = unsafe { rtv_heap.GetCPUDescriptorHandleForHeapStart() };
        let mut render_targets: [&'static mut winapi::um::d3d12::ID3D12Resource; BUFFER_COUNT] =
            unsafe { zeroed() };
        for n in 0..BUFFER_COUNT {
            ThrowIfFailed!(swapchain.GetBuffer(
                n as winapi::shared::minwindef::UINT,
                &winapi::um::d3d12::ID3D12Resource::uuidof(),
                transmute(&mut render_targets[n])
            ));
            unsafe {
                device
                    .get_data()
                    .CreateRenderTargetView(render_targets[n], null_mut(), rtv_handle);
            }
            rtv_handle.ptr += rtv_descriptor_size as winapi::shared::basetsd::SIZE_T;
        }
        Self {
            swapchain,
            render_targets,
            device,
        }
    }

    pub(crate) fn get_current_frame_index(&self) -> u32 {
        return unsafe { self.swapchain.GetCurrentBackBufferIndex() };
    }
}

#[cfg(debug_mode)]
impl std::fmt::Debug for Swapchain {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "Directx12-Swapchain");
    }
}

unsafe impl Send for Swapchain {}
unsafe impl Sync for Swapchain {}
