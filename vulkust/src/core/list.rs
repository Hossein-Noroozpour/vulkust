use std::sync::{
    Arc,
    RwLock,
};

struct Node<T> {
    pub preceding: Option<Arc<RwLock<Node<T>>>>,
    pub value: T,
    pub suceeding: Option<Arc<RwLock<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            preceding: None,
            value,
            suceeding: None,
        }
    }
}

struct List<T> {
    pub starting: Option<Arc<RwLock<Node<T>>>>,
    pub ending: Option<Arc<RwLock<Node<T>>>>,
    pub count: usize,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            starting: None,
            ending: None,
            count: 0,
        }
    }

    pub fn add_after(node: &Node<T>, value: T) {
        let new_node = Arc::new(RwLock::new(Node::new(value)));

    }
}