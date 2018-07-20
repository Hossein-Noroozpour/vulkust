use super::scene::Manager as SceneManager;
use super::super::system::file::File;
use std::io::Read;
use std::sync::{Arc, RwLock};

pub struct Gx3D {
    file: File,
    different_endianness: bool,
}

impl Gx3D {
    pub fn new() -> Option<Self> {
        let file = File::open("data.gx3d");
        if file.is_err() {
            return None;
        }
        let mut file = vxresult!(file);
        let mut endian = [0u8; 1];
        vxresult!(file.read(&mut endian));
        #[cfg(target_endian = "little")]
        let different_endianness = endian[0] == 0;
        #[cfg(target_endian = "big")]
        let different_endianness = endian[0] != 0;
        Some(Gx3D {
            file,
            different_endianness,
        })
    }

    #[cfg(debug_assertions)]
    fn read_typed_bytes(&mut self, bytes: &mut [u8]) {
        let n = vxresult!(self.file.read(bytes));
        if n != bytes.len() {
            vxunexpected!();
        }
        if self.different_endianness {
            let mut end = bytes.len() - 1;
            let mut start = 0;
            while start < end {
                let tmp = bytes[start];
                bytes[start] = bytes[end];
                bytes[end] = tmp;
                start += 1;
                end -= 1;
            }
        }
    }

    #[cfg(not(debug_assertions))]
    fn read_typed_bytes(&mut self, bytes: &mut [u8]) {
        vxresult!(self.file.read(bytes));
        if self.different_endianness {
            let mut end = bytes.len() - 1;
            let mut start = 0;
            while start < end {
                let tmp = bytes[start];
                bytes[start] = bytes[end];
                bytes[end] = tmp;
                start += 1;
                end -= 1;
            }
        }
    }

    pub fn read_id() -> 
} 

pub fn import(scenemgr: &Arc<RwLock<SceneManager>>) {

}