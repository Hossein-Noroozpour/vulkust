use std::fs::File as StdFile;
use std::io::{
    BufReader,
    Read as StdRead,
    Result,
};
use std::mem::{
    transmute,
    size_of,
};
use std::slice::from_raw_parts_mut;
use super::read::Read;

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

    // pub fn read_bool(&mut self) -> bool {
    //     let mut byte = [0; 1];
    //     match self.reader.read(&mut byte) {
    //         Ok(size) => {
    //             if size != 1 {
    //                 logf!("Error read size is not equal to size of bool.");
    //             }
    //         }
    //         Err(e) => {
    //             logf!("Error {:?} in reading file.", e);
    //         }
    //     }
    //     if byte[0] == 0 {
    //         false
    //     } else {
    //         true
    //     }
    // }
    //
    // pub fn read<T>(&mut self, default: &T) -> T where T: Sized + Copy {
    //     let t_size = size_of::<T>();
    //     let mut t = *default;
    //     unsafe {
    //         let u = transmute::<*mut T, *mut u8>(&mut t);
    //         let bytes = from_raw_parts_mut(u, t_size);
    //         match self.reader.read(bytes) {
    //             Ok(size) => {
    //                 if size != t_size {
    //                     logf!("Error read size is not equal to size of bool.");
    //                 }
    //             }
    //             Err(e) => {
    //                 logf!("Error {:?} in reading file.", e);
    //             }
    //         }
    //         if !self.endian_compatible {
    //             for i in 0..(t_size / 2) {
    //                 let ci = t_size - (i + 1);
    //                 let tmp = (*bytes)[ci];
    //                 (*bytes)[ci] = (*bytes)[i];
    //                 (*bytes)[i] = tmp;
    //             }
    //         }
    //     }
    //     t
    // }
    //
    // pub fn read_string(&mut self) -> String {
    //     let string_length = self.read(&0u16);
    //     let mut s = String::from("");
    //     for _ in 0..string_length {
    //         let c = (self.read(&0u8) as char).to_string();
    //         s = s + &c;
    //     }
    //     s
    // }
}

impl StdRead for File {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.reader.read(buf)
    }
}

impl Read for File {
    fn is_endian_compatible(&self) -> bool {
        return self.endian_compatible;
    }
}
