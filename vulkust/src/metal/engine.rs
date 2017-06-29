extern crate block;

use std::ptr::null_mut;
use std::mem::{size_of, transmute};
use std::os::raw::c_void;
use std::f32::consts::PI;
use std::sync::{Arc, Mutex};
use self::block::ConcreteBlock;
use super::super::objc;
use super::super::core::application::ApplicationTrait;
use super::super::math::matrix::{Mat4x4, SMat3x3F, SMat4x4F};
use super::super::math::vector::{Vec3, SVec3F, SVec3U32};
use super::super::sync::semaphore::Semaphore;
use super::super::system::os::OsApplication;
use super::super::render::engine::EngineTrait;
use super::super::render::texture::TextureTrait;
use super::super::system::metal as mtl;
use super::super::system::metal::kit as mtk;
use super::super::system::metal::model_io as mdl;

pub struct Engine<CoreApp>
where
    CoreApp: ApplicationTrait,
{
    pub core_app: *mut CoreApp,
    pub os_app: *mut OsApplication<CoreApp>,
    pub depth_state: mtl::Id,
    pub command_queue: mtl::Id,
    pub metal_vertex_descriptor: mtl::Id,
    pub texture_loader: mtl::Id,
    pub uniform_buffer_index: mtl::NSUInteger,
    pub uniform_buffer_size: mtl::NSUInteger,
    pub uniform_buffer_offset: mtl::NSUInteger,
    pub uniform_buffer_address: *mut c_void,
    pub dynamic_uniform_buffer: mtl::Id,
    pub projection_matrix: Mat4x4<f32>,
    pub rotation: f32,
    pub in_flight_semaphore: Arc<Mutex<Semaphore>>,
    pub pipeline_state: mtl::Id,
    pub mtk_mesh: mtl::Id,
    pub texture: Option<Arc<TextureTrait>>,
}

pub const MAX_BUFFERS_COUNT: mtl::NSUInteger = 3;
pub const BUFFER_INDEX_MESH_POSITIONS: mtl::NSUInteger = 0;
pub const BUFFER_INDEX_MESH_GENERICS: mtl::NSUInteger = 1;
pub const BUFFER_INDEX_UNIFORMS: mtl::NSUInteger = 2;
pub const VERTEX_ATTRIBUTE_POSITION: mtl::NSUInteger = 0;
pub const VERTEX_ATTRIBUTE_TEXCOORD: mtl::NSUInteger = 1;
pub const VERTEX_ATTRIBUTE_NORMAL: mtl::NSUInteger = 2;
pub const TEXTURE_INDEX_COLOR: mtl::NSUInteger = 0;

#[repr(C)]
pub struct Uniforms {
    pub projection_matrix: SMat4x4F,
    pub view_matrix: SMat4x4F,
    pub material_shininess: f32,
    pub model_view_matrix: SMat4x4F,
    pub normal_matrix: SMat3x3F,
    pub ambient_light_color: SVec3F,
    pub directional_light_direction: SVec3F,
    pub directional_light_color: SVec3F,
}

