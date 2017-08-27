use std::mem::transmute;
use std::ptr::null_mut;

pub struct ListNode<T> where T: 'static {
    pub data: T,
    list: &'static mut List<T>,
    child: &'static mut ListNode<T>,
    parent: &'static mut ListNode<T>,
}

impl<T> ListNode<T> {
    pub fn get_child(&mut self) -> Option<&'static mut ListNode<T>> {
        if 0usize == unsafe { transmute(self.child) } {
            return None;
        }
        Some(unsafe { transmute(self.child) })
    }

    pub fn get_parent(&mut self) -> Option<&'static mut ListNode<T>> {
        if 0usize == unsafe { transmute(self.parent) } {
            return None;
        }
        Some(unsafe { transmute(self.parent) })
    }

    pub fn add_child(&mut self, data: T) {
        let grand_child = self.child;
        self.child = unsafe { 
            transmute(
                Box::into_raw(
                    Box::new(
                        ListNode {
                            data: data,
                            list: transmute(self.list),
                            child: grand_child,
                            parent: transmute(self),
                        }
                    )
                )
            )
        };
        if 0usize == unsafe { transmute(grand_child) } {
            self.list.end = unsafe { transmute(self.child) };
        } else {
            grand_child.parent = unsafe { transmute(self.child) };
        }
    }

    pub fn add_parent(&mut self, data: T) {
        let grand_parent = self.parent;
        self.parent = unsafe { 
            transmute(
                Box::into_raw(
                    Box::new(
                        ListNode {
                            data: data,
                            list: transmute(self.list),
                            child: transmute(self),
                            parent: grand_parent,
                        }
                    )
                )
            )
        };
        if 0usize == unsafe { transmute(grand_parent) } {
            self.list.front = unsafe { transmute(self.parent) };
        } else {
            grand_parent.child = unsafe { transmute(self.parent) };
        }
    }

    pub fn remove(&mut self) -> Option<&'static mut ListNode<T>> {
        let parent = self.parent;
        let child = self.child;
        if parent != null_mut() {
            let parent: &mut ListNode<T> = unsafe { transmute(parent) };
            Box::from_raw(parent.child);
            parent.child = child;
            if child != null_mut() {
                let child: &mut ListNode<T> = unsafe { transmute(child) };
                child.parent = parent;
                return Some(child);
            }
            return None;
        }
        if child != null_mut() {
            let child: &mut ListNode<T> = unsafe { transmute(child) };
            Box::from_raw(child.parent);
            child.parent = parent;
            if parent != null_mut() {
                let parent: &mut ListNode<T> = unsafe { transmute(parent) };
                parent.child = child;
            }
            return Some(child);
        }
        Box::from_raw(self);
        return None;
    }

    pub fn remove_child(&mut self) {
        if self.child == null_mut() {
            return;
        }
        let removed_child = self.child;
        let sptr = unsafe { (*self.child).parent };
        self.child = unsafe { (*self.child).child };
        if self.child != null_mut() {
            unsafe {
                (*self.child).parent = sptr;
            }
        }
        Box::from_raw(removed_child);
    }
}

pub struct List<T> where T: 'static {
    front: &'static mut ListNode<T>,
    end: &'static mut ListNode<T>,
}

impl<T> List<T> where T: 'static {
    pub fn new() -> Self {
        List {
            front: null_mut(),
            end: null_mut(),
        }
    }

    pub fn get_front(&mut self) -> Option<&'static mut ListNode<T>> {
        if self.front == null_mut() {
            return None;
        }
        Some(unsafe { transmute(self.front) })
    }

    pub fn get_end(&mut self) -> Option<&'static mut ListNode<T>> {
        if self.end == null_mut() {
            return None;
        }
        Some(unsafe { transmute(self.end) })
    }

    pub fn add_front(&mut self, data: T) {
        let last_front = self.front;
        self.front =  Box::into_raw(Box::new(ListNode {
            data: data,
            list: unsafe { transmute(self) },
            child: last_front,
            parent: null_mut(),
        }));
        if last_front != null_mut() {
            unsafe {
                (*last_front).parent = self.front;
            }
        }
        if self.end == null_mut() {
            self.end == self.front;
        }
    }

    pub fn add_end(&mut self, data: T) {
        let last_end = self.end;
        self.end =  Box::into_raw(Box::new(ListNode {
            data: data,
            list: unsafe { transmute(self) },
            child: null_mut(),
            parent: last_end,
        }));
        if last_end != null_mut() {
            unsafe {
                (*last_end).child = self.end;
            }
        }
        if self.front == null_mut() {
            self.front == self.end;
        }
    }
}