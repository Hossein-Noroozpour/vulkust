use std::sync::Arc;
use std::cell::RefCell;
use std::mem::transmute;
use super::super::core::application::ApplicationTrait;
use super::super::math::matrix::Mat4x4;
use super::super::system::os::OsApplication;
use super::super::system::file::File;
use super::buffer::Buffer;
use super::material::{read_material, Material};

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
        indices_buffer: &mut Buffer) -> Self {
        let material = read_material(file);
        let vertex_size = material.borrow().get_vertex_size();
        let vertices_size = vertex_size * file.read_count();
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
