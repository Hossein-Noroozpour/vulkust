use super::super::render::config::Configurations;
use super::super::render::pipeline::PipelineType;
use super::device::Device;
use super::render_pass::RenderPass;
use std::collections::BTreeMap;
use std::mem::{transmute, zeroed};
use std::ptr::{null, null_mut};
use std::sync::{Arc, Weak};
use winapi;
use winapi::Interface;

macro_rules! include_shader {
    ($name:expr, $ext:expr) => {
        include_bytes!(concat!(
            env!("OUT_DIR"),
            "/directx12/shaders/",
            $name,
            ".hlsl.",
            $ext,
            ".fxc"
        ))
    };
}

pub(crate) struct Layout {
    device: Arc<Device>,
    root_signature: &'static mut winapi::um::d3d12::ID3D12RootSignature,
}

impl Layout {
    pub(super) fn new(device: Arc<Device>) -> Self {
        let root_signature_desc = winapi::um::d3d12::D3D12_ROOT_SIGNATURE_DESC {
            NumParameters: 0,
            pParameters: null(),
            NumStaticSamplers: 0,
            pStaticSamplers: null(),
            Flags: winapi::um::d3d12::D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT,
        };
        let mut signature: &'static mut winapi::um::d3dcommon::ID3DBlob = unsafe { zeroed() };
        let mut error: &'static mut winapi::um::d3dcommon::ID3DBlob = unsafe { zeroed() };
        ThrowIfFailed!(winapi::um::d3d12::D3D12SerializeRootSignature(
            &root_signature_desc,
            winapi::um::d3d12::D3D_ROOT_SIGNATURE_VERSION_1,
            transmute(&mut signature),
            transmute(&mut error)
        ));
        let mut root_signature: &'static mut winapi::um::d3d12::ID3D12RootSignature =
            unsafe { zeroed() };
        ThrowIfFailed!(device.get_data().CreateRootSignature(
            0,
            signature.GetBufferPointer(),
            signature.GetBufferSize(),
            &winapi::um::d3d12::ID3D12RootSignature::uuidof(),
            transmute(&mut root_signature)
        ));
        Self {
            device,
            root_signature,
        }
    }
}

pub(crate) struct Pipeline {
    device: Arc<Device>,
    pipeline_state: &'static mut winapi::um::d3d12::ID3D12PipelineState,
}

