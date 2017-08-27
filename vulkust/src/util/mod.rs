pub mod gc;
pub mod list;
pub mod string;

use std::collections::BTreeMap;
use std::default::Default;
use std::mem::transmute;
use std::ptr::{null, null_mut};
use std::sync::{Arc, Weak};

pub struct Cacher<ID, VAL> where ID: Ord {
    cached: BTreeMap<ID, Weak<VAL>>,
}

impl<ID, VAL> Cacher<ID, VAL> where ID: Ord {
    pub fn new() -> Self {
        Cacher {
            cached: BTreeMap::new(),
        }
    }

    pub fn get<F>(&mut self, id: ID, new: &F) -> Arc<VAL> 
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
        let res = Arc::new(new());
        self.cached.insert(id, Arc::downgrade(&res));
        return res;
    }
}