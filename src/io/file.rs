use std::fs::File;
use std::io::{
    BufReader,
    Read,
};
use std::mem::{
    transmute,
    size_of,
};
use std::slice::from_raw_parts_mut;

pub struct Stream {
    pub not_same_endianness: bool,
    pub reader: BufReader<File>,
}

impl Stream {

    #[cfg(target_endian = "big")]
    fn check_endian(&mut self) {
        if self.read_bool() {
            self.not_same_endianness = true;
        } else {
            self.not_same_endianness = false;
        }
    }

    #[cfg(target_endian = "little")]
    fn check_endian(&mut self) {
        if self.read_bool() {
            self.not_same_endianness = false;
        } else {
            self.not_same_endianness = true;
        }
    }

    pub fn new(file_name: &String) -> Stream {
        match File::open(file_name) {
            Ok(f) => {
                let mut s = Stream {not_same_endianness: false, reader: BufReader::new(f)};
                s.check_endian();
                s
            }
            Err(e) => {
                println!("{:?}", e);
                panic!("Error in file reading.");
            }
        }
    }

    pub fn read_bool(&mut self) -> bool {
        let mut byte = [0; 1];
        match self.reader.read(&mut byte) {
            Ok(size) => {
                if size != 1 {
                    panic!("Error read size is not equal to size of bool.");
                }
            }
            Err(e) => {
                println!("{:?}", e);
                panic!("Error in reading file.");
            }
        }
        if byte[0] == 0 {
            false
        } else {
            true
        }
    }

    pub fn read<T>(&mut self, default: &T) -> T where T: Sized + Copy {
        let t_size = size_of::<T>();
        let mut t = *default;
        unsafe {
            let u = transmute::<*mut T, *mut u8>(&mut t);
            let bytes = from_raw_parts_mut(u, t_size);
            match self.reader.read(bytes) {
                Ok(size) => {
                    if size != t_size {
                        panic!("Error read size is not equal to size of bool.");
                    }
                }
                Err(e) => {
                    println!("{:?}", e);
                    panic!("Error in reading file.");
                }
            }
            if self.not_same_endianness {
                for i in 0..(t_size / 2) {
                    let ci = t_size - (i + 1);
                    let tmp = (*bytes)[ci];
                    (*bytes)[ci] = (*bytes)[i];
                    (*bytes)[i] = tmp;
                }
            }
        }
        t
    }

    pub fn read_string(&mut self) -> String {
        let string_length = self.read(&0u16);
        let mut s = String::from("");
        for _ in 0..string_length {
            let c = (self.read(&0u8) as char).to_string();
            s = s + &c;
        }
        s
    }
}
