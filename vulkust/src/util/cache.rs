use std::collections::BTreeMap;
use std::sync::{Arc, Weak};
use super::cell::DebugCell;

pub struct Cacher<ID, VAL> where ID: Ord {
    cached: BTreeMap<ID, Weak<DebugCell<VAL>>>,
}

impl<ID, VAL> Cacher<ID, VAL> where ID: Ord {
    pub fn new() -> Self {
        Cacher {
            cached: BTreeMap::new(),
        }
    }

    pub fn get<F>(&mut self, id: ID, new: &F) -> Arc<DebugCell<VAL>>
    where F: Fn() -> VAL {
        match self.cached.get(&id) {
            Some(res) => match res.upgrade() {
                Some(res) => {
                    return res;
                }
                None => {}
            },
            None => {}
        }
        let res = Arc::new(DebugCell::new(new()));
        self.cached.insert(id, Arc::downgrade(&res));
        return res;
    }
}