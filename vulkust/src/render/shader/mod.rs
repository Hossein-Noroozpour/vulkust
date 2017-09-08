pub mod manager;
pub mod stage;

use std::sync::Arc;
use std::mem::size_of;
use super::super::system::file::File;
use super::super::util::cell::DebugCell;
use super::device::logical::Logical as LogicalDevice;
use super::vertex::Attribute as VertexAttribute;
use super::material;
use self::stage::Stage;

pub const WHITE_ID: Id = 0;
pub const DIRECTIONAL_TEXTURED_SPECULATED_NOCUBE_FULLSHADOW_OPAQUE_ID: Id = 2207629967616;

pub trait Shader {
    fn as_two_stage(&self) -> &TwoStage {
        logf!("This object can not convert to TwoStage Shader.");
    }
    fn get_stages_count(&self) -> usize;
    fn get_stage(&self, i: usize) -> &Stage;
    fn get_vertex_stage(&self) -> &Stage;
    fn get_fragment_stage(&self) -> &Stage;
}

pub struct TwoStage {
    pub vertex: Stage,
    pub fragment: Stage,
}

impl TwoStage {
    pub fn new(file: &Arc<DebugCell<File>>, logical_device: Arc<LogicalDevice>) -> Self {
        let size: u64 = file.borrow_mut().read_type();
        // logi!("shader size is: {}", size);
        let vertex = file.borrow_mut().read_bytes(size as usize);
        let size: u64 = file.borrow_mut().read_type();
        // logi!("shader size is: {}", size);
        let fragment = file.borrow_mut().read_bytes(size as usize);
        TwoStage {
            vertex: Stage::new(vertex, logical_device.clone()),
            fragment: Stage::new(fragment, logical_device),
        }
    }
}

impl Shader for TwoStage {
    fn as_two_stage(&self) -> &TwoStage {
        return self;
    }

    fn get_stages_count(&self) -> usize {
        2
    }

    fn get_stage(&self, i: usize) -> &Stage {
        match i {
            0 => &self.vertex,
            1 => &self.fragment,
            c @ _ => {
                logf!("TwoStage does not have stage number {}", c);
            }
        }
    }

    fn get_vertex_stage(&self) -> &Stage {
        &self.vertex
    }

    fn get_fragment_stage(&self) -> &Stage {
        &self.fragment
    }
}

pub type Id = u64;

pub fn from_gx3d_id(v: Vec<u8>) -> Id {
    let mut id = 0;
    for b in v {
        id <<= 8;
        id |= b as Id;
    }
    return id;
}

pub const ID_BYTES_COUNT: usize = 6;

pub fn read_id(file: &Arc<DebugCell<File>>) -> u64 {
    from_gx3d_id(file.borrow_mut().read_bytes(ID_BYTES_COUNT))
}

pub fn get_vertex_size(sid: Id) -> usize {
    match sid {
        WHITE_ID => 12,
        DIRECTIONAL_TEXTURED_SPECULATED_NOCUBE_FULLSHADOW_OPAQUE_ID => 32,
        id @ _ => { logf!("The Shader Id {} is not expected.", id); },
    }
}

pub fn shader_id_to_vertex_attributes(id: Id) -> Vec<VertexAttribute> {
    match id {
        WHITE_ID => vec![VertexAttribute::Vec3F32],
        DIRECTIONAL_TEXTURED_SPECULATED_NOCUBE_FULLSHADOW_OPAQUE_ID =>
            vec![
                VertexAttribute::Vec3F32, 
                VertexAttribute::Vec3F32,
                VertexAttribute::Vec2F32
            ],
        id @ _ => { logf!("The Shader Id {} is not expected.", id); },
    }
}

pub enum BindingStage {
    Vertex,
    Fragment,
}

pub enum ResourceType {
    Uniform,
}

pub fn shader_id_resources(id: Id) -> Vec<(Vec<BindingStage>, u32, ResourceType)> {
    match id {
        WHITE_ID => 
            vec![(vec![BindingStage::Vertex], 1, ResourceType::Uniform)],
        DIRECTIONAL_TEXTURED_SPECULATED_NOCUBE_FULLSHADOW_OPAQUE_ID => {
            loge!("this is temporary");
            vec![(vec![BindingStage::Vertex], 1, ResourceType::Uniform)]
        }, // ONLY FOR NOW 
        id @ _ => { logf!("The Shader Id {} is not expected.", id); },
    }
}

pub fn shader_uniform_size(id: Id) -> usize {
    match id {
        WHITE_ID => 
            size_of::<material::WhiteUniform>(),
        DIRECTIONAL_TEXTURED_SPECULATED_NOCUBE_FULLSHADOW_OPAQUE_ID => 
            size_of::<material::DirectionalTexturedSpeculatedNocubeFullshadowOpaqueUniform>(),
        id @ _ => { logf!("The Shader Id {} is not expected.", id); },
    }
}