pub mod manager;

use std::sync::Arc;
use std::default::Default;
use super::super::core::application::ApplicationTrait;
use super::super::math::matrix::Mat4x4;
use super::super::system::file::File;
use super::super::util::cell::DebugCell;
use super::buffer::Manager as BufferManager;
use super::command::buffer::Buffer as CmdBuff;
use super::engine::RenderEngine;
use super::material::Material;
use super::mesh::{Mesh, OccMesh};
use super::scene::UniformData as ScUniData;
use super::shader::manager::Manager as ShaderManager;
use super::texture::manager::Manager as TextureManager;
use self::manager::Manager;

#[derive(Default)]
pub struct UniformData {
    pub mvp: Mat4x4<f32>,
    pub m: Mat4x4<f32>,
}

pub enum Dynamism {
    Dynamic,
    Static,
}

pub trait Model {
    fn parent_update_uniform(&mut self, sud: &ScUniData, frame_index: usize) {
        logf!("This struct does not implement parent_update_uniform because it is not parent.");
    }
    fn child_update_uniform(&mut self, sud: &ScUniData, pud: &UniformData, frame_index: usize) {
        logf!("This struct does not implement child_update_uniform because it is not child.");
    }
    fn get_dynamism(&self) -> Dynamism;
    fn rec_occ(&mut self, cmd_buff: &mut CmdBuff, mat: &Arc<DebugCell<Material>>, frame_index: usize);
    fn is_static(&self) -> bool {
        return false;
    }
    fn to_static(&self) -> &StaticModel {
        logf!("This struct is not static.");
    }
    fn is_dynamic(&self) -> bool {
        return false;
    }
    fn to_dynamic(&self) -> &DynamicModel {
        logf!("This struct is not dynamic.");
    }
}

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
        shader_manager: &Arc<DebugCell<ShaderManager>>,
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

impl Model for StaticModel {
    fn child_update_uniform(&mut self, sud: &ScUniData, pud: &UniformData, frame_index: usize) {
        for &mut c in &mut self.children {
            c.child_update_uniform(sud, pud, frame_index);
        }
        self.draw_mesh.update_uniform(sud, pud, frame_index);
    }

    fn get_dynamism(&self) -> Dynamism {
        Dynamism::Static
    }

    fn rec_occ(&mut self, cmd_buff: &mut CmdBuff, mat: &Arc<DebugCell<Material>>, frame_index: usize) {
        for m in &mut self.children {
            m.rec_occ(cmd_buff, mat, frame_index);
        }
    }

    fn is_static(&self) -> bool {
        true
    }
    
    fn to_static(&self) -> &StaticModel {
        self
    }
}

pub struct RootStaticModel {
    pub occ_mesh: OccMesh,
    pub children: Vec<Box<Model>>,
    pub ud: UniformData,
}

impl RootStaticModel {
    pub fn new(
        file: &mut File,
        model_manager: &mut Manager,
        buffer_manager: &mut BufferManager,
        texture_manager: &mut TextureManager,
        shader_manager: &Arc<DebugCell<ShaderManager>>,
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
            ud: UniformData::default(),
        }
    }
}

impl Model for RootStaticModel {
    fn parent_update_uniform(&mut self, sud: &ScUniData, frame_index: usize) {
        self.ud.mvp = sud.vp;
        for &mut c in &mut self.children {
            c.child_update_uniform(sud, &self.ud, frame_index);
        }
        self.occ_mesh.update_uniform(&self.ud, frame_index);
    }

    fn child_update_uniform(&mut self, sud: &ScUniData, pud: &UniformData, frame_index: usize) {
        for &mut c in &mut self.children {
            c.child_update_uniform(sud, pud, frame_index);
        }
        self.occ_mesh.update_uniform(pud, frame_index);
    }

    fn get_dynamism(&self) -> Dynamism {
        Dynamism::Static
    }

    fn rec_occ(&mut self, cmd_buff: &mut CmdBuff, mat: &Arc<DebugCell<Material>>, frame_index: usize) {
        // todo;
        for m in &mut self.children {
            m.rec_occ(cmd_buff, mat, frame_index);
        }
    }
}

