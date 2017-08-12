use std::sync::Arc;
use std::cell::RefCell;
use std::mem::transmute;
use super::super::system::file::File;
use super::buffer::Buffer;
use super::device::logical::Logical as LogicalDevice;
use super::material::{read_material, Material};
use super::shader::manager::Manager as ShaderManager;
use super::shader::read_id as read_shader_id;
use super::texture::manager::Manager as TextureManager;

pub const INDEX_ELEMENTS_SIZE: u64 = 4;

pub struct Mesh {
    pub material: Arc<RefCell<Material>>,
    pub vertex_size: u64,
    pub vertices_size: u64,
    pub vertices_buffer_offset: u64,
    pub indices_size: u64,
    pub indices_buffer_offset: u64,
}

impl Mesh {
    pub fn new(
        file: &mut File,
        vertices_buffer: &mut Buffer,
        indices_buffer: &mut Buffer,
        logical_device: Arc<LogicalDevice>,
        shader_manager: &mut ShaderManager,
        texture_manager: &mut TextureManager,
    ) -> Self {
        #[cfg(mesh_debug)]
        logi!("before material read {}", file.tell());
        let material = read_material(file, logical_device, shader_manager, texture_manager);
        #[cfg(mesh_debug)]
        logi!("after material read {}", file.tell());
        let vertex_size = material.borrow().get_vertex_size();
        let vertices_count = file.read_count();
        #[cfg(mesh_debug)]
        logi!("mesh vertices count is: {}", vertices_count);
        let vertices_size = vertex_size * vertices_count;
        let data = file.read_bytes(vertices_size as usize);
        let vertices_buffer_offset = vertices_buffer.offset;
        vertices_buffer.write(unsafe { transmute(data.as_ptr()) }, vertices_size);
        let indices_size = INDEX_ELEMENTS_SIZE * file.read_count();
        let data = file.read_bytes(indices_size as usize);
        let indices_buffer_offset = indices_buffer.offset;
        indices_buffer.write(unsafe { transmute(data.as_ptr()) }, indices_size);
        Mesh {
            material: material,
            vertex_size: vertex_size,
            vertices_size: vertices_size,
            vertices_buffer_offset: vertices_buffer_offset,
            indices_size: indices_size,
            indices_buffer_offset: indices_buffer_offset,
        }
    }
}

pub struct OccMesh {
    pub vertices_size: u64,
    pub vertices_buffer_offset: u64,
    pub indices_size: u64,
    pub indices_buffer_offset: u64,
}

impl OccMesh {
    pub fn new(
        file: &mut File,
        vertices_buffer: &mut Buffer,
        indices_buffer: &mut Buffer,
    ) -> Self {
        let _ = read_shader_id(file);
        let vertices_size = 3 * 4 * file.read_count();
        #[cfg(mesh_debug)]
        logi!("occlusion mesh with vertices size {}", vertices_size);
        let data = file.read_bytes(vertices_size as usize);
        let vertices_buffer_offset = vertices_buffer.offset;
        vertices_buffer.write(unsafe { transmute(data.as_ptr()) }, vertices_size);
        let indices_size = INDEX_ELEMENTS_SIZE * file.read_count();
        let data = file.read_bytes(indices_size as usize);
        let indices_buffer_offset = indices_buffer.offset;
        indices_buffer.write(unsafe { transmute(data.as_ptr()) }, indices_size);
        OccMesh {
            vertices_size: vertices_size,
            vertices_buffer_offset: vertices_buffer_offset,
            indices_size: indices_size,
            indices_buffer_offset: indices_buffer_offset,
        }
    }
}
