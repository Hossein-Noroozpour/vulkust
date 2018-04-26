use std::sync::{Arc, RwLock};

pub struct Node<T> {
    preceding: Option<Arc<RwLock<Node<T>>>>,
    pub value: T,
    suceeding: Option<Arc<RwLock<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            preceding: None,
            value,
            suceeding: None,
        }
    }

    pub fn previous(&self) -> Option<Arc<RwLock<Node<T>>>> {
        self.preceding
    }
}

pub struct List<T> {
    starting: Option<Arc<RwLock<Node<T>>>>,
    ending: Option<Arc<RwLock<Node<T>>>>,
    count: usize,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            starting: None,
            ending: None,
            count: 0,
        }
    }

    pub fn add_after(&mut self, node: &Arc<RwLock<Node<T>>>, value: T) -> Arc<RwLock<Node<T>>> {
        self.count += 1;
        let new_node = Arc::new(RwLock::new(Node::new(value)));
        let next_node = vxunwrap!(node.read()).suceeding;
        vxunwrap!(node.write()).suceeding = Some(new_node.clone());
        vxunwrap!(new_node.write()).preceding = Some(node);
        vxunwrap!(new_node.write()).suceeding = next_node;
        match next_node {
            Some(next_node) => vxunwrap!(next_node.write()).preceding = Some(new_node.clone()),
            None => self.ending = Some(new_node.clone()),
        }
        new_node
    }

    pub fn append(&mut self, value: T) {
        self.count += 1;
        let node = Arc::new(RwLock::new(Node::new(value)));
        if self.ending.is_some() {
            let end = vxunwrap!(self.ending);
            vxunwrap!(end.write()).suceeding = Some(node.clone());
            vxunwrap!(node.write()).preceding = Some(end);
            self.ending = Some(node);
        } else {
            self.starting = Some(node.clone());
            self.ending = Some(node);
        }
    }

    pub fn front(&self) -> Option<Arc<RwLock<Node<T>>>> {
        self.starting
    }

    pub fn back(&self) -> Option<Arc<RwLock<Node<T>>>> {
        self.ending
    }

    pub fn clear(&mut self) {
        let mut cn = self.starting;
        while cn.is_some() {
            let n = vxunwrap!(vxunwrap!(cn).write());
            n.preceding = None;
            cn = n.suceeding;
        }
        self.starting = None;
        self.ending = None;
        self.count = 0;
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        self.clear();
    }
}