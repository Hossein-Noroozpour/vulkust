use std::sync::Arc;
use std::mem::transmute;
use super::super::math::matrix::Mat4x4;
use super::super::system::file::File;
use super::super::util::cell::DebugCell;
use super::buffer::Manager as BufferManager;
use super::device::logical::Logical as LogicalDevice;
use super::material::{read_material, Material};
use super::model::UniformData as MdlUniData;
use super::scene::UniformData as ScnUniData;
use super::shader::manager::Manager as ShaderManager;
use super::shader::read_id as read_shader_id;
use super::texture::manager::Manager as TextureManager;

pub const INDEX_ELEMENTS_SIZE: usize = 4;

pub struct Mesh {
    pub material: Arc<DebugCell<Material>>,
    pub vertex_size: u64,
    pub vertices_range: (usize, usize),
    pub indices_count: u64,
    pub indices_range: (usize, usize),
}

impl Mesh {
    pub fn new(
        file: &mut File,
        buffer_manager: &mut BufferManager,
        logical_device: Arc<LogicalDevice>,
        shader_manager: &mut ShaderManager,
        texture_manager: &mut TextureManager,
    ) -> Self {
        #[cfg(mesh_debug)]
        logi!("before material read {}", file.tell());
        let material = read_material(file, logical_device, shader_manager, texture_manager, buffer_manager);
        #[cfg(mesh_debug)]
        logi!("after material read {}", file.tell());
        let vertex_size = material.borrow().get_vertex_size();
        let vertices_count = file.read_count();
        #[cfg(mesh_debug)]
        logi!("mesh vertices count is: {}", vertices_count);
        let vertices_size = (vertex_size * vertices_count) as usize;
        let mut data = file.read_bytes(vertices_size);
        let vertices_range =
            buffer_manager.add_vi(unsafe { transmute(data.as_ptr()) }, vertices_size);
        let indices_count = file.read_count();
        let indices_size = (INDEX_ELEMENTS_SIZE * indices_count) as usize;
        data = file.read_bytes(indices_size as usize);
        let indices_range =
            buffer_manager.add_vi(unsafe { transmute(data.as_ptr()) }, indices_size);
        Mesh {
            material: material,
            vertex_size: vertex_size,
            vertices_range: vertices_range,
            indices_count: indices_count,
            indices_range: indices_range,
        }
    }

    pub fn update_uniform(&mut self, sud: &ScnUniData, mud: &MdlUniData, frame_index: usize) {
        self.material.borrow_mut().update_uniform(sud, mud, frame_index);
    }
}

#[derive(Default)]
struct OccUniform {
    pub mvp: Mat4x4<f32>,
}

pub struct OccMesh {
    pub vertices_range: (usize, usize),
    pub indices_count: u64,
    pub indices_range: (usize, usize),
    pub uniforms: Vec<&'static mut OccUniform>,
    pub uniforms_ranges: Vec<(usize, usize)>,
}

impl OccMesh {
    pub fn new(file: &mut File, buffer_manager: &mut BufferManager) -> Self {
        let _ = read_shader_id(file);
        let vertices_size = (3 * 4 * file.read_count()) as usize;
        #[cfg(mesh_debug)]
        logi!("occlusion mesh with vertices size {}", vertices_size);
        let mut data = file.read_bytes(vertices_size);
        let vertices_range =
            buffer_manager.add_vi(unsafe { transmute(data.as_ptr()) }, vertices_size);
        let indices_count = file.read_count();
        let indices_size = (INDEX_ELEMENTS_SIZE * indices_count) as usize;
        data = file.read_bytes(indices_size);
        let indices_range =
            buffer_manager.add_vi(unsafe { transmute(data.as_ptr()) }, indices_size);
        let (uniforms, uniforms_ranges) = buffer_manager.add_u(&OccUniform::default());
        OccMesh {
            vertices_range: vertices_range,
            indices_count: indices_count,
            indices_range: indices_range,
            uniforms: uniforms,
            uniforms_ranges: uniforms_ranges,
        }
    }

    pub fn update_uniform(&mut self, mud: &MdlUniData, frame_index: usize) {
        self.uniforms[frame_index].mvp = mud.mvp;
    }
}
