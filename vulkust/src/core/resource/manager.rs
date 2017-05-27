use std::io::Read;
use std::sync::Arc;
use super::Resource;

pub trait Manager {
    fn read_tabale(&mut self, file: &mut Read);
    fn get_resource(&mut self, id: u64) -> Arc<Resource>;
}
