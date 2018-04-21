#[cfg(cell_debug)]
use std::sync::Mutex;
use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};

#[cfg(cell_debug)]
pub struct DebugCell<T: ?Sized> {
    control: Mutex<(u64, bool)>,
    data: UnsafeCell<T>,
}

#[cfg(not(cell_debug))]
#[repr(C)]
pub struct DebugCell<T: ?Sized> {
    data: UnsafeCell<T>,
}

impl<T> DebugCell<T> {
    #[cfg(cell_debug)]
    pub fn new(data: T) -> Self {
        DebugCell {
            control: Mutex::new((0, false)),
            data: UnsafeCell::new(data),
        }
    }

    #[cfg(not(cell_debug))]
    pub fn new(data: T) -> Self {
        DebugCell {
            data: UnsafeCell::new(data),
        }
    }

}

#[cfg(cell_debug)]
pub struct DebugCellRefMut<'a, T: ?Sized + 'a> {
    cell: &'a DebugCell<T>,
}

#[cfg(cell_debug)]
impl<'a, T: ?Sized + 'a> DebugCellRefMut<'a, T> {
    fn new(cell: &'a DebugCell<T>) -> Self {
        DebugCellRefMut {
            cell: cell,
        }
    }
}

#[cfg(cell_debug)]
impl<'a, T: ?Sized> Deref for DebugCellRefMut<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.cell.data.get() }
    }
}

#[cfg(cell_debug)]
impl<'a, T: ?Sized> DerefMut for DebugCellRefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.cell.data.get() }
    }
}

#[cfg(cell_debug)]
impl<'a, T: ?Sized> Drop for DebugCellRefMut<'a, T> {
    fn drop(&mut self) {
        self.cell.clear_lock();
    }
}

#[cfg(cell_debug)]
pub struct DebugCellRef<'a, T: ?Sized + 'a> {
    cell: &'a DebugCell<T>,
}

#[cfg(cell_debug)]
impl<'a, T: ?Sized + 'a> DebugCellRef<'a, T> {
    fn new(cell: &'a DebugCell<T>) -> Self {
        DebugCellRef {
            cell: cell,
        }
    }
}

#[cfg(cell_debug)]
impl<'a, T: ?Sized> Deref for DebugCellRef<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.cell.data.get() }
    }
}

#[cfg(cell_debug)]
impl<'a, T: ?Sized> Drop for DebugCellRef<'a, T> {
    fn drop(&mut self) {
        self.cell.read_unlock();
    }
}

#[cfg(not(cell_debug))]
pub type DebugCellRef<'a, T: ?Sized + 'a> = &'a T;
#[cfg(not(cell_debug))]
pub type DebugCellRefMut<'a, T: ?Sized + 'a> = &'a mut T;

impl<T> DebugCell<T> where T: ?Sized {
    #[cfg(cell_debug)]
    pub fn borrow(&self) -> DebugCellRef<T> {
        let mut locked = self.control.lock().unwrap();
        if locked.1 {
            logf!("Violation of shared mutable content.");
        }
        locked.0 += 1;
        DebugCellRef::new(self)
    }

    #[cfg(not(cell_debug))]
    pub fn borrow(&self) -> DebugCellRef<T> {
        unsafe { &*self.data.get() }
    }

    #[cfg(cell_debug)]
    pub fn borrow_mut(&self) -> DebugCellRefMut<T> {
        let mut locked = self.control.lock().unwrap();
        if locked.1 || locked.0 != 0 {
            logf!("Violation of shared mutable content.");
        }
        locked.1 = true;
        DebugCellRefMut::new(self)
    }

    #[cfg(not(cell_debug))]
    pub fn borrow_mut(&self) -> DebugCellRefMut<T> {
        unsafe { &mut *self.data.get() }
    }

    pub unsafe fn untraced_mut_ref(&self) -> &mut T {
        &mut *self.data.get()
    }

    #[cfg(cell_debug)]
    fn clear_lock(&self) {
        let mut locked = self.control.lock().unwrap();
        locked.1 = false;
    }

    #[cfg(cell_debug)]
    fn read_unlock(&self) {
        let mut locked = self.control.lock().unwrap();
        locked.0 -= 1;
    }
}

unsafe impl<T: ?Sized + Send + Sync> Send for DebugCell<T> {}

unsafe impl<T: ?Sized + Send + Sync> Sync for DebugCell<T> {}