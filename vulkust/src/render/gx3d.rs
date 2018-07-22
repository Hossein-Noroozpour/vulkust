use super::super::core::object::NEXT_ID;
use super::super::core::types::{Id, Offset, TypeId};
use super::super::system::file::File;
use super::scene::Manager as SceneManager;
use std::collections::BTreeMap;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::mem::{size_of, transmute};
use std::ptr::copy;
use std::sync::atomic::Ordering;
use std::sync::{Arc, RwLock};

pub struct Gx3DReader {
    file: BufReader<File>,
    different_endianness: bool,
}

pub trait Readable: 'static + Sized + Default + Clone {}

impl Readable for u32 {}
impl Readable for u64 {}

impl Gx3DReader {
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
        Some(Gx3DReader {
            file,
            different_endianness,
        })
    }

    pub fn read_u8(&mut self) -> u8 {
        let mut d = [0u8; 1];
        #[cfg(debug_assertions)]
        {
            if 1 != vxresult!(self.file.read(&mut d)) {
                vxunexpected!();
            }
        }
        #[cfg(not(debug_assertions))]
        vxresult!(self.file.read(&mut d));
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

    #[cfg(not(debug_assertions))]
    pub fn seek(&mut self, offset: Offset) {
        vxresult!(self.file.seek(SeekFrom::Start(offset)));
    }

    #[cfg(debug_assertions)]
    pub fn seek(&mut self, offset: Offset) {
        if offset != vxresult!(self.file.seek(SeekFrom::Start(offset))) {
            vxunexpected!();
        }
    }
}

pub struct Table {
    pub reader: Gx3DReader,
    pub id_offset: BTreeMap<Id, Offset>,
}

impl Table {
    pub fn new(reader: &mut Gx3DReader) -> Self {
        let count = reader.read::<u64>() << 1;
        let mut id_offset = BTreeMap::new();
        for _ in 0..count {
            id_offset.insert(reader.read::<Id>(), reader.read::<Offset>());
        }
        Table {
            reader: vxunwrap_o!(Gx3DReader::new()),
            id_offset,
        }
    }

    pub fn goto(&mut self, id: Id) {
        let off = vxunwrap_o!(self.id_offset.get(&id));
        self.reader.seek(*off);
    }
}

pub fn import(scenemgr: &Arc<RwLock<SceneManager>>) {
    let main_file = Gx3DReader::new();
    if main_file.is_none() {
        return;
    }
    let mut main_file = vxunwrap_o!(main_file);
    let last_id: Id = main_file.read();
    NEXT_ID.store(last_id, Ordering::Relaxed);
    let mut scnmgr = vxresult!(scenemgr.write());
    macro_rules! set_table {
        ($mgr:ident) => {{
            let mut mgr = vxresult!(scnmgr.$mgr.write());
            let table = Table::new(&mut main_file);
            mgr.gx3d_table = Some(table);
        }};
    }
    set_table!(camera_manager);
    let _audio_table = Table::new(&mut main_file);
    set_table!(light_manager);
    set_table!(texture_manager);
    set_table!(font_manager);
    set_table!(mesh_manager);
    set_table!(model_manager);
    let _skybox_table = Table::new(&mut main_file);
    let _constraint_table = Table::new(&mut main_file);
    let table = Table::new(&mut main_file);
    scnmgr.gx3d_table = Some(table);
}
