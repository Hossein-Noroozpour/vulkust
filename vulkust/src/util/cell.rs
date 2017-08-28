#[cfg(cell_debug)]
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
#[cfg(not(cell_debug))]
use std::mem::transmute;

#[cfg(cell_debug)]
pub struct DebugCell<T> {
    data: RwLock<T>,
}

#[cfg(not(cell_debug))]
pub struct DebugCell<T> {
    data: *mut T,
}

impl<T> DebugCell<T> {
    #[cfg(cell_debug)]
    pub fn new(data: T) -> Self {
        DebugCell {
            data: RwLock::new(data),
        }
    }

    #[cfg(not(cell_debug))]
    pub fn new(data: T) -> Self {
        DebugCell {
            data: Box::into_raw(Box::new(data)),
        }
    }

    #[cfg(cell_debug)]
    pub fn borrow(&self) -> RwLockReadGuard<T> {
        match self.data.try_read() {
            Ok(r) => r,
            Err(_) => {
                logf!("Violation of shared mutable content.");
            },
        }
    }

    #[cfg(not(cell_debug))]
    pub fn borrow(&self) -> &T {
        transmute(self.data)
    }

    #[cfg(cell_debug)]
    pub fn borrow_mut(&self) -> RwLockWriteGuard<T> {
        match self.data.try_write() {
            Ok(r) => r,
            Err(_) => {
                logf!("Violation of shared mutable content.");
            },
        }
    }

    #[cfg(not(cell_debug))]
    pub fn borrow_mut(&self) -> &mut T {
        transmute(self.data)
    }
}

#[cfg(not(cell_debug))]
impl<T> Drop for DebugCell<T> {
    fn drop(&mut self) {
        Box::from_raw(self.data);
    }
}