use super::super::system::file::File;
use super::scene::Manager as SceneManager;
use std::io::{BufReader, Read};
use std::mem::{size_of, transmute};
use std::ptr::copy;
use std::sync::{Arc, RwLock};

pub struct Gx3D {
    file: BufReader<File>,
    different_endianness: bool,
}

pub trait Readable: 'static + Sized + Default + Clone {}

impl Readable for u8 {}
impl Readable for u32 {}
impl Readable for u64 {}

impl Gx3D {
    pub fn new() -> Option<Self> {
        let file = File::open("data.gx3d");
        if file.is_err() {
            return None;
        }
        let file = vxresult!(file);
        let mut file = BufReader::new(file);
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

    fn read_typed_bytes(&mut self, dest: *mut u8, count: usize) {
        let mut bytes = vec![0u8; count];
        let _n = vxresult!(self.file.read(&mut bytes));
        #[cfg(debug_assertions)]
        {
            if _n != count {
                vxunexpected!();
            }
        }
        if self.different_endianness {
            let mut end = count - 1;
            let mut start = 0;
            while start < end {
                let tmp = bytes[start];
                bytes[start] = bytes[end];
                bytes[end] = tmp;
                start += 1;
                end -= 1;
            }
        }
        unsafe {
            copy(bytes.as_ptr(), dest, count);
        }
    }

    fn read_array_typed_bytes(&mut self, dest: *mut u8, esize: usize, count: usize) {
        let size = esize * count;
        let mut bytes = vec![0u8; size];
        let _n = vxresult!(self.file.read(&mut bytes));
        #[cfg(debug_assertions)]
        {
            if _n != size {
                vxunexpected!();
            }
        }
        if self.different_endianness {
            let inc = esize - 1;
            let mut starte = 0;
            let mut ende = inc;
            for _ in 0..count {
                let mut end = ende;
                let mut start = starte;
                starte += esize;
                ende += esize;
                while start < end {
                    let tmp = bytes[start];
                    bytes[start] = bytes[end];
                    bytes[end] = tmp;
                    start += 1;
                    end -= 1;
                }
            }
        }
        unsafe {
            copy(bytes.as_ptr(), dest, size);
        }
    }

    pub fn read<T>(&mut self) -> T
    where
        T: Readable,
    {
        let mut t = T::default();
        let buff: *mut u8 = unsafe { transmute(&mut t) };
        self.read_typed_bytes(buff, size_of::<T>());
        return t;
    }

    pub fn read_array<T>(&mut self) -> Vec<T>
    where
        T: Readable,
    {
        let count = self.read::<u64>() as usize;
        let mut ts = vec![T::default(); count];
        self.read_array_typed_bytes(unsafe { transmute(ts.as_mut_ptr()) }, size_of::<T>(), count);
        return ts;
    }
}

pub fn import(scenemgr: &Arc<RwLock<SceneManager>>) {}
