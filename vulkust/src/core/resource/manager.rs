use std::sync::{
    Arc,
    Mutex,
};
use super::Resource;

pub trait Manager {
    fn read_tabale(&mut self);
    fn get_resource(&mut self, id: u64) -> Arc<Resource>;
}
