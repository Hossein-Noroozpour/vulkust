use super::super::super::io::read::Read;
use std::sync::{
    Arc,
    Mutex,
};
use super::Resource;

pub trait Manager<File> where File: Read {
    fn read_tabale(&mut self, file: Arc<Mutex<File>>);
    fn get_resource(&mut self, id: u64) -> Arc<Resource>;
}
