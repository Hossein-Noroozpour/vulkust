#[cfg(debug_assertions)]
use std::fmt::Debug as StdDebug;

#[cfg(debug_assertions)]
pub trait Debug: StdDebug {}

#[cfg(debug_assertions)]
impl<T> Debug for T where T: StdDebug {}

#[cfg(not(debug_assertions))]
pub trait Debug {}

#[cfg(not(debug_assertions))]
impl<T> Debug for T {}
