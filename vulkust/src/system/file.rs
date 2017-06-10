use std::fs::File as StdFile;
use std::io::{BufReader, Read, Seek, SeekFrom, Result};
use std::mem::{transmute, size_of};
use std::slice::from_raw_parts_mut;

#[derive(Debug)]
pub struct File {
    pub endian_compatible: bool,
    pub reader: BufReader<StdFile>,
}

impl File {
    #[cfg(target_endian = "big")]
    fn check_endian(&mut self) {
        if self.read_bool() {
            self.endian_compatible = false;
        } else {
            self.endian_compatible = true;
        }
    }

    #[cfg(target_endian = "little")]
    fn check_endian(&mut self) {
        if self.read_bool() {
            self.endian_compatible = true;
        } else {
            self.endian_compatible = false;
        }
    }

    pub fn new(file_name: &String) -> Self {
        match StdFile::open(file_name) {
            Ok(f) => {
                let mut s = File {
                    endian_compatible: false,
                    reader: BufReader::new(f)
                };
                s.check_endian();
                s
            }
            Err(e) => {
                logf!("Error {:?} in file reading.", e);
            }
        }
    }

    pub fn read_typed_bytes(&mut self, des: *mut u8, count: usize) {
        let mut b = self.read_bytes(count);
        let b = b.as_ptr();
        if self.endian_compatible {
            for i in 0..count {
                unsafe {
                    *des.offset(i as isize) = *b.offset(i as isize);
                }
            }
        } else {
            let mut i = 0usize;
            let mut j = count - 1;
            while i < count {
                unsafe {
                    *des.offset(i as isize) = *b.offset(j as isize);
                }
                i += 1;
                j -= 1;
            }
        }
    }

    pub fn read_bytes(&mut self, count: usize) -> Vec<u8> {
        let mut b = vec![0u8; count];
        if match self.read(&mut b) {
            Ok(c) => { c },
            Err(_) => { logf!("Error in reading stream."); },
        } < count {
            logf!("Expected bytes are not in stream.");
        }
        return b;
    }

    pub fn read_bool(&mut self) -> bool {
        let mut b = self.read_bytes(1);
        if b[0] == 1 {
            return true;
        }
        return false;
    }

    pub fn read_type<T>(&mut self) -> T where T: Default {
        let mut r = T::default();
        self.read_typed_bytes(unsafe {transmute(&mut r)}, size_of::<T>());
        r
    }
}

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.reader.read(buf)
    }
}

impl Seek for File {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        self.reader.seek(pos)
    }
}
