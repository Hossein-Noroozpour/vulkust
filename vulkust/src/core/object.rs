use super::types::Id;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

pub fn create_id() -> Id {
    return NEXT_ID.fetch_add(1, Ordering::Relaxed);
}

pub trait Object {
    fn get_id(&self) -> Id;
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Basic {
    pub id: Id,
}

impl Basic {
    pub fn new() -> Self {
        Basic {
            id: create_id(),
        }
    }

    pub fn new_with_id(id: Id) -> Self {
        Basic {
            id,
        }
    }
}

impl Object for Basic {
    fn get_id(&self) -> Id {
        self.id
    }
}