impl Pipeline {
    fn new(device: Arc<Device>, pipeline_type: PipelineType) -> Self {
        let layout = Layout::new(device.clone());
        let vert_bytes: &'static [u8] = match pipeline_type {
            PipelineType::GBuffer => include_shader!("g-buffers-filler", "vert"),
            // PipelineType::Deferred => include_shader!("deferred.vert"),
            // PipelineType::ShadowMapper => include_shader!("shadow-mapper.vert"),
            // PipelineType::ShadowAccumulatorDirectional => {
            //     include_shader!("shadow-accumulator-directional.vert")
            // }
            _ => vxunimplemented!(),
        };
        let frag_bytes: &'static [u8] = match pipeline_type {
            PipelineType::GBuffer => include_shader!("g-buffers-filler", "frag"),
            // PipelineType::Deferred => include_shader!("deferred.frag"),
            // PipelineType::ShadowMapper => include_shader!("shadow-mapper.frag"),
            // PipelineType::ShadowAccumulatorDirectional => {
            //     include_shader!("shadow-accumulator-directional.frag")
            // }
            _ => vxunimplemented!(),
        };
        let input_element_descs = [
            winapi::um::d3d12::D3D12_INPUT_ELEMENT_DESC {
                SemanticName: unsafe { transmute("POSITION".as_ptr()) },
                SemanticIndex: 0,
                Format: winapi::shared::dxgiformat::DXGI_FORMAT_R32G32B32_FLOAT,
                InputSlot: 0,
                AlignedByteOffset: 0,
                InputSlotClass: winapi::um::d3d12::D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
                InstanceDataStepRate: 0,
            },
            winapi::um::d3d12::D3D12_INPUT_ELEMENT_DESC {
                SemanticName: unsafe { transmute("NORMAL".as_ptr()) },
                SemanticIndex: 0,
                Format: winapi::shared::dxgiformat::DXGI_FORMAT_R32G32B32_FLOAT,
                InputSlot: 0,
                AlignedByteOffset: 12,
                InputSlotClass: winapi::um::d3d12::D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
                InstanceDataStepRate: 0,
            },
            winapi::um::d3d12::D3D12_INPUT_ELEMENT_DESC {
                SemanticName: unsafe { transmute("TANGENT".as_ptr()) },
                SemanticIndex: 0,
                Format: winapi::shared::dxgiformat::DXGI_FORMAT_R32G32B32A32_FLOAT,
                InputSlot: 0,
                AlignedByteOffset: 24,
                InputSlotClass: winapi::um::d3d12::D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
                InstanceDataStepRate: 0,
            },
            winapi::um::d3d12::D3D12_INPUT_ELEMENT_DESC {
                SemanticName: unsafe { transmute("TEXCOORD".as_ptr()) },
                SemanticIndex: 0,
                Format: winapi::shared::dxgiformat::DXGI_FORMAT_R32G32_FLOAT,
                InputSlot: 0,
                AlignedByteOffset: 40,
                InputSlotClass: winapi::um::d3d12::D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
                InstanceDataStepRate: 0,
            },
        ];

        let mut blend_state_render_targets: [winapi::um::d3d12::D3D12_RENDER_TARGET_BLEND_DESC;
                                                8] = unsafe { zeroed() };
        for t in &mut blend_state_render_targets {
            t.BlendEnable = winapi::shared::minwindef::FALSE;
            t.LogicOpEnable = winapi::shared::minwindef::FALSE;
            t.SrcBlend = winapi::um::d3d12::D3D12_BLEND_ONE;
            t.DestBlend = winapi::um::d3d12::D3D12_BLEND_ZERO;
            t.BlendOp = winapi::um::d3d12::D3D12_BLEND_OP_ADD;
            t.SrcBlendAlpha = winapi::um::d3d12::D3D12_BLEND_ONE;
            t.DestBlendAlpha = winapi::um::d3d12::D3D12_BLEND_ZERO;
            t.BlendOpAlpha = winapi::um::d3d12::D3D12_BLEND_OP_ADD;
            t.LogicOp = winapi::um::d3d12::D3D12_LOGIC_OP_NOOP;
            t.RenderTargetWriteMask = winapi::um::d3d12::D3D12_COLOR_WRITE_ENABLE_ALL as u8;
        }

        let mut pso_desc: winapi::um::d3d12::D3D12_GRAPHICS_PIPELINE_STATE_DESC =
            unsafe { zeroed() };
        pso_desc.InputLayout.NumElements =
            input_element_descs.len() as winapi::shared::minwindef::UINT;
        pso_desc.InputLayout.pInputElementDescs = input_element_descs.as_ptr();
        pso_desc.pRootSignature = layout.root_signature;
        pso_desc.VS.BytecodeLength = vert_bytes.len();
        pso_desc.VS.pShaderBytecode = unsafe { transmute(vert_bytes.as_ptr()) };
        pso_desc.PS.BytecodeLength = frag_bytes.len();
        pso_desc.PS.pShaderBytecode = unsafe { transmute(frag_bytes.as_ptr()) };
        pso_desc.RasterizerState.FillMode = winapi::um::d3d12::D3D12_FILL_MODE_SOLID;
        pso_desc.RasterizerState.CullMode = winapi::um::d3d12::D3D12_CULL_MODE_BACK;
        pso_desc.RasterizerState.FrontCounterClockwise = winapi::shared::minwindef::FALSE;
        pso_desc.RasterizerState.DepthBias =
            winapi::um::d3d12::D3D12_DEFAULT_DEPTH_BIAS as winapi::shared::minwindef::INT;
        pso_desc.RasterizerState.DepthBiasClamp = winapi::um::d3d12::D3D12_DEFAULT_DEPTH_BIAS_CLAMP;
        pso_desc.RasterizerState.SlopeScaledDepthBias =
            winapi::um::d3d12::D3D12_DEFAULT_SLOPE_SCALED_DEPTH_BIAS;
        pso_desc.RasterizerState.DepthClipEnable = winapi::shared::minwindef::TRUE;
        pso_desc.RasterizerState.MultisampleEnable = winapi::shared::minwindef::FALSE;
        pso_desc.RasterizerState.AntialiasedLineEnable = winapi::shared::minwindef::FALSE;
        pso_desc.RasterizerState.ForcedSampleCount = 0;
        pso_desc.RasterizerState.ConservativeRaster =
            winapi::um::d3d12::D3D12_CONSERVATIVE_RASTERIZATION_MODE_OFF;
        pso_desc.BlendState.AlphaToCoverageEnable = winapi::shared::minwindef::FALSE;
        pso_desc.BlendState.IndependentBlendEnable = winapi::shared::minwindef::FALSE;
        pso_desc.BlendState.RenderTarget = blend_state_render_targets;
        pso_desc.DepthStencilState.DepthEnable = winapi::shared::minwindef::TRUE;
        pso_desc.DepthStencilState.StencilEnable = winapi::shared::minwindef::FALSE;
        pso_desc.SampleMask = winapi::vc::limits::UINT_MAX;
        pso_desc.PrimitiveTopologyType = winapi::um::d3d12::D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE;
        pso_desc.NumRenderTargets = 3;
        pso_desc.RTVFormats[0] = winapi::shared::dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM;
        pso_desc.SampleDesc.Count = 1;
        let mut pipeline_state: &'static mut winapi::um::d3d12::ID3D12PipelineState =
            unsafe { zeroed() };
        ThrowIfFailed!(device.get_data().CreateGraphicsPipelineState(
            &pso_desc,
            &winapi::um::d3d12::ID3D12PipelineState::uuidof(),
            transmute(&pipeline_state)
        ));
        Self {
            device,
            pipeline_state,
        }
    }
}

#[cfg(debug_mode)]
impl std::fmt::Debug for Pipeline {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "Directx12-Pipeline");
    }
}

unsafe impl Send for Pipeline {}

unsafe impl Sync for Pipeline {}

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct Manager {
    device: Arc<Device>,
    pipelines: BTreeMap<PipelineType, Weak<Pipeline>>,
}

impl Manager {
    pub(super) fn new(device: Arc<Device>) -> Self {
        let pipelines = BTreeMap::new();

        Self { device, pipelines }
    }

    pub(crate) fn create(
        &mut self,
        _render_pass: Arc<RenderPass>,
        pipeline_type: PipelineType,
        _config: &Configurations,
    ) -> Arc<Pipeline> {
        if let Some(p) = self.pipelines.get(&pipeline_type) {
            if let Some(p) = p.upgrade() {
                return p;
            }
        }
        let p = Arc::new(Pipeline::new(self.device.clone(), pipeline_type));
        self.pipelines.insert(pipeline_type, Arc::downgrade(&p));
        return p;
    }
}
