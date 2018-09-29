use super::engine::Engine;
use std::sync::{RwLock, Weak};

#[cfg_attr(debug_mode, derive(Debug))]
pub (in super) struct Kernel {
    index: usize,
    engine: Weak<RwLock<Engine>>,
}

impl Kernel {
    pub fn new(index: usize, engine: Weak<RwLock<Engine>>) -> Self {
        Self {
            index,
            engine,
        }
    }
}