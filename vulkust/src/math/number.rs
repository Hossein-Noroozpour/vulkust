use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
    Neg,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
};
use std::fmt::Debug;

pub fn min<T>(a: T, b: T) -> T where T: PartialOrd + Copy + Clone {
    if a.lt(&b) {
        return a;
    }
    return b;
}

pub fn max<T>(a: T, b: T) -> T where T: PartialOrd + Copy + Clone {
    if a.gt(&b) {
        return a;
    }
    return b;
}

pub trait Number:
        Add<Output = Self> +
        Sub<Output = Self> +
        Mul<Output = Self> +
        Div<Output = Self> +
        AddAssign +
        SubAssign +
        MulAssign +
        DivAssign +
        Sized +
        Copy +
        Clone +
        Debug {
    fn new(f: f64) -> Self;
    fn sqrt(&self) -> Self;
    fn abs(&self) -> Self;
    fn to(&self) -> f64;
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    fn objc_encode() -> &'static str;
}

impl Number for f64 {
    fn new(f: f64) -> Self { f }
    fn sqrt(&self) -> Self { f64::sqrt(*self) }
    fn abs(&self) -> Self { f64::abs(*self) }
    fn to(&self) -> f64 { *self }
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    fn objc_encode() -> &'static str { "f" }
}

impl Number for f32 {
    fn new(f: f64) -> Self { f as f32 }
    fn sqrt(&self) -> Self { f32::sqrt(*self) }
    fn abs(&self) -> Self { f32::abs(*self) }
    fn to(&self) -> f64 { *self as f64 }
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    fn objc_encode() -> &'static str { "d" }
}

impl Number for u32 {
    fn new(f: f64) -> Self { f as u32 }
    fn sqrt(&self) -> Self { f64::sqrt(*self as f64) as u32 }
    fn abs(&self) -> Self { *self }
    fn to(&self) -> f64 { *self as f64 }
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    fn objc_encode() -> &'static str { "I" }
}

pub trait Float: Number + Neg<Output = Self> {}

impl Float for f32 {}

impl Float for f64 {}
