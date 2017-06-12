use std::ptr::null_mut;
use std::mem::size_of;
use super::super::core::application::ApplicationTrait;
use super::super::math::matrix::{Mat4x4, Mat3x3};
use super::super::math::vector::Vec3;
use super::super::system::os::OsApplication;
use super::super::render::engine::EngineTrait;
use super::super::system::metal as mtl;

pub struct Engine<CoreApp> where CoreApp: ApplicationTrait {
    pub core_app: *mut CoreApp,
    pub os_app: *mut OsApplication<CoreApp>,
}

const MAX_BUFFERS_COUNT: mtl::NSUInteger = 3;

#[repr(C)]
#[derive(Debug)]
pub struct Uniforms {
    pub projectionMatrix: Mat4x4<f32>,
    pub viewMatrix: Mat4x4<f32>,
    pub materialShininess: f32,
    pub modelViewMatrix: Mat4x4<f32>,
    pub normalMatrix: Mat3x3<f32>,
    pub ambientLightColor: Vec3<f32>,
    pub directionalLightDirection: Vec3<f32>,
    pub directionalLightColor: Vec3<f32>,
}

impl<CoreApp> EngineTrait<CoreApp> for Engine<CoreApp> where CoreApp: ApplicationTrait {
    fn new() -> Self {
        Engine {
            core_app: null_mut(),
            os_app: null_mut(),
        }
    }

    fn set_core_app(&mut self, c: *mut CoreApp) {
        self.core_app = c;
    }

    fn set_os_app(&mut self, o: *mut OsApplication<CoreApp>) {
        self.os_app = o;
    }

