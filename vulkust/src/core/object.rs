use super::types::Id;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
pub trait Object {
    fn get_id(&self) -> Id;
}

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

pub fn create_id() -> Id {
    return NEXT_ID.fetch_add(1, Ordering::Relaxed);
}
