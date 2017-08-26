pub struct ListNode<T> {
    pub data: T,
    child: *mut ListNode<T>,
    parent: *mut ListNode<T>,
}

impl<T> ListNode<T> {
    pub fn get_child(&mut self) -> Option<&mut ListNode<T>> {
        if self.child == null_mut() {
            return None;
        }
        Some(unsafe { transmute(self.child) })
    }

    pub fn get_parent(&mut self) -> Option<&mut ListNode<T>> {
        if self.parent == null_mut() {
            return None;
        }
        Some(unsafe { transmute(self.parent) })
    }

    pub fn add_child(&mut self, data: T) {
        let mut sptr: usize = unsafe { transmute(self) };
        let grand_child = self.child;
        self.child = Box::into_raw(Box::new(ListNode {
            data: data,
            child: grand_child,
            parent:  unsafe { transmute(sptr) },
        }));
        if grand_child != null_mut() {
            unsafe {
                let mut grand_child: &mut ListNode<T> = grand_child;
                grand_child.parent = self.child;
            }
        }
    }

    pub fn add_parent(&mut self, data: T) {
        let mut sptr: usize = unsafe { transmute(self) };
        let grand_parent = self.parent;
        self.parent = Box::into_raw(Box::new(ListNode {
            data: data,
            child: unsafe { transmute(sptr) },
            parent: grand_parent,
        }));
        if grand_parent != null_mut() {
            unsafe {
                let mut grand_parent: &mut ListNode<T> = grand_parent;
                grand_parent.child = self.parent;
            }
        }
    }

    pub fn remove(&mut self) -> Option<&mut ListNode<T>> {
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

pub struct List<T> {
    front: *mut ListNode<T>,
    end: *mut ListNode<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            front: null_mut(),
            end: null_mut(),
        }
    }

    pub fn get_first_node(&mut self) -> Option<&mut ListNode<T>> {
        if self.front == null_mut() {
            return None;
        }
        Some(unsafe { transmute(self.front) })
    }

    pub fn add_front(&mut self, data: T) {
        let last_front = self.front;
        self.front =  Box::into_raw(Box::new(ListNode {
            data: data,
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