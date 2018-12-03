use std::mem::{size_of, transmute};
use std::os::raw::c_void;
use std::sync::{Arc, RwLock};

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct Buffer {}

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct Dynamic {
    actual_size: isize,
}

impl Dynamic {
    pub(crate) fn update_with_ptr(&mut self, _data: *const c_void, _frame_number: usize) {
        vxunimplemented!();
    }

    pub(crate) fn update<T>(&mut self, data: &T, frame_number: usize)
    where
        T: Sized,
    {
        #[cfg(debug_mode)]
        {
            if size_of::<T>() != self.actual_size as usize {
                vxlogf!("Data must have same size of buffer.");
            }
        }
        self.update_with_ptr(unsafe { transmute(data) }, frame_number);
    }

    pub(crate) fn get_buffer(&self, _frame_number: usize) -> &Arc<RwLock<Buffer>> {
        vxunimplemented!();
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct Static {}

#[cfg_attr(debug_mode, derive(Debug))]
pub(crate) struct Manager {}

impl Manager {
    pub(crate) fn create_dynamic_buffer(&mut self, _: isize) -> Dynamic {
        vxunimplemented!();
    }

    pub(crate) fn create_static_buffer_with_vec<T>(&mut self, _data: &[T]) -> Static {
        vxunimplemented!();
    }
}
