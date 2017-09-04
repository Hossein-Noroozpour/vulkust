use std::sync::Arc;
use std::mem::transmute;
use super::super::core::application::ApplicationTrait;
use super::super::math::matrix::Mat4x4;
use super::super::system::file::File;
use super::super::util::cell::DebugCell;
use super::buffer::{
    Manager as BufferManager,
    MeshBuffer,
};
use super::engine::RenderEngine;
use super::material::{read_material, Material};
use super::model::UniformData as MdlUniData;
use super::scene::UniformData as ScnUniData;
use super::shader::{read_id as read_shader_id, get_vertex_size};

pub const INDEX_ELEMENTS_SIZE: usize = 4;

pub struct Mesh {
    pub buffer: Arc<DebugCell<MeshBuffer>>,
    pub buffer_manager: Arc<DebugCell<BufferManager>>,
    pub material: Arc<DebugCell<Material>>,
}

impl Mesh {
    pub fn new<CoreApp>(
        file: &Arc<DebugCell<File>>,
        engine: &mut RenderEngine<CoreApp>
    ) -> Self
    where CoreApp: ApplicationTrait {
        let buffer_manager = engine.buffer_manager.as_ref().unwrap().clone();
        #[cfg(mesh_debug)]
        logi!("before material read {}", file.borrow().tell());
        let material = read_material(file, engine);
        #[cfg(mesh_debug)]
        logi!("after material read {}", file.borrow().tell());
        let vertex_size = material.borrow().get_shader().borrow().get_vertex_size();
        let vertices_count = file.borrow_mut().read_count();
        #[cfg(mesh_debug)]
        logi!("mesh vertices count is: {}", vertices_count);
        let vertices_size = vertex_size * (vertices_count as usize);
        let vertices_data = file.borrow_mut().read_bytes(vertices_size);
        let indices_count = file.borrow_mut().read_count();
        let buffer = buffer_manager.borrow_mut().create_mesh(
            vertex_size, vertices_count as usize, indices_count as usize
        );
        buffer.borrow_mut().upload_vertices(
            unsafe { transmute(vertices_data.as_ptr()) }, vertices_size);
        let indices_size = INDEX_ELEMENTS_SIZE * (indices_count as usize);
        let indices_data = file.borrow_mut().read_bytes(indices_size);
        buffer.borrow_mut().upload_indices(
            unsafe { transmute(indices_data.as_ptr()) }, indices_size);
        Mesh {
            buffer: buffer,
            buffer_manager: buffer_manager,
            material: material,
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
    pub buffer: Arc<DebugCell<MeshBuffer>>,
    pub buffer_manager: Arc<DebugCell<BufferManager>>,
}

impl OccMesh {
    pub fn new<CoreApp>(
        file: &Arc<DebugCell<File>>, 
        engine: &mut RenderEngine<CoreApp>,
    ) -> Self
    where CoreApp: ApplicationTrait {
        let shader_id = read_shader_id(file);
        #[cfg(mesh_debug)]
        {
            if shader_id != 0 {
                logf!("Only zero excepted.");
            }
        }
        let vertex_size = get_vertex_size(shader_id);
        let vertices_count = file.borrow_mut().read_count() as usize;
        let vertices_size = vertex_size * vertices_count;
        #[cfg(mesh_debug)]
        logi!("occlusion mesh with vertices size {}", vertices_size);
        let vertices_data = file.borrow_mut().read_bytes(vertices_size);
        let indices_count = file.borrow_mut().read_count() as usize;
        let indices_size = INDEX_ELEMENTS_SIZE * indices_count;
        let indices_data = file.borrow_mut().read_bytes(indices_size);
        let buffer_manager = engine.buffer_manager.as_ref().unwrap().clone();
        let buffer = buffer_manager.borrow_mut().create_mesh(
            vertex_size, vertices_count as usize, indices_count as usize
        );
        buffer.borrow_mut().upload_vertices(
            unsafe { transmute(vertices_data.as_ptr()) }, vertices_size);
        buffer.borrow_mut().upload_indices(
            unsafe { transmute(indices_data.as_ptr()) }, indices_size);
        OccMesh {
            buffer: buffer,
            buffer_manager: buffer_manager,
        }
    }
}
