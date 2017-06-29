use std::default::Default;
use std::mem::zeroed;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ARect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Default for ARect {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
