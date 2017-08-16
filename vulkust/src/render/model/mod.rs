pub mod manager;

use std::sync::Arc;
use std::cell::RefCell;
use super::super::math::matrix::Mat4x4;
use super::super::system::file::File;
use super::buffer::Manager as BufferManager;
use super::shader::manager::Manager as ShaderManager;
use super::texture::manager::Manager as TextureManager;
use super::mesh::{Mesh, OccMesh};
use self::manager::Manager;

pub trait Model {}

pub struct StaticModel {
    pub draw_mesh: Mesh,
    pub children: Vec<Box<Model>>,
}

impl StaticModel {
    pub fn new(
        file: &mut File,
        model_manager: &mut Manager,
        buffer_manager: &mut BufferManager,
        texture_manager: &mut TextureManager,
        shader_manager: &mut ShaderManager,
    ) -> Self {
        let device = buffer_manager.get_device().clone();
        let mesh = Mesh::new(
            file,
            buffer_manager,
            device,
            shader_manager,
            texture_manager,
        );
        let children_count: u64 = file.read_type();
        let mut children = Vec::new();
        for _ in 0..children_count {
            children.push(read_boxed_model(
                file,
                model_manager,
                buffer_manager,
                texture_manager,
                shader_manager,
            ));
        }
        StaticModel {
            draw_mesh: mesh,
            children: children,
        }
    }
}

impl Model for StaticModel {}

pub struct RootStaticModel {
    pub occ_mesh: OccMesh,
    pub children: Vec<Box<Model>>,
}

impl RootStaticModel {
    pub fn new(
        file: &mut File,
        model_manager: &mut Manager,
        buffer_manager: &mut BufferManager,
        texture_manager: &mut TextureManager,
        shader_manager: &mut ShaderManager,
    ) -> Self {
        let mesh = OccMesh::new(file, buffer_manager);
        let children_count = file.read_count();
        let mut children = Vec::new();
        for _ in 0..children_count {
            children.push(read_boxed_model(
                file,
                model_manager,
                buffer_manager,
                texture_manager,
                shader_manager,
            ));
        }
        RootStaticModel {
            occ_mesh: mesh,
            children: children,
        }
    }
}

impl Model for RootStaticModel {}

pub struct DynamicModel {
    pub transform: Mat4x4<f32>,
    pub occ_mesh: OccMesh,
    pub children: Vec<Box<Model>>,
}

impl DynamicModel {
    pub fn new(
        file: &mut File,
        model_manager: &mut Manager,
        buffer_manager: &mut BufferManager,
        texture_manager: &mut TextureManager,
        shader_manager: &mut ShaderManager,
    ) -> Self {
        let m = Mat4x4::new_from_file(file);
        let mesh = OccMesh::new(file, buffer_manager);
        let children_count: u64 = file.read_type();
        let mut children = Vec::new();
        for _ in 0..children_count {
            children.push(read_boxed_model(
                file,
                model_manager,
                buffer_manager,
                texture_manager,
                shader_manager,
            ));
        }
        DynamicModel {
            transform: m,
            occ_mesh: mesh,
            children: children,
        }
    }
}

impl Model for DynamicModel {}

pub struct CopyModel {
    pub t: Mat4x4<f32>,
    pub sm: Arc<RefCell<Model>>,
}

impl CopyModel {
    pub fn new(
        file: &mut File,
        model_manager: &mut Manager,
        buffer_manager: &mut BufferManager,
        texture_manager: &mut TextureManager,
        shader_manager: &mut ShaderManager,
    ) -> Self {
        let t = Mat4x4::new_from_file(file);
        let id = file.read_id();
        let offset = file.tell();
        let sm = model_manager.get(id, file, buffer_manager, texture_manager, shader_manager);
        file.goto(offset);
        CopyModel { t: t, sm: sm }
    }
}

impl Model for CopyModel {}

pub fn read_model(
    file: &mut File,
    model_manager: &mut Manager,
    buffer_manager: &mut BufferManager,
    texture_manager: &mut TextureManager,
    shader_manager: &mut ShaderManager,
) -> Arc<RefCell<Model>> {
    return if file.read_bool() {
        Arc::new(RefCell::new(CopyModel::new(
            file,
            model_manager,
            buffer_manager,
            texture_manager,
            shader_manager,
        )))
    } else if file.read_bool() {
        Arc::new(RefCell::new(DynamicModel::new(
            file,
            model_manager,
            buffer_manager,
            texture_manager,
            shader_manager,
        )))
    } else {
        Arc::new(RefCell::new(RootStaticModel::new(
            file,
            model_manager,
            buffer_manager,
            texture_manager,
            shader_manager,
        )))
    };
}

fn read_boxed_model(
    file: &mut File,
    model_manager: &mut Manager,
    buffer_manager: &mut BufferManager,
    texture_manager: &mut TextureManager,
    shader_manager: &mut ShaderManager,
) -> Box<Model> {
    return if file.read_bool() {
        Box::new(CopyModel::new(
            file,
            model_manager,
            buffer_manager,
            texture_manager,
            shader_manager,
        ))
    } else if file.read_bool() {
        Box::new(DynamicModel::new(
            file,
            model_manager,
            buffer_manager,
            texture_manager,
            shader_manager,
        ))
    } else {
        Box::new(StaticModel::new(
            file,
            model_manager,
            buffer_manager,
            texture_manager,
            shader_manager,
        ))
    };
}
