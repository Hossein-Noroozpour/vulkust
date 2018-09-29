#[cfg(debug_mode)]
use std::fmt::Debug as StdDebug;

#[cfg(debug_mode)]
pub trait Debug: StdDebug {}

#[cfg(debug_mode)]
impl<T> Debug for T where T: StdDebug {}

#[cfg(not(debug_mode))]
pub trait Debug {}

#[cfg(not(debug_mode))]
impl<T> Debug for T {}
