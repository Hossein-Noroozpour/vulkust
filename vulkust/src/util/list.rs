use std::mem::transmute;

pub struct ListNode<T> {
    pub data: T,
    list: usize,
    child: usize,
    parent: usize,
}

impl<T: 'static> ListNode<T> {
    pub fn get_child(&mut self) -> Option<&'static mut ListNode<T>> {
        if 0 == self.child {
            return None;
        }
        Some(unsafe { transmute(self.child) })
    }

    pub fn get_parent(&mut self) -> Option<&'static mut ListNode<T>> {
        if 0 == self.parent {
            return None;
        }
        Some(unsafe { transmute(self.parent) })
    }

    pub fn add_child(&mut self, data: T) {
        let self_ptr = unsafe { transmute(self) };
        let grand_child = self.child;
        self.child = unsafe { 
            transmute(
                Box::into_raw(
                    Box::new(
                        ListNode {
                            data: data,
                            list: self.list,
                            child: grand_child,
                            parent: self_ptr,
                        }
                    )
                )
            )
        };
        if 0 == grand_child {
            let list: &'static mut List = unsafe { transmute(self.list) };
            list.end = self.child;
        } else {
            let grand_child: &'static mut ListNode<T> = unsafe { transmute(grand_child) };
            grand_child.parent = self.child;
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
                            list: self.list,
                            child: transmute(self),
                            parent: grand_parent,
                        }
                    )
                )
            )
        };
        if 0 == grand_parent {
            let list: &'static mut List = unsafe { transmute(self.list) };
            list.front = self.parent;
        } else {
            let grand_parent: &'static mut ListNode<T> = unsafe { transmute(grand_parent) };
            grand_parent.child = self.parent;
        }
    }

    pub fn remove(&mut self) -> Option<&'static mut ListNode<T>> {
        let parent = self.parent;
        let child = self.child;
        let list = self.list;
        let parent2: &'static mut ListNode<T> = unsafe { transmute(parent) };
        let child2: &'static mut ListNode<T> = unsafe { transmute(child) };
        let list2: &'static mut List = unsafe { transmute(list) };
        unsafe { Box::from_raw(self); }
        if 0 != parent {
            parent2.child = child;
            if 0 == child {
                list2.end = parent;
                return None;
            } else {
                child2.parent = parent;
                return Some(child2);
            }
        }
        list2.front = child;
        if 0 != child {
            child2.parent = parent;
            return Some(child2);
        }
        return None;
    }
}

pub struct List {
    front: usize,
    end: usize,
}

impl List {
    pub fn new() -> Self {
        List {
            front: 0,
            end: 0,
        }
    }

    pub fn get_front<T: 'static>(&mut self) -> Option<&'static mut ListNode<T>> {
        if 0 == self.front {
            return None;
        }
        Some(unsafe { transmute(self.front) })
    }

    pub fn get_end<T: 'static>(&mut self) -> Option<&'static mut ListNode<T>> {
        if 0 == self.end {
            return None;
        }
        Some(unsafe { transmute(self.end) })
    }

    pub fn add_front<T: 'static>(&mut self, data: T) {
        let last_front = self.front;
        self.front =  unsafe { 
            transmute(
                Box::into_raw(
                    Box::new(
                        ListNode {
                            data: data,
                            list: transmute(self),
                            child: last_front,
                            parent: 0,
                        }
                    )
                )
            )
        };
        if 0 != last_front {
            let last_front: &'static mut ListNode<T> = unsafe { transmute(last_front) };
            last_front.parent = self.front;
        }
        if 0 == self.end {
            self.end = self.front;
        }
    }

    pub fn add_end<T: 'static>(&mut self, data: T) {
        let last_end = self.end;
        self.end =  unsafe { 
            transmute(
                Box::into_raw(
                    Box::new(
                        ListNode {
                            data: data,
                            list: transmute(self),
                            child: 0,
                            parent: last_end,
                        }
                    )
                )
            )
        };
        if 0 != last_end {
            let last_end: &'static mut ListNode<T> = unsafe { transmute(last_end) };
            last_end.child = unsafe { transmute(self.end) };
        }
        if 0 == self.front {
            self.front = self.end;
        }
    }
}