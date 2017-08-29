use std::sync::{Arc, Weak};
use super::list::{List, ListNode};
use super::cell::DebugCell;

pub trait GcObject {
    fn get_size(&self) -> usize;
    fn move_to(&mut self, offset: usize);
} 

struct MemInfo {
    front: usize,
    end: usize,
    size: usize,
    pointer: Weak<DebugCell<GcObject>>,
}

pub struct Gc {
    front: usize,
    end: usize,
    size: usize,
    last_offset: usize,
    last_checked: Option<&'static mut ListNode<MemInfo>>,
    objects_count: usize,
    objects: List<MemInfo>,
}

impl Gc {
    pub fn new(front: usize, size: usize) -> Self {
        Gc {
            front: front,
            end: front + size,
            size: size,
            last_offset: front,
            last_checked: None,
            objects_count: 0,
            objects: List::new(),
        }
    }

    pub fn clean(&mut self) {
        self.last_checked = None;
        let mut obj = self.objects.get_front();
        self.last_offset = self.front;
        while obj.is_some() {
            let o = obj.unwrap();
            obj = match o.data.pointer.upgrade() {
                Some(op) => {        
                    if o.data.front != self.last_offset {
                        op.borrow_mut().move_to(self.last_offset);
                        o.data.front = self.last_offset;
                        o.data.end = self.last_offset + o.data.size;
                    }
                    self.last_offset = o.data.end;
                    o.get_child()
                },
                None => {
                    self.objects_count -= 1;
                    o.remove()
                },
            };
        }
        
    }

    pub fn allocate(&mut self, object: &Arc<DebugCell<GcObject>>) {
        let obj_size = object.borrow().get_size();
        self.objects_count += 1;
        if self.size < obj_size {
            logf!("The Object you want to allocate is bigger than GC memory!");
        }
        if self.end - self.last_offset >= obj_size {
            object.borrow_mut().move_to(self.last_offset);
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
                Some(_) => {
                    let offset_free_end = last_checked.data.front;
                    if offset_free_end - offset_free >= obj_size {
                        object.borrow_mut().move_to(offset_free);
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
                if self.end - offset_free >= obj_size {
                    object.borrow_mut().move_to(offset_free);
                    self.objects.add_end(
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
        if self.end - self.last_offset < obj_size {
            logf!("Out of GC memory!");
        }
        object.borrow_mut().move_to(self.last_offset);
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

impl GcObject for Gc {
    fn get_size(&self) -> usize {
        self.size
    }

    fn move_to(&mut self, offset: usize) {
        self.front = offset;
        self.end = offset + self.size;
        self.clean();
    }
}