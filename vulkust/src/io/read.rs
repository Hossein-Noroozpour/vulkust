use std::io::Read as StdRead;
use std::mem::{transmute, size_of};

pub trait Read: StdRead {
    fn is_endian_compatible(&self) -> bool;
    fn read_type<T>(&mut self) -> T where T: Default {
        let mut r = T::default();
        self.read_typed_bytes(unsafe {transmute(&mut r)}, size_of::<T>());
        r
    }
    fn read_typed_bytes(&mut self, des: *mut u8, count: usize) {
        let mut b = vec![0u8; count];
        if match self.read(&mut b) {
            Ok(c) => { c },
            Err(_) => { logf!("Error in reading stream."); },
        } < count {
            logf!("Expected bytes are not in stream.");
        }
        let b = b.as_ptr();
        if self.is_endian_compatible() {
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
}