    fn initialize(&mut self) {
        let device = unsafe { (*self.os_app).metal_device };
        let asset_manager = unsafe {&mut (*self.os_app).asset_manager };
        let shader = asset_manager.get_shader(1, self.os_app);
        let uniform_buffer_size: mtl::NSUInteger =
            ((size_of::<Uniforms>() as mtl::NSUInteger & !0xFF) + 0x100) * MAX_BUFFERS_COUNT;
        let dynamic_uniform_buffer: mtl::Id = unsafe {
            msg_send![device, newBufferWithLength:uniform_buffer_size
                options:mtl::RESOURCE_STORAGE_MODE_SHARED] };
        let label = mtl::NSString::new("UniformBuffer");
        unsafe { msg_send![dynamic_uniform_buffer, setLabel:label.s]; }
        let vertex_descriptor = mtl::get_instance("MTLVertexDescriptor");
        const BUFFER_INDEX_MESH_POSITIONS: mtl::NSUInteger = 0;
        const BUFFER_INDEX_MESH_GENERICS:  mtl::NSUInteger = 1;
        const BUFFER_INDEX_UNIFORMS:       mtl::NSUInteger = 2;
        const VERTEX_ATTRIBUTE_POSITION:   mtl::NSUInteger = 0;
        const VERTEX_ATTRIBUTE_TEXCOORD:   mtl::NSUInteger = 1;
        const VERTEX_ATTRIBUTE_NORMAL:     mtl::NSUInteger = 2;
        let attributes: mtl::Id = unsafe { msg_send![vertex_descriptor, attributes] };
        let attribute: mtl::Id = unsafe { msg_send![
            attributes, objectAtIndexedSubscript:VERTEX_ATTRIBUTE_POSITION] };
        unsafe {
            msg_send![attribute, setFormat:mtl::VERTEX_FORMAT_FLOAT3];
            msg_send![attribute, setOffset:0 as mtl::NSUInteger];
            msg_send![attribute, setBufferIndex:BUFFER_INDEX_MESH_POSITIONS];
            msg_send![attributes,
                setObject:attribute atIndexedSubscript:VERTEX_ATTRIBUTE_POSITION];
        }
        let attribute: mtl::Id = unsafe { msg_send![
            attributes, objectAtIndexedSubscript:VERTEX_ATTRIBUTE_TEXCOORD] };
        unsafe {
            msg_send![attribute, setFormat:mtl::VERTEX_FORMAT_FLOAT2];
            msg_send![attribute, setOffset:0 as mtl::NSUInteger];
            msg_send![attribute, setBufferIndex:BUFFER_INDEX_MESH_GENERICS];
            msg_send![attributes,
                setObject:attribute atIndexedSubscript:VERTEX_ATTRIBUTE_TEXCOORD];
        }
        let attribute: mtl::Id = unsafe { msg_send![
            attributes, objectAtIndexedSubscript:VERTEX_ATTRIBUTE_NORMAL] };
        unsafe {
            msg_send![attribute, setFormat:mtl::VERTEX_FORMAT_HALF4];
            msg_send![attribute, setOffset:8 as mtl::NSUInteger];
            msg_send![attribute, setBufferIndex:BUFFER_INDEX_MESH_GENERICS];
            msg_send![attributes,
                setObject:attribute atIndexedSubscript:VERTEX_ATTRIBUTE_NORMAL];
        }
        let layouts: mtl::Id = unsafe { msg_send![vertex_descriptor, layouts] };
        let layout: mtl::Id = unsafe { msg_send![
            layouts, objectAtIndexedSubscript:BUFFER_INDEX_MESH_POSITIONS] };
        unsafe {
            msg_send![layout, setStride:12 as mtl::NSUInteger];
            msg_send![layout, setStepRate:1 as mtl::NSUInteger];
            msg_send![layout, setStepFunction:mtl::VERTEX_STEP_FUNCTION_PER_VERTEX];
            msg_send![layouts,
                setObject:layout atIndexedSubscript:BUFFER_INDEX_MESH_POSITIONS];
        }
        let layout: mtl::Id = unsafe { msg_send![
            layouts, objectAtIndexedSubscript:BUFFER_INDEX_MESH_GENERICS] };
        unsafe {
            msg_send![layout, setStride:16 as mtl::NSUInteger];
            msg_send![layout, setStepRate:1 as mtl::NSUInteger];
            msg_send![layout, setStepFunction:mtl::VERTEX_STEP_FUNCTION_PER_VERTEX];
            msg_send![layouts,
                setObject:layout atIndexedSubscript:BUFFER_INDEX_MESH_GENERICS];
        }
        let render_destination = unsafe { (*self.os_app).game_view_controller };
        let sample_count = 1 as mtl::NSUInteger;
        let depth_stencil_format = mtl::PIXEL_FORMAT_DEPTH32_FLOAT_STENCIL8;
        let color_format = mtl::PIXEL_FORMAT_BGRA8_UNORM_SRGB;
        unsafe {
            msg_send![render_destination, setDepthStencilPixelFormat:depth_stencil_format];
            msg_send![render_destination, setColorPixelFormat:color_format];
            msg_send![render_destination, setSampleCount:sample_count];
        }
        let pipeline_state_descriptor = mtl::get_instance("MTLRenderPipelineDescriptor");
        let label = mtl::NSString::new("MyPipeline");
        unsafe {
            msg_send![pipeline_state_descriptor, setLabel:label.s];
            msg_send![pipeline_state_descriptor, setSampleCount:sample_count];
            msg_send![pipeline_state_descriptor,
                setVertexFunction:shader.as_shader().vertex.function];
            msg_send![pipeline_state_descriptor,
                setFragmentFunction:shader.as_shader().fragment.function];
            msg_send![pipeline_state_descriptor, setVertexDescriptor:vertex_descriptor];
            msg_send![pipeline_state_descriptor, setcolorAttachments[0].pixelFormat = _renderDestination.colorPixelFormat];
            msg_send![pipeline_state_descriptor,
                setDepthAttachmentPixelFormat:depth_stencil_format];
            msg_send![pipeline_state_descriptor,
                setStencilAttachmentPixelFormat:depth_stencil_format];
        }
        //
        // NSError *error = NULL;
        // _pipelineState = [_device newRenderPipelineStateWithDescriptor:pipelineStateDescriptor error:&error];
        // if (!_pipelineState)
        // {
        //     NSLog(@"Failed to created pipeline state, error %@", error);
        // }
        //
        // MTLDepthStencilDescriptor *depthStateDesc = [[MTLDepthStencilDescriptor alloc] init];
        // depthStateDesc.depthCompareFunction = MTLCompareFunctionLess;
        // depthStateDesc.depthWriteEnabled = YES;
        // _depthState = [_device newDepthStencilStateWithDescriptor:depthStateDesc];
        //
        // // Create the command queue
        // _commandQueue = [_device newCommandQueue];

    }

    fn update(&mut self) {

    }

    fn terminate(&mut self) {

    }
}
