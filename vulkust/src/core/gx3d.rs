use super::super::system::file::File;
use super::config::Configurations;
use super::types::{Id, Offset, Size, TypeId};
use std::collections::BTreeMap;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::mem::{size_of, transmute};
use std::ptr::copy;

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Gx3DReader {
    file: BufReader<File>,
    different_endianness: bool,
}

pub trait Readable: 'static + Sized + Default + Clone {}

impl Readable for f32 {}
impl Readable for u32 {}
impl Readable for u64 {}

impl Gx3DReader {
    pub(super) fn new(name: &str) -> Option<Self> {
        let file = File::open(name);
        if file.is_err() {
            return None;
        }
        let file = vx_result!(file);
        let mut file = BufReader::new(file);
        let mut endian = [0u8; 1];
        vx_result!(file.read(&mut endian));
        #[cfg(target_endian = "little")]
        let different_endianness = endian[0] == 0;
        #[cfg(target_endian = "big")]
        let different_endianness = endian[0] != 0;
        Some(Gx3DReader {
            file,
            different_endianness,
        })
    }

    pub fn read_u8(&mut self) -> u8 {
        let mut d = [0u8; 1];
        #[cfg(debug_mode)]
        {
            if 1 != vx_result!(self.file.read(&mut d)) {
                vx_unexpected!();
            }
        }
        #[cfg(not(debug_mode))]
        vx_result!(self.file.read(&mut d));
        return d[0];
    }

    pub fn read_bool(&mut self) -> bool {
        self.read_u8() != 0
    }

    pub fn read_type_id(&mut self) -> TypeId {
        self.read_u8()
    }

    fn read_typed_bytes(&mut self, dest: *mut u8, count: usize) {
        let mut bytes = vec![0u8; count];
        let mut n = vx_result!(self.file.read(&mut bytes));
        let mut readcount = n;
        while readcount < count {
            if n < 1 {
                vx_unexpected!();
            }
            n = vx_result!(self.file.read(&mut bytes[readcount..count]));
            readcount += n;
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
        let mut n = vx_result!(self.file.read(&mut bytes));
        let mut readsize = n;
        while readsize < size {
            if n < 1 {
                vx_unexpected!();
            }
            n = vx_result!(self.file.read(&mut bytes[readsize..size]));
            readsize += n;
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

    #[cfg(not(debug_mode))]
    pub fn seek(&mut self, offset: Offset) {
        vx_result!(self.file.seek(SeekFrom::Start(offset)));
    }

    #[cfg(debug_mode)]
    pub fn seek(&mut self, offset: Offset) {
        if offset != vx_result!(self.file.seek(SeekFrom::Start(offset))) {
            vx_unexpected!();
        }
    }

    pub fn read_bytes(&mut self, count: Size) -> Vec<u8> {
        let mut data = vec![0u8; count as usize];
        let _n = vx_result!(self.file.read(&mut data));
        #[cfg(debug_mode)]
        {
            if _n as Size != count {
                vx_unexpected!();
            }
        }
        return data;
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct Table {
    reader: Gx3DReader,
    id_offset: BTreeMap<Id, Offset>,
}

impl Table {
    pub(super) fn new(reader: &mut Gx3DReader, config: &Configurations) -> Self {
        let count = reader.read::<u64>();
        let mut id_offset = BTreeMap::new();
        for _ in 0..count {
            id_offset.insert(reader.read::<Id>(), reader.read::<Offset>());
        }
        Table {
            reader: vx_unwrap!(Gx3DReader::new(config.get_gx3d_file_name())),
            id_offset,
        }
    }

    pub fn goto(&mut self, id: Id) {
        let off = vx_unwrap!(self.id_offset.get(&id));
        self.reader.seek(*off);
    }

    pub fn get_mut_reader(&mut self) -> &mut Gx3DReader {
        return &mut self.reader;
    }
}
