pub mod manager;

use std::sync::Arc;
use std::cell::RefCell;
use super::super::math::matrix::Mat4x4;
use super::super::system::file::File;
use super::buffer::Manager as BufferManager;
use super::command::buffer::Buffer as CmdBuff;
use super::material::Material;
use super::shader::manager::Manager as ShaderManager;
use super::texture::manager::Manager as TextureManager;
use super::mesh::{Mesh, OccMesh};
use self::manager::Manager;

pub enum Dynamism {
    Dynamic,
    Static,
}

pub trait Model {
    fn compute_mvp(&mut self, vp: &Mat4x4<f32>);
    fn static_mvp(&mut self, vp: &Mat4x4<f32>, parent_mvp: &Mat4x4<f32>);
    fn get_dynamism(&self) -> Dynamism;
    fn occ(&mut self, cmd_buff: &mut CmdBuff, mat: &Arc<RefCell<Material>>, frame_index: usize);
}

pub struct StaticModel {
    pub mvp: Mat4x4<f32>,
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
            mvp: Mat4x4::ident(),
            draw_mesh: mesh,
            children: children,
        }
    }
}

impl Model for StaticModel {
    fn compute_mvp(&mut self, _vp: &Mat4x4<f32>) {
        logf!("This function must not be called because the static model mvp depends on its parent");
    }

    fn static_mvp(&mut self, vp: &Mat4x4<f32>, parent_mvp: &Mat4x4<f32>) {
        self.mvp = *parent_mvp;
        for m in &mut self.children {
            match m.get_dynamism() {
                Dynamism::Dynamic => m.compute_mvp(vp),
                Dynamism::Static => m.static_mvp(vp, parent_mvp),
            }
        }
    }

    fn get_dynamism(&self) -> Dynamism {
        Dynamism::Static
    }

    fn occ(&mut self, cmd_buff: &mut CmdBuff, mat: &Arc<RefCell<Material>>, frame_index: usize) {
        for m in &mut self.children {
            m.occ(cmd_buff, mat, frame_index);
        }
    }
}

pub struct RootStaticModel {
    pub occ_mesh: OccMesh,
    pub children: Vec<Box<Model>>,
    pub mvp: Mat4x4<f32>,
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
            mvp: Mat4x4::ident(),
        }
    }
}

impl Model for RootStaticModel {
    fn compute_mvp(&mut self, vp: &Mat4x4<f32>) {
        self.mvp = *vp;
        for m in &mut self.children {
            match m.get_dynamism() {
                Dynamism::Dynamic => m.compute_mvp(vp),
                Dynamism::Static => m.static_mvp(vp, &self.mvp)
            }
        }
    }

    fn static_mvp(&mut self, _vp: &Mat4x4<f32>, _parent_mvp: &Mat4x4<f32>) {
        logf!("Static root model mvp does not have parent.");
    }

    fn get_dynamism(&self) -> Dynamism {
        Dynamism::Static
    }

    fn occ(&mut self, cmd_buff: &mut CmdBuff, mat: &Arc<RefCell<Material>>, frame_index: usize) {
        
        for m in &mut self.children {
            m.occ(cmd_buff, mat, frame_index);
        }
    }
}

pub struct DynamicModel {
    pub mvp: Mat4x4<f32>,
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
            mvp: Mat4x4::ident(),
            transform: m,
            occ_mesh: mesh,
            children: children,
        }
    }
}

impl Model for DynamicModel {
    fn compute_mvp(&mut self, vp: &Mat4x4<f32>) {
        self.mvp = vp * &self.transform;
        for m in &mut self.children {
            match m.get_dynamism() {
                Dynamism::Dynamic => m.compute_mvp(vp),
                Dynamism::Static => m.static_mvp(vp, &self.mvp)
            }
        }
    }

    fn static_mvp(&mut self, _vp: &Mat4x4<f32>, _parent_mvp: &Mat4x4<f32>) {
        logf!("Dynamic model mvp does not depend on its parent.");
    }

    fn get_dynamism(&self) -> Dynamism {
        Dynamism::Dynamic
    }
}

pub struct CopyModel {
    pub mvp: Mat4x4<f32>,
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
        CopyModel {
            mvp: Mat4x4::ident(),
            t: t, 
            sm: sm 
        }
    }
}

impl Model for CopyModel {
    fn compute_mvp(&mut self, vp: &Mat4x4<f32>) {
        self.mvp = vp * &self.t;
        self.sm.borrow_mut().static_mvp(vp, &self.mvp);
    }

    fn static_mvp(&mut self, _vp: &Mat4x4<f32>, _parent_mvp: &Mat4x4<f32>) {
        logf!("Copy model mvp does not depend on its parent.");
    }

    fn get_dynamism(&self) -> Dynamism {
        Dynamism::Dynamic
    }
}

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
