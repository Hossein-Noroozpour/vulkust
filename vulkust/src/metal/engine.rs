#![allow(resolve_trait_on_defaulted_unit)]
use std::ptr::null_mut;
use std::mem::size_of;
use super::super::core::application::ApplicationTrait;
use super::super::math::matrix::{Mat4x4, Mat3x3};
use super::super::math::vector::Vec3;
use super::super::system::os::OsApplication;
use super::super::render::engine::EngineTrait;
use super::super::system::metal as mtl;
use super::super::system::metal::kit as mtk;
use super::super::objc::__send_message;

pub struct Engine<CoreApp> where CoreApp: ApplicationTrait {
    pub core_app: *mut CoreApp,
    pub os_app: *mut OsApplication<CoreApp>,
    pub depth_state: mtl::Id,
    pub command_queue: mtl::Id,
    pub metal_vertex_descriptor: mtl::Id,
}

pub const MAX_BUFFERS_COUNT: mtl::NSUInteger = 3;
pub const BUFFER_INDEX_MESH_POSITIONS: mtl::NSUInteger = 0;
pub const BUFFER_INDEX_MESH_GENERICS:  mtl::NSUInteger = 1;
pub const BUFFER_INDEX_UNIFORMS:       mtl::NSUInteger = 2;
pub const VERTEX_ATTRIBUTE_POSITION:   mtl::NSUInteger = 0;
pub const VERTEX_ATTRIBUTE_TEXCOORD:   mtl::NSUInteger = 1;
pub const VERTEX_ATTRIBUTE_NORMAL:     mtl::NSUInteger = 2;

#[repr(C)]
#[derive(Debug)]
pub struct Uniforms {
    pub projection_matrix: Mat4x4<f32>,
    pub view_matrix: Mat4x4<f32>,
    pub material_shininess: f32,
    pub model_view_matrix: Mat4x4<f32>,
    pub normal_matrix: Mat3x3<f32>,
    pub ambient_light_color: Vec3<f32>,
    pub directional_light_direction: Vec3<f32>,
    pub directional_light_color: Vec3<f32>,
}

impl<CoreApp> EngineTrait<CoreApp> for Engine<CoreApp> where CoreApp: ApplicationTrait {
    fn new() -> Self {
        Engine {
            core_app: null_mut(),
            os_app: null_mut(),
            depth_state: null_mut(),
            command_queue: null_mut(),
            metal_vertex_descriptor: null_mut(),
        }
    }

    fn set_core_app(&mut self, c: *mut CoreApp) {
        self.core_app = c;
    }

    fn set_os_app(&mut self, o: *mut OsApplication<CoreApp>) {
        self.os_app = o;
    }

    fn initialize(&mut self) {
        self.load_metal();
        self.load_assets();
    }

    fn update(&mut self) {

    }

    fn terminate(&mut self) {

    }
}

