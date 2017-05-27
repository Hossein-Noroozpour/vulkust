use std::io::Read as StdRead;
use std::mem::{transmute, size_of};

pub trait Read: StdRead {
    fn is_endian_compatible(&self) -> bool;
    fn read_type<T>(&mut self) -> T {
        let mut r: T = 0;
        self.read_typed_bytes(unsafe {transmute(&mut r)}, size_of::<T>());
        r
    }
    fn read_typed_bytes(&mut self, des: *mut u8, count: isize) {
        let mut b = vec![0u8; count];
        self.read(&b);
        let b = b.as_ptr();
        if self.is_endian_compatible() {
            for i in 0..count {
                unsafe {
                    *des.offset(i) = *b.offset(i);
                }
            }
        } else {
            let mut i = 0isize;
            let mut j = count - 1;
            while i < count {
                unsafe {
                    *des.offset(i) = *b.offset(j);
                }
                i += 1;
                j -= 1;
            }
        }
    }
}
