use std::sync::{Arc, RwLock, Weak};

pub fn align(size: isize, alignment: isize) -> isize {
    let tmp = size / alignment;
    let aligned_size = tmp * alignment;
    if aligned_size == size {
        return size;
    }
    aligned_size + alignment
}

pub trait Object {
    fn get_size(&self) -> isize;
    fn get_offset(&self) -> isize;
    fn get_offset_alignment(&self) -> isize;
    fn place(&mut self, offset: isize);
}

pub trait Allocator {
    fn increase_size(&mut self, size: isize);
    fn allocate(&mut self, offset_alignment: isize, obj: &Arc<RwLock<Object>>);
    fn clean(&mut self);
}

pub struct Memory {
    pub offset: isize,
    pub end: isize,
    pub size: isize,
    pub offset_alignment: isize,
}

impl Memory {
    pub fn new(size: isize, offset_alignment: isize) -> Self {
        Memory {
            offset: 0,
            end: size,
            size,
            offset_alignment,
        }
    }
}

impl Object for Memory {
    fn get_size(&self) -> isize {
        self.size
    }

    fn get_offset(&self) -> isize {
        self.offset
    }

    fn get_offset_alignment(&self) -> isize {
        self.offset_alignment
    }

    fn place(&mut self, offset: isize) {
        self.offset = offset;
        self.end = self.size + offset;
    }
}

pub struct Container {
    pub base: Memory,
    pub free_offset: isize,
    pub objects: Vec<Weak<RwLock<Object>>>,
}

impl Container {
    pub fn new(size: isize, offset_alignment: isize) -> Self {
        Container {
            base: Memory::new(size, offset_alignment),
            free_offset: 0,
            objects: Vec::new(),
        }
    }
}

impl Object for Container {
    fn get_size(&self) -> isize {
        self.base.size
    }

    fn get_offset(&self) -> isize {
        self.base.offset
    }

    fn get_offset_alignment(&self) -> isize {
        self.base.offset_alignment
    }

    fn place(&mut self, offset: isize) {
        self.base.place(offset);
        self.clean();
    }
}

impl Allocator for Container {
    fn increase_size(&mut self, size: isize) {
        self.base.end += size;
        self.base.size += size;
    }

    fn allocate(&mut self, offset_alignment: isize, obj: &Arc<RwLock<Object>>) {
        let obj_size = vxresult!(obj.read()).get_size();
        let offset = align(self.free_offset, offset_alignment);
        let free_offset = obj_size + offset;
        if free_offset > self.base.end {
            vxlogf!(
                "Out of space, {} offset_alignment: {} {} {} {}",
                "you probably forget to increase the size or cleaning the allocator.",
                offset_alignment, free_offset, self.base.size, obj_size
            );
        }
        vxresult!(obj.write()).place(offset);
        self.objects.push(Arc::downgrade(obj));
        self.free_offset = free_offset;
    }

    fn clean(&mut self) {
        let mut objects = Vec::new();
        self.free_offset = self.base.offset;
        for obj in &self.objects {
            if let Some(obj) = obj.upgrade() {
                let mut objm = vxresult!(obj.write());
                let size = objm.get_size();
                let offset = objm.get_offset();
                let aligned_offset = align(
                    self.free_offset, objm.get_offset_alignment());
                if aligned_offset != offset {
                    objm.place(aligned_offset);
                }
                objects.push(Arc::downgrade(&obj));
                self.free_offset = aligned_offset + size;
            }
        }
        self.objects = objects;
    }
}