impl<CoreApp> Engine<CoreApp> where CoreApp: ApplicationTrait {
    fn load_metal(&mut self) {
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
            let color_attachments: mtl::Id = msg_send![
                pipeline_state_descriptor, colorAttachments];
            let color_attachment: mtl::Id = msg_send![
                color_attachments, objectAtIndexedSubscript:0 as mtl::NSUInteger];
            msg_send![color_attachment, setPixelFormat:color_format];
            msg_send![
                color_attachments, setObject:color_attachment
                atIndexedSubscript:0 as mtl::NSUInteger];
            msg_send![pipeline_state_descriptor,
                setDepthAttachmentPixelFormat:depth_stencil_format];
            msg_send![pipeline_state_descriptor,
                setStencilAttachmentPixelFormat:depth_stencil_format];
        }
        let mut error = mtl::NSError::null();
        let pipeline_state: mtl::Id = unsafe { msg_send![
            device,
            newRenderPipelineStateWithDescriptor:pipeline_state_descriptor
            error:error.as_ptr()] };
        if pipeline_state == null_mut() {
            logf!("Failed to created pipeline state, error is {}", error);
        }
        let depth_state_desc = mtl::get_instance("MTLDepthStencilDescriptor");
        unsafe {
            msg_send![depth_state_desc, setDepthCompareFunction:mtl::COMPARE_FUNCTION_LESS];
            msg_send![depth_state_desc, setDepthWriteEnabled:mtl::YES];
        }
        self.depth_state = unsafe { msg_send![
            device, newDepthStencilStateWithDescriptor:depth_state_desc] };
        self.command_queue = unsafe { msg_send![device, newCommandQueue] };
        self.metal_vertex_descriptor = vertex_descriptor;
    }

    fn load_assets(&mut self) {
        let device = unsafe { (*self.os_app).metal_device };
        let mut error = mtl::NSError::null();
        let metal_allocator: mtl::Id = unsafe { msg_send![
            mtl::alloc("MTKMeshBufferAllocator"), initWithDevice:device] };
        #[repr(simd)]
        struct Fv3 (f32, f32, f32);
        #[repr(simd)]
        struct Uv3 (u32, u32, u32);
        let dimension = Fv3(4.0f32, 4.0f32, 4.0f32);
        let segments = Uv3(2, 2, 2);
        let geometry_type = mtl::GEOMETRY_TYPE_TRIANGLES;
        let inward_normals = mtl::NO;
        let class = mtl::get_class("MDLMesh");
        let sel = sel!(newBoxWithDimensions:segments:geometryType:inwardNormals:allocator:);
        let mdl_mesh: mtl::Id = mtl::util::send_unverified(class, sel, (
            dimension, segments, geometry_type, inward_normals, metal_allocator
        ));
        // let mdl_mesh: mtl::Id = unsafe { msg_send![
        //     class, newBoxWithDimensions:dimension segments:segments geometryType:geometry_type
        //     inwardNormals:inward_normals allocator:metal_allocator] };

        // let modle_vertex_descriptor =
        //     mtk::model_io_vertex_descriptor_from_metal(self.metal_vertex_descriptor);

        // // Indicate how each Metal vertex descriptor attribute maps to each ModelIO attribute
        // mdlVertexDescriptor.attributes[kVertexAttributePosition].name  = MDLVertexAttributePosition;
        // mdlVertexDescriptor.attributes[kVertexAttributeTexcoord].name  = MDLVertexAttributeTextureCoordinate;
        // mdlVertexDescriptor.attributes[kVertexAttributeNormal].name    = MDLVertexAttributeNormal;
        //
        // // Perform the format/relayout of mesh vertices by setting the new vertex descriptor in our
        // //   Model IO mesh
        // mdlMesh.vertexDescriptor = mdlVertexDescriptor;
        //
        // // Crewte a MetalKit mesh (and submeshes) backed by Metal buffers
        // _mesh = [[MTKMesh alloc] initWithMesh:mdlMesh
        //                                device:_device
        //                                 error:&error];
        //
        // if(!_mesh || error)
        // {
        //     NSLog(@"Error creating MetalKit mesh %@", error.localizedDescription);
        // }
        //
        // // Use MetalKit's to load textures from our asset catalog (Assets.xcassets)
        // MTKTextureLoader* textureLoader = [[MTKTextureLoader alloc] initWithDevice:_device];
        //
        // // Load our textures with shader read using private storage
        // NSDictionary *textureLoaderOptions =
        // @{
        //   MTKTextureLoaderOptionTextureUsage       : @(MTLTextureUsageShaderRead),
        //   MTKTextureLoaderOptionTextureStorageMode : @(MTLStorageModePrivate)
        //   };
        //
        // _colorMap = [textureLoader newTextureWithName:@"ColorMap"
        //                                      scaleFactor:1.0
        //                                           bundle:nil
        //                                          options:textureLoaderOptions
        //                                            error:&error];
        //
        // if(!_colorMap || error)
        // {
        //     NSLog(@"Error creating texture %@", error.localizedDescription);
        // }
    }
}
