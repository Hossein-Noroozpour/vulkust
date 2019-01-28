use super::debug::Debug;
use super::types::Id;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, RwLock};

pub static NEXT_ID: AtomicU64 = AtomicU64::new(1);

pub fn create_id() -> Id {
    return NEXT_ID.fetch_add(1, Ordering::Relaxed);
}

pub trait Object: Debug {
    fn get_id(&self) -> Id;
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Base {
    pub id: Id,
}

impl Base {
    pub fn new() -> Self {
        Base { id: create_id() }
    }

    pub fn new_with_id(id: Id) -> Self {
        Base { id }
    }
}

impl Object for Base {
    fn get_id(&self) -> Id {
        self.id
    }
}

impl<T> Object for Arc<T>
where
    T: Object,
{
    fn get_id(&self) -> Id {
        return self.get_id();
    }
}

impl<T> Object for Mutex<T>
where
    T: Object,
{
    fn get_id(&self) -> Id {
        return vxresult!(self.lock()).get_id();
    }
}

impl<T> Object for RwLock<T>
where
    T: Object,
{
    fn get_id(&self) -> Id {
        return vxresult!(self.read()).get_id();
    }
}
