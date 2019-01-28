pub mod manager;
pub mod transparent;
pub mod unlit;

use super::super::core::object::Object;

pub trait Pass: Object {}
