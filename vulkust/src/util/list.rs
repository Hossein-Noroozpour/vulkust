use std::mem::transmute;

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
        let list = self.list;
        Box::from_raw(self);
        if 0usize != unsafe { transmute(parent) } {
            parent.child = child;
            if 0usize == unsafe { transmute(child) } {
                list.end = parent;
                return None;
            } else {
                child.parent = parent;
                return Some(child);
            }
        }
        list.front = child;
        if 0usize != unsafe { transmute(child) } {
            child.parent = parent;
            return Some(child);
        }
        return None;
    }
}

pub struct List<T> where T: 'static {
    front: &'static mut ListNode<T>,
    end: &'static mut ListNode<T>,
}

impl<T> List<T> where T: 'static {
    pub fn new() -> Self {
        List {
            front: unsafe { transmute(0usize) },
            end: unsafe { transmute(0usize) },
        }
    }

    pub fn get_front(&mut self) -> Option<&'static mut ListNode<T>> {
        if 0usize == unsafe { transmute(self.front) } {
            return None;
        }
        Some(unsafe { transmute(self.front) })
    }

    pub fn get_end(&mut self) -> Option<&'static mut ListNode<T>> {
        if 0usize == unsafe { transmute(self.end) } {
            return None;
        }
        Some(unsafe { transmute(self.end) })
    }

    pub fn add_front(&mut self, data: T) {
        let last_front = self.front;
        self.front =  unsafe { 
            transmute(
                Box::into_raw(
                    Box::new(
                        ListNode {
                            data: data,
                            list: transmute(self),
                            child: last_front,
                            parent: transmute(0usize),
                        }
                    )
                )
            )
        };
        if 0usize != unsafe { transmute(last_front) } {
            last_front.parent = unsafe { transmute(self.front) };
        }
        if 0usize == unsafe { transmute(self.end) } {
            self.end = unsafe { transmute(self.front) };
        }
    }

    pub fn add_end(&mut self, data: T) {
        let last_end = self.end;
        self.end =  unsafe { 
            transmute(
                Box::into_raw(
                    Box::new(
                        ListNode {
                            data: data,
                            list: transmute(self),
                            child: transmute(0usize),
                            parent: last_end,
                        }
                    )
                )
            )
        };
        if 0usize != unsafe { transmute(last_end) } {
            last_end.child = unsafe { transmute(self.end) };
        }
        if 0usize == unsafe { transmute(self.front) } {
            self.front = unsafe { transmute(self.end) };
        }
    }
}