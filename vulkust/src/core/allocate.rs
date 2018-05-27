use std::sync::{Arc, RwLock, Weak};

pub fn align(size: isize, alignment: isize) -> isize {
    let tmp = size / alignment;
    if tmp * alignment == size {
        return size;
    }
    (tmp + 1) * alignment
}

pub trait Object {
    fn size(&self) -> isize;
    fn offset(&self) -> isize;
    fn place(&mut self, offset: isize);
}

pub trait Allocator {
    fn increase_size(&mut self, size: isize);
    fn allocate(&mut self, obj: &Arc<RwLock<Object>>);
    fn clean(&mut self);
}

pub struct Memory {
    pub offset: isize,
    pub end: isize,
    pub size: isize,
}

impl Memory {
    pub fn new(size: isize) -> Self {
        Memory {
            offset: 0,
            end: size,
            size,
        }
    }
}

impl Object for Memory {
    fn size(&self) -> isize {
        self.size
    }

    fn offset(&self) -> isize {
        self.offset
    }

    fn place(&mut self, offset: isize) {
        self.offset = offset;
        self.end = self.size + offset;
    }
}

pub struct Container {
    pub offset: isize,
    pub end: isize,
    pub size: isize,
    pub free_offset: isize,
    pub free_space: isize,
    pub objects: Vec<Weak<RwLock<Object>>>,
}

impl Container {
    pub fn new(size: isize) -> Self {
        Container {
            offset: 0,
            end: size,
            size,
            free_offset: 0,
            free_space: size,
            objects: Vec::new(),
        }
    }
}

impl Object for Container {
    fn size(&self) -> isize {
        self.size
    }

    fn offset(&self) -> isize {
        self.offset
    }

    fn place(&mut self, offset: isize) {
        self.offset = offset;
        self.clean();
    }
}

impl Allocator for Container {
    fn increase_size(&mut self, size: isize) {
        self.end += size;
        self.size += size;
        self.free_space += size;
    }

    fn allocate(&mut self, obj: &Arc<RwLock<Object>>) {
        let obj_size = vxresult!(obj.read()).size();
        if obj_size > self.free_space {
            vxlogf!(
                "Out of space, you probably forget to increase the size or cleaning the allocator."
            );
        }
        vxresult!(obj.write()).place(self.free_offset);
        self.objects.push(Arc::downgrade(obj));
        self.free_offset += obj_size;
        self.free_space -= obj_size;
    }

    fn clean(&mut self) {
        let mut objects = Vec::new();
        self.free_offset = self.offset;
        self.free_space = self.size;
        for obj in &self.objects {
            match obj.upgrade() {
                Some(obj) => {
                    let size = vxresult!(obj.read()).size();
                    let offset = vxresult!(obj.read()).offset();
                    if offset != self.free_offset {
                        vxresult!(obj.write()).place(self.free_offset);
                    }
                    objects.push(Arc::downgrade(&obj));
                    self.free_offset += size;
                    self.free_space -= size;
                }
                None => continue,
            }
        }
        self.objects = objects;
    }
}
