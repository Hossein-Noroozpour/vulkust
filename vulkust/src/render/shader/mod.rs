pub mod manager;
pub mod stage;

use std::sync::Arc;
use super::super::system::file::File;
use super::device::logical::Logical as LogicalDevice;
use self::stage::Stage;

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
    pub fn new(file: &mut File, logical_device: Arc<LogicalDevice>) -> Self {
        let size: u64 = file.read_type();
        // logi!("shader size is: {}", size);
        let vertex = file.read_bytes(size as usize);
        let size: u64 = file.read_type();
        // logi!("shader size is: {}", size);
        let fragment = file.read_bytes(size as usize);
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

pub fn read_id(file: &mut File) -> u64 {
    from_gx3d_id(file.read_bytes(ID_BYTES_COUNT))
}