pub struct DynamicModel {
    pub ud: UniformData,
    pub occ_mesh: OccMesh,
    pub children: Vec<Box<Model>>,
}

impl DynamicModel {
    pub fn new(
        file: &mut File,
        model_manager: &mut Manager,
        buffer_manager: &mut BufferManager,
        texture_manager: &mut TextureManager,
        shader_manager: &Arc<DebugCell<ShaderManager>>,
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
        let mut ud = UniformData::default();
        ud.m = m;
        DynamicModel {
            ud: ud,
            occ_mesh: mesh,
            children: children,
        }
    }
}

impl Model for DynamicModel {
    fn parent_update_uniform(&mut self, sud: &ScUniData, frame_index: usize) {
        self.ud.mvp = &sud.vp * &self.ud.m;
        for &mut c in &mut self.children {
            c.child_update_uniform(sud, &self.ud, frame_index);
        }
        self.occ_mesh.update_uniform(&self.ud, frame_index);
    }

    fn child_update_uniform(&mut self, sud: &ScUniData, _pud: &UniformData, frame_index: usize) {
        self.parent_update_uniform(sud, frame_index);
    }

    fn get_dynamism(&self) -> Dynamism {
        Dynamism::Dynamic
    }

    fn rec_occ(&mut self, cmd_buff: &mut CmdBuff, mat: &Arc<DebugCell<Material>>, frame_index: usize) {
        // todo;
        for m in &mut self.children {
            m.rec_occ(cmd_buff, mat, frame_index);
        }
    }
}

pub struct CopyModel {
    pub ud: UniformData,
    pub sm: Arc<DebugCell<Model>>,
}

impl CopyModel {
    pub fn new(
        file: &mut File,
        model_manager: &mut Manager,
        buffer_manager: &mut BufferManager,
        texture_manager: &mut TextureManager,
        shader_manager: &Arc<DebugCell<ShaderManager>>,
    ) -> Self {
        let m = Mat4x4::new_from_file(file);
        let id = file.read_id();
        let offset = file.tell();
        let sm = model_manager.get(id, file, buffer_manager, texture_manager, shader_manager);
        file.goto(offset);
        let mut ud = UniformData::default();
        ud.m = m;
        CopyModel {
            ud: ud, 
            sm: sm 
        }
    }
}

impl Model for CopyModel {
    fn parent_update_uniform(&mut self, sud: &ScUniData, frame_index: usize) {
        self.ud.mvp = &sud.vp * &self.ud.m;
        self.sm.borrow_mut().child_update_uniform(sud, &self.ud, frame_index);
    }

    fn child_update_uniform(&mut self, sud: &ScUniData, _pud: &UniformData, frame_index: usize) {
        self.parent_update_uniform(sud, frame_index);
    }

    fn get_dynamism(&self) -> Dynamism {
        Dynamism::Dynamic
    }

    fn rec_occ(&mut self, cmd_buff: &mut CmdBuff, mat: &Arc<DebugCell<Material>>, frame_index: usize) {
        self.sm.borrow_mut().rec_occ(cmd_buff, mat, frame_index);
    }
}

pub fn read_model(
    file: &mut File,
    model_manager: &mut Manager,
    buffer_manager: &mut BufferManager,
    texture_manager: &mut TextureManager,
    shader_manager: &Arc<DebugCell<ShaderManager>>,
) -> Arc<DebugCell<Model>> {
    return if file.read_bool() {
        Arc::new(DebugCell::new(CopyModel::new(
            file,
            model_manager,
            buffer_manager,
            texture_manager,
            shader_manager,
        )))
    } else if file.read_bool() {
        Arc::new(DebugCell::new(DynamicModel::new(
            file,
            model_manager,
            buffer_manager,
            texture_manager,
            shader_manager,
        )))
    } else {
        Arc::new(DebugCell::new(RootStaticModel::new(
            file,
            model_manager,
            buffer_manager,
            texture_manager,
            shader_manager,
        )))
    };
}

fn read_boxed_model<CoreApp>(
    file: &Arc<DebugCell<File>>,
    engine: &mut RenderEngine<CoreApp>
) -> Box<Model> 
where CoreApp: ApplicationTrait {
    return if file.read_bool() {
        Box::new(CopyModel::new(file, engine))
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
