use std::cell::RefCell;
use std::sync::{Arc, Weak};
use super::list::{List, ListNode};

pub trait GcObject {
    fn get_size(&self) -> usize;
    fn allocate(&mut self, offset: usize);
    fn move_to(&mut self, offset: usize);
} 

struct MemInfo {
    front: usize,
    end: usize,
    size: usize,
    pointer: Weak<RefCell<GcObject>>,
}

pub struct Gc {
    size: usize,
    last_offset: usize,
    last_checked: Option<&'static mut ListNode<MemInfo>>,
    objects_count: usize,
    objects: List<MemInfo>,
}

impl Gc {
    pub fn new(size: usize) -> Self {
        Gc {
            size: size,
            last_offset: 0,
            last_checked: None,
            objects_count: 0,
            objects: List::new(),
        }
    }

    pub fn clean(&mut self) {
        self.last_checked = None;
        let mut obj = self.objects.get_front();
        let mut last_offset = 0;
        while obj.is_some() {
            let o = obj.unwrap();
            obj = match o.data.pointer.upgrade() {
                Some(op) => {        
                    if o.data.front != last_offset {
                        op.borrow_mut().move_to(last_offset);
                        o.data.front = last_offset;
                        o.data.end = last_offset + o.data.size;
                    }
                    last_offset = o.data.end;
                    o.get_child()
                },
                None => {
                    self.objects_count -= 1;
                    o.remove()
                },
            };
        }
        
    }

    pub fn allocate(&mut self, object: &Arc<RefCell<GcObject>>) {
        let obj_size = object.borrow().get_size();
        self.objects_count += 1;
        if self.size < obj_size {
            logf!("The Object you want to allocate is bigger than GC memory!");
        }
        if self.size - self.last_offset >= obj_size {
            object.borrow_mut().allocate(self.last_offset);
            self.objects.add_end(
                MemInfo {
                    front: self.last_offset,
                    end: self.last_offset + obj_size,
                    size: obj_size,
                    pointer: Arc::downgrade(object),
                }
            );
            self.last_checked = None;
            self.last_offset += obj_size;
            return;
        }
        if self.last_checked.is_none() {
            self.last_checked = self.objects.get_front();
        }
        let mut last_checked = self.last_checked.as_ref().unwrap();
        let mut offset_free = last_checked.data.front;
        let obj_count = self.objects_count;
        for _ in 0..obj_count {
            let obj = last_checked.data.pointer.upgrade();
            self.last_checked = match obj {
                Some(o) => {
                    let offset_free_end = last_checked.data.front;
                    if offset_free_end - offset_free >= obj_size {
                        object.borrow_mut().allocate(offset_free);
                        last_checked.add_parent(
                            MemInfo {
                                front: offset_free,
                                end: offset_free + obj_size,
                                size: obj_size,
                                pointer: Arc::downgrade(object),
                            }
                        );
                        self.last_checked = last_checked.get_child();
                        return;
                    }
                    offset_free = last_checked.data.end;
                    last_checked.get_child()
                },
                None => {
                    self.objects_count -= 1;
                    last_checked.remove()
                },
            };
            if self.last_checked.is_none() {
                if self.size - offset_free >= obj_size {
                    object.borrow_mut().allocate(offset_free);
                    last_checked.add_parent(
                        MemInfo {
                            front: offset_free,
                            end: offset_free + obj_size,
                            size: obj_size,
                            pointer: Arc::downgrade(object),
                        }
                    );
                    self.last_offset = offset_free + obj_size;
                    return;
                }
                self.last_checked = self.objects.get_front();
            }
            last_checked = self.last_checked.as_ref().unwrap();
        }
        loge!("Performance warning, GC called automatically, please do gc cleaning manually for preventing lag in game.");
        self.clean();
        if self.size - self.last_offset < obj_size {
            logf!("Out of GC memory!");
        }
        object.borrow_mut().allocate(self.last_offset);
        self.objects.add_end(
            MemInfo {
                front: self.last_offset,
                end: self.last_offset + obj_size,
                size: obj_size,
                pointer: Arc::downgrade(object),
            }
        );
        self.last_offset += obj_size;
    }
}