impl<CoreApp> EngineTrait<CoreApp> for Engine<CoreApp>
where
    CoreApp: ApplicationTrait,
{
    fn new() -> Self {
        Engine {
            core_app: null_mut(),
            os_app: null_mut(),
            depth_state: null_mut(),
            command_queue: null_mut(),
            metal_vertex_descriptor: null_mut(),
            texture_loader: null_mut(),
            uniform_buffer_index: 0,
            uniform_buffer_size: 0,
            uniform_buffer_offset: 0,
            uniform_buffer_address: null_mut(),
            dynamic_uniform_buffer: null_mut(),
            projection_matrix: Mat4x4::projection(65.0 / (PI / 180.0), 1.5, 0.1, 100.0),
            rotation: 0.0,
            in_flight_semaphore: Arc::new(Mutex::new(Semaphore::new(MAX_BUFFERS_COUNT as isize))),
            pipeline_state: null_mut(),
            mtk_mesh: null_mut(),
            texture: None,
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
        self.render();
    }

    fn terminate(&mut self) {}
}

impl<CoreApp> Engine<CoreApp>
where
    CoreApp: ApplicationTrait,
{
    fn load_metal(&mut self) {
        let device = unsafe { (*self.os_app).metal_device };
        let asset_manager = unsafe { &mut (*self.os_app).asset_manager };
        let shader = asset_manager.get_shader(1, self.os_app);
        let uniform_buffer_size: mtl::NSUInteger =
            ((size_of::<Uniforms>() as mtl::NSUInteger & !0xFF) + 0x100) * MAX_BUFFERS_COUNT;
        self.uniform_buffer_size = uniform_buffer_size;
        let dynamic_uniform_buffer: mtl::Id = unsafe {
            msg_send![device, newBufferWithLength:uniform_buffer_size
                options:mtl::RESOURCE_STORAGE_MODE_SHARED]
        };
        self.dynamic_uniform_buffer = dynamic_uniform_buffer;
        let label = mtl::NSString::new("UniformBuffer");
        unsafe {
            let _: () = msg_send![dynamic_uniform_buffer, setLabel:label.s];
        }
        let vertex_descriptor = mtl::get_instance("MTLVertexDescriptor");
        let attributes: mtl::Id = unsafe { msg_send![vertex_descriptor, attributes] };
        let attribute: mtl::Id = unsafe {
            msg_send![
                attributes,
                objectAtIndexedSubscript: VERTEX_ATTRIBUTE_POSITION
            ]
        };
        unsafe {
            let _: () = msg_send![attribute, setFormat: mtl::VERTEX_FORMAT_FLOAT3];
            let _: () = msg_send![attribute, setOffset:0 as mtl::NSUInteger];
            let _: () = msg_send![attribute, setBufferIndex: BUFFER_INDEX_MESH_POSITIONS];
            let _: () = msg_send![attributes,
                setObject:attribute atIndexedSubscript:VERTEX_ATTRIBUTE_POSITION];
        }
        let attribute: mtl::Id = unsafe {
            msg_send![
                attributes,
                objectAtIndexedSubscript: VERTEX_ATTRIBUTE_TEXCOORD
            ]
        };
        unsafe {
            let _: () = msg_send![attribute, setFormat: mtl::VERTEX_FORMAT_FLOAT2];
            let _: () = msg_send![attribute, setOffset:0 as mtl::NSUInteger];
            let _: () = msg_send![attribute, setBufferIndex: BUFFER_INDEX_MESH_GENERICS];
            let _: () = msg_send![attributes,
                setObject:attribute atIndexedSubscript:VERTEX_ATTRIBUTE_TEXCOORD];
        }
        let attribute: mtl::Id = unsafe {
            msg_send![
                attributes,
                objectAtIndexedSubscript: VERTEX_ATTRIBUTE_NORMAL
            ]
        };
        unsafe {
            let _: () = msg_send![attribute, setFormat: mtl::VERTEX_FORMAT_HALF4];
            let _: () = msg_send![attribute, setOffset:8 as mtl::NSUInteger];
            let _: () = msg_send![attribute, setBufferIndex: BUFFER_INDEX_MESH_GENERICS];
            let _: () = msg_send![attributes,
                setObject:attribute atIndexedSubscript:VERTEX_ATTRIBUTE_NORMAL];
        }
        let layouts: mtl::Id = unsafe { msg_send![vertex_descriptor, layouts] };
        let layout: mtl::Id = unsafe {
            msg_send![
                layouts,
                objectAtIndexedSubscript: BUFFER_INDEX_MESH_POSITIONS
            ]
        };
        unsafe {
            let _: () = msg_send![layout, setStride:12 as mtl::NSUInteger];
            let _: () = msg_send![layout, setStepRate:1 as mtl::NSUInteger];
            let _: () = msg_send![
                layout,
                setStepFunction: mtl::VERTEX_STEP_FUNCTION_PER_VERTEX
            ];
            let _: () = msg_send![layouts,
                setObject:layout atIndexedSubscript:BUFFER_INDEX_MESH_POSITIONS];
        }
        let layout: mtl::Id = unsafe {
            msg_send![
                layouts,
                objectAtIndexedSubscript: BUFFER_INDEX_MESH_GENERICS
            ]
        };
        unsafe {
            let _: () = msg_send![layout, setStride:16 as mtl::NSUInteger];
            let _: () = msg_send![layout, setStepRate:1 as mtl::NSUInteger];
            let _: () = msg_send![
                layout,
                setStepFunction: mtl::VERTEX_STEP_FUNCTION_PER_VERTEX
            ];
            let _: () = msg_send![layouts,
                setObject:layout atIndexedSubscript:BUFFER_INDEX_MESH_GENERICS];
        }
        let render_destination = unsafe { (*self.os_app).game_view_controller };
        let sample_count = 1 as mtl::NSUInteger;
        let depth_stencil_format = mtl::PIXEL_FORMAT_DEPTH32_FLOAT_STENCIL8;
        let color_format = mtl::PIXEL_FORMAT_BGRA8_UNORM_SRGB;
        unsafe {
            let _: () = msg_send![
                render_destination,
                setDepthStencilPixelFormat: depth_stencil_format
            ];
            let _: () = msg_send![render_destination, setColorPixelFormat: color_format];
            let _: () = msg_send![render_destination, setSampleCount: sample_count];
        }
        let pipeline_state_descriptor = mtl::get_instance("MTLRenderPipelineDescriptor");
        let label = mtl::NSString::new("MyPipeline");
        unsafe {
            let _: () = msg_send![pipeline_state_descriptor, setLabel:label.s];
            let _: () = msg_send![pipeline_state_descriptor, setSampleCount: sample_count];
            let _: () = msg_send![pipeline_state_descriptor,
                setVertexFunction:shader.as_shader().vertex.function];
            let _: () = msg_send![pipeline_state_descriptor,
                setFragmentFunction:shader.as_shader().fragment.function];
            let _: () = msg_send![
                pipeline_state_descriptor,
                setVertexDescriptor: vertex_descriptor
            ];
            let color_attachments: mtl::Id = msg_send![pipeline_state_descriptor, colorAttachments];
            let color_attachment: mtl::Id = msg_send![
                color_attachments, objectAtIndexedSubscript:0 as mtl::NSUInteger];
            let _: () = msg_send![color_attachment, setPixelFormat: color_format];
            let _: () = msg_send![
                color_attachments, setObject:color_attachment
                atIndexedSubscript:0 as mtl::NSUInteger];
            let _: () = msg_send![
                pipeline_state_descriptor,
                setDepthAttachmentPixelFormat: depth_stencil_format
            ];
            let _: () = msg_send![
                pipeline_state_descriptor,
                setStencilAttachmentPixelFormat: depth_stencil_format
            ];
        }
        let mut error = mtl::NSError::null();
        let pipeline_state: mtl::Id = unsafe {
            msg_send![
            device,
            newRenderPipelineStateWithDescriptor:pipeline_state_descriptor
            error:error.as_ptr()]
        };
        if pipeline_state == null_mut() {
            logf!("Failed to created pipeline state, error is {}", error);
        }
        self.pipeline_state = pipeline_state;
        let depth_state_desc = mtl::get_instance("MTLDepthStencilDescriptor");
        unsafe {
            let _: () = msg_send![
                depth_state_desc,
                setDepthCompareFunction: mtl::COMPARE_FUNCTION_LESS
            ];
            let _: () = msg_send![depth_state_desc, setDepthWriteEnabled: mtl::YES];
        }
        self.depth_state =
            unsafe { msg_send![device, newDepthStencilStateWithDescriptor: depth_state_desc] };
        self.command_queue = unsafe { msg_send![device, newCommandQueue] };
        self.metal_vertex_descriptor = vertex_descriptor;
    }

    fn load_assets(&mut self) {
        let device = unsafe { (*self.os_app).metal_device };
        let mut error = mtl::NSError::null();
        let metal_allocator: mtl::Id =
            unsafe { msg_send![mtl::alloc("MTKMeshBufferAllocator"), initWithDevice: device] };
        let dimension = SVec3F(4.0, 4.0, 4.0);
        let segments = SVec3U32(2, 2, 2);
        let geometry_type = mtl::GEOMETRY_TYPE_TRIANGLES;
        let inward_normals = mtl::NO;
        let class = mtl::get_class("MDLMesh");
        let mdl_mesh: mtl::Id = mtl::util::send_unverified(
            class,
            sel!(newBoxWithDimensions:segments:geometryType:inwardNormals:allocator:),
            (
                dimension,
                segments,
                geometry_type,
                inward_normals,
                metal_allocator,
            ),
        );
        let modle_vertex_descriptor =
            mtk::model_io_vertex_descriptor_from_metal(self.metal_vertex_descriptor);
        let attributes: mtl::Id = unsafe { msg_send![modle_vertex_descriptor, attributes] };
        let set = |attributes: mtl::Id, index: mtl::NSUInteger, att: mtl::Id| {
            let attribute: mtl::Id =
                unsafe { msg_send![attributes, objectAtIndexedSubscript: index] };
            let _: () = unsafe { msg_send![attribute, setName: att] };
            let _: () = unsafe {
                msg_send![attributes, setObject:attribute
                atIndexedSubscript:index]
            };
        };
        set(attributes, VERTEX_ATTRIBUTE_POSITION, unsafe {
            mdl::MDLVertexAttributePosition
        });
        set(attributes, VERTEX_ATTRIBUTE_TEXCOORD, unsafe {
            mdl::MDLVertexAttributeTextureCoordinate
        });
        set(attributes, VERTEX_ATTRIBUTE_NORMAL, unsafe {
            mdl::MDLVertexAttributeNormal
        });
        unsafe {
            let _: () = msg_send![mdl_mesh, setVertexDescriptor: modle_vertex_descriptor];
        }
        let mtk_mesh: mtl::Id = unsafe {
            msg_send![mtl::alloc("MTKMesh"),
            initWithMesh:mdl_mesh device:device error:error.as_ptr()]
        };
        if mtk_mesh == null_mut() || error.is_error() {
            logf!("Creating MetalKit mesh failed with error {}", error);
        }
        self.mtk_mesh = mtk_mesh;
        self.texture_loader =
            unsafe { msg_send![mtl::alloc("MTKTextureLoader"), initWithDevice: device] };
        self.texture = Some(unsafe {
            (*self.os_app).asset_manager.get_texture(1, self.os_app)
        });
    }

    fn update_dynamic_buffer_state(&mut self) {
        self.uniform_buffer_index = (self.uniform_buffer_index + 1) % MAX_BUFFERS_COUNT;
        self.uniform_buffer_offset = self.uniform_buffer_size as mtl::NSUInteger *
            self.uniform_buffer_index as mtl::NSUInteger;
        self.uniform_buffer_address = unsafe { msg_send![self.dynamic_uniform_buffer, contents] };
        let tmp_add: *mut u8 = unsafe { transmute(self.uniform_buffer_address) };
        self.uniform_buffer_address =
            unsafe { transmute(tmp_add.offset(self.uniform_buffer_offset as isize)) };
    }

    fn update_game_state(&mut self) {
        let uniforms: &mut Uniforms = unsafe { transmute(self.uniform_buffer_address) };
        let ambient_light_color = SVec3F(0.02, 0.02, 0.02);
        uniforms.ambient_light_color = ambient_light_color;
        let directional_light_direction = SVec3F(0.0, 0.0, -1.0);
        uniforms.directional_light_direction = directional_light_direction;
        let directional_light_color = SVec3F(0.7, 0.7, 0.7);
        uniforms.directional_light_color = directional_light_color;
        uniforms.material_shininess = 30.0;
        let mut view_matrix = Mat4x4::new();
        view_matrix.translate(0.0f32, 0.0, -8.0);
        uniforms.view_matrix = view_matrix.get_smat4x4f();
        uniforms.projection_matrix = self.projection_matrix.get_smat4x4f();
        let mut rotation_axis = Vec3::new(1f32);
        rotation_axis.z = 0.0;
        let model_matrix = Mat4x4::rotation(self.rotation, &rotation_axis);
        let model_view_matrix = &view_matrix * &model_matrix;
        uniforms.model_view_matrix = model_view_matrix.get_smat4x4f();
        let normal_matrix = model_view_matrix.get_mat3x3();
        uniforms.normal_matrix = normal_matrix.inv().t().get_smat3x3f();
        self.rotation += 0.01;
    }

    pub fn draw_rect_resized(&mut self, size: &mtl::NSSize) {
        let aspect = size.width / size.height;
        self.projection_matrix = Mat4x4::projection(65.0 / (PI / 180.0), aspect as f32, 0.1, 100.0);
    }

    pub fn render(&mut self) {
        self.in_flight_semaphore.lock().unwrap().acquire();
        let command_buffer: mtl::Id = unsafe { msg_send![self.command_queue, commandBuffer] };
        unsafe { let _: () =  msg_send![command_buffer, setLabel:mtl::NSString::new("MyCommand")]; }
        let in_flight_semaphore = self.in_flight_semaphore.clone();
        let b = ConcreteBlock::new(move |_buffer: mtl::Id| {
            in_flight_semaphore.lock().unwrap().release();
        });
        #[repr(C)]
        struct EncoderTmp<T> {
            pub b: *const T
        }
        unsafe impl<T> objc::Encode for EncoderTmp<T> {
            fn encode() -> objc::Encoding {
                unsafe { objc::Encoding::from_str("@?") }
            }
        }
        let b = b.copy();
        let b = EncoderTmp {
            b: &*b
        };
        unsafe { let _: () = msg_send![command_buffer, addCompletedHandler:b]; }
        self.update_dynamic_buffer_state();
        self.update_game_state();
        let render_pass_descriptor: mtl::Id = unsafe { msg_send![
            (*self.os_app).game_view_controller, currentRenderPassDescriptor] };
        if render_pass_descriptor != null_mut() {
            let render_encoder: mtl::Id = unsafe { msg_send![
                command_buffer, renderCommandEncoderWithDescriptor:render_pass_descriptor] };
            unsafe {
                let _: () = msg_send![
                    render_encoder, setLabel:mtl::NSString::new("MyRenderEncoder")];
                let _: () = msg_send![render_encoder, pushDebugGroup:mtl::NSString::new("DrawBox")];
                let _: () = msg_send![
                    render_encoder, setFrontFacingWinding:mtl::WINDING_COUNTER_CLOCKWISE];
                let _: () = msg_send![render_encoder, setCullMode:mtl::CULL_MODE_BACK];
                let _: () = msg_send![render_encoder, setRenderPipelineState:self.pipeline_state];
                let _: () = msg_send![render_encoder, setDepthStencilState:self.depth_state];
                let _: () = msg_send![
                    render_encoder, setVertexBuffer:self.dynamic_uniform_buffer 
                    offset:self.uniform_buffer_offset atIndex:BUFFER_INDEX_UNIFORMS];
                let _: () = msg_send![
                    render_encoder, setFragmentBuffer:self.dynamic_uniform_buffer 
                    offset:self.uniform_buffer_offset atIndex:BUFFER_INDEX_UNIFORMS];
            }
            let vertex_buffers: mtl::Id = unsafe { msg_send![self.mtk_mesh, vertexBuffers] };
            let buffer_index_count: mtl::NSUInteger = unsafe { msg_send![vertex_buffers, count] };
            for buffer_index in 0..buffer_index_count {
                let vertex_buffer: mtl::Id = unsafe { msg_send![
                    vertex_buffers, objectAtIndexedSubscript:buffer_index] };
                let ns_null = mtl::get_class("NSNull");
                let ns_null: mtl::Id = unsafe { msg_send![ns_null, null] };
                if vertex_buffer != ns_null {
                    let buffer: mtl::Id = unsafe { msg_send![vertex_buffer, buffer] };
                    let offset: mtl::NSUInteger = unsafe { msg_send![vertex_buffer, offset] };
                    unsafe { let _: () = msg_send![
                        render_encoder, setVertexBuffer:buffer offset:offset atIndex:buffer_index];
                    }
                }
            }
            unsafe { let _: () = msg_send![
                render_encoder, 
                setFragmentTexture:self.texture.as_ref().unwrap().as_texture2d().raw.color_map
                atIndex:TEXTURE_INDEX_COLOR]; 
            }
            let submeshes: mtl::Id = unsafe {
                msg_send![self.mtk_mesh, submeshes]
            };
            let submeshes_count: mtl::NSUInteger = unsafe { msg_send![submeshes, count] };
            for submesh_index in 0..submeshes_count {
                let submesh: mtl::Id = unsafe { msg_send![
                    submeshes, objectAtIndexedSubscript:submesh_index] };
                let primitive_type: mtl::NSUInteger = unsafe { msg_send![submesh, primitiveType] };
                let index_count: mtl::NSUInteger = unsafe { msg_send![submesh, indexCount] };
                let index_type: mtl::NSUInteger = unsafe { msg_send![submesh, indexType] };
                let index_buffer: mtl::Id = unsafe { msg_send![submesh, indexBuffer] };
                let buffer: mtl::Id = unsafe { msg_send![index_buffer, buffer] };
                let offset: mtl::NSUInteger = unsafe { msg_send![index_buffer, offset] };
                unsafe { let _: () = msg_send![
                    render_encoder, drawIndexedPrimitives:primitive_type 
                    indexCount:index_count indexType:index_type
                    indexBuffer:buffer indexBufferOffset:offset];
                }
            }
            unsafe {
                let _: () = msg_send![render_encoder, popDebugGroup];
                let _: () = msg_send![render_encoder, endEncoding];
            }
        }
        let current_drawable: mtl::Id = unsafe { msg_send![
            (*self.os_app).game_view_controller, currentDrawable] 
        };
        unsafe {
            let _: () = msg_send![command_buffer, presentDrawable:current_drawable];
            let _: () = msg_send![command_buffer, commit];
        }
    }
}
