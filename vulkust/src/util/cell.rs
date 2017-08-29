#[cfg(cell_debug)]
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
#[cfg(not(cell_debug))]
use std::mem::transmute;

#[cfg(cell_debug)]
pub struct DebugCell<T: ?Sized> {
    data: RwLock<T>,
}

#[cfg(not(cell_debug))]
#[repr(C)]
pub struct DebugCell<T: ?Sized> {
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

}
#[cfg(cell_debug)]
pub type DebugCellRefMut<'a, T: 'a> = RwLockWriteGuard<'a, T>;
#[cfg(cell_debug)]
pub type DebugCellRef<'a, T: 'a> = RwLockReadGuard<'a, T>;

#[cfg(not(cell_debug))]
pub type DebugCellRef<'a, T: ?Sized + 'a> = &'a T;
#[cfg(not(cell_debug))]
pub type DebugCellRefMut<'a, T: ?Sized + 'a> = &'a mut T;

impl<T> DebugCell<T> where T: ?Sized {
    #[cfg(cell_debug)]
    pub fn borrow(&self) -> DebugCellRef<T> {
        match self.data.try_read() {
            Ok(r) => r,
            Err(_) => {
                logf!("Violation of shared mutable content.");
            },
        }
    }

    #[cfg(not(cell_debug))]
    pub fn borrow(&self) -> DebugCellRef<T> {
        transmute(self.data)
    }

    #[cfg(cell_debug)]
    pub fn borrow_mut(&self) -> DebugCellRefMut<T> {
        match self.data.try_write() {
            Ok(r) => r,
            Err(_) => {
                logf!("Violation of shared mutable content.");
            },
        }
    }

    #[cfg(not(cell_debug))]
    pub fn borrow_mut(&self) -> DebugCellRefMut<T> {
        transmute(self.data)
    }
}

#[cfg(not(cell_debug))]
impl<T: ?Sized> Drop for DebugCell<T> {
    fn drop(&mut self) {
        Box::from_raw(self.data);
    }
}