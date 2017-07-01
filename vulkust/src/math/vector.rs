use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign, SubAssign, MulAssign, DivAssign};
#[cfg(any(target_os = "macos", target_os = "ios"))]
use super::super::objc;
use super::number::Number;

#[repr(simd)]
pub struct SVec4D(pub f64, pub f64, pub f64, pub f64);
#[repr(simd)]
pub struct SVec3D(pub f64, pub f64, pub f64);
#[repr(simd)]
pub struct SVec2D(pub f64, pub f64);
#[repr(simd)]
pub struct SVec4F(pub f32, pub f32, pub f32, pub f32);
#[repr(simd)]
pub struct SVec3F(pub f32, pub f32, pub f32);
#[repr(simd)]
pub struct SVec2F(pub f32, pub f32);
#[repr(simd)]
pub struct SVec4U32(pub u32, pub u32, pub u32, pub u32);
#[repr(simd)]
pub struct SVec3U32(pub u32, pub u32, pub u32);
#[repr(simd)]
pub struct SVec2U32(pub u32, pub u32);

#[repr(usize)]
#[derive(Debug, Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
    W,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vec4<T>
where
    T: Number,
{
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
unsafe impl<T> objc::Encode for Vec4<T>
where
    T: Number,
{
    fn encode() -> objc::Encoding {
        let encoding = format!(
            "{{?={}{}{}{}}}",
            T::objc_encode(),
            T::objc_encode(),
            T::objc_encode(),
            T::objc_encode()
        );
        unsafe { objc::Encoding::from_str(&encoding) }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vec3<T>
where
    T: Number,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

macro_rules! as_expr { ($e:expr) => {$e} }

macro_rules! op3 {
    ($func:ident, $tra:ident, $opt:tt) => (
        impl<'a, 'b, T> $tra<&'b Vec3<T>> for &'a Vec3<T> where T: Number {
            type Output = Vec3<T>;
            fn $func(self, other: &'b Vec3<T>) -> Vec3<T> {
                Vec3 {
                    x: as_expr!(self.x $opt other.x),
                    y: as_expr!(self.y $opt other.y),
                    z: as_expr!(self.z $opt other.z),
                }
            }
        }
    )
}

macro_rules! sop3 {
    ($func:ident, $tra:ident, $opt:tt) => (
        impl<'a, T> $tra<T> for &'a Vec3<T> where T: Number {
            type Output = Vec3<T>;
            fn $func(self, other: T) -> Vec3<T> {
                Vec3 {
                    x: as_expr!(self.x $opt other),
                    y: as_expr!(self.y $opt other),
                    z: as_expr!(self.z $opt other),
                }
            }
        }
    )
}

macro_rules! opasg3 {
    ($func:ident, $tra:ident, $opt:tt) => (
        impl<'a, T> $tra<&'a Vec3<T>> for Vec3<T> where T: Number {
            fn $func(&mut self, other: &'a Vec3<T>) {
                as_expr!(self.x $opt other.x);
                as_expr!(self.y $opt other.y);
                as_expr!(self.z $opt other.z);
            }
        }
    )
}

macro_rules! sopasg3 {
    ($func:ident, $tra:ident, $opt:tt) => (
        impl<T> $tra<T> for Vec3<T> where T: Number {
            fn $func(&mut self, other: T) {
                as_expr!(self.x $opt other);
                as_expr!(self.y $opt other);
                as_expr!(self.z $opt other);
            }
        }
    )
}

op3!(add, Add, +);
op3!(sub, Sub, -);
op3!(mul, Mul, *);
op3!(div, Div, /);

sop3!(add, Add, +);
sop3!(sub, Sub, -);
sop3!(mul, Mul, *);
sop3!(div, Div, /);

opasg3!(add_assign, AddAssign, +=);
opasg3!(sub_assign, SubAssign, -=);
opasg3!(mul_assign, MulAssign, *=);
opasg3!(div_assign, DivAssign, /=);

sopasg3!(add_assign, AddAssign, +=);
sopasg3!(sub_assign, SubAssign, -=);
sopasg3!(mul_assign, MulAssign, *=);
sopasg3!(div_assign, DivAssign, /=);

impl<'a, E> Neg for &'a Vec3<E>
where
    E: Number + Neg<Output = E>,
{
    type Output = Vec3<E>;
    fn neg(self) -> Vec3<E> {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T> Vec3<T>
where
    T: Number,
{
    pub fn new(e: T) -> Self {
        Vec3 { x: e, y: e, z: e }
    }

    pub fn dot(&self, o: &Vec3<T>) -> T {
        self.x * o.x + self.y * o.y + self.z * o.z
    }

    pub fn cross(&self, o: &Vec3<T>) -> Self {
        Vec3 {
            x: self.y * o.z - self.z * o.y,
            y: self.z * o.x - self.x * o.z,
            z: self.x * o.y - self.y * o.x,
        }
    }

    pub fn length(&self) -> T {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn absolute_length(&self) -> T {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    pub fn square_length(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(&mut self) {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        self.x /= len;
        self.y /= len;
        self.z /= len;
    }

    pub fn normalized(&self) -> Self {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Vec3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
unsafe impl<T> objc::Encode for Vec3<T>
where
    T: Number,
{
    fn encode() -> objc::Encoding {
        let encoding = format!("![3{}]", T::objc_encode());
        unsafe { objc::Encoding::from_str(&encoding) }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vec2<T>
where
    T: Number,
{
    pub x: T,
    pub y: T,
}

macro_rules! op2 {
    ($func:ident, $tra:ident, $opt:tt) => (
        impl<'a, 'b, T> $tra<&'b Vec2<T>> for &'a Vec2<T> where T: Number {
            type Output = Vec2<T>;
            fn $func(self, other: &'b Vec2<T>) -> Vec2<T> {
                Vec2 {
                    x: as_expr!(self.x $opt other.x),
                    y: as_expr!(self.y $opt other.y),
                }
            }
        }
    )
}

macro_rules! sop2 {
    ($func:ident, $tra:ident, $opt:tt) => (
        impl<'a, T> $tra<T> for &'a Vec2<T> where T: Number {
            type Output = Vec2<T>;
            fn $func(self, other: T) -> Vec2<T> {
                Vec2 {
                    x: as_expr!(self.x $opt other),
                    y: as_expr!(self.y $opt other),
                }
            }
        }
    )
}

macro_rules! opasg2 {
    ($func:ident, $tra:ident, $opt:tt) => (
        impl<'a, T> $tra<&'a Vec2<T>> for Vec2<T> where T: Number {
            fn $func(&mut self, other: &'a Vec2<T>) {
                as_expr!(self.x $opt other.x);
                as_expr!(self.y $opt other.y);
            }
        }
    )
}

macro_rules! sopasg2 {
    ($func:ident, $tra:ident, $opt:tt) => (
        impl<T> $tra<T> for Vec2<T> where T: Number {
            fn $func(&mut self, other: T) {
                as_expr!(self.x $opt other);
                as_expr!(self.y $opt other);
            }
        }
    )
}

op2!(add, Add, +);
op2!(sub, Sub, -);
op2!(mul, Mul, *);
op2!(div, Div, /);

sop2!(add, Add, +);
sop2!(sub, Sub, -);
sop2!(mul, Mul, *);
sop2!(div, Div, /);

opasg2!(add_assign, AddAssign, +=);
opasg2!(sub_assign, SubAssign, -=);
opasg2!(mul_assign, MulAssign, *=);
opasg2!(div_assign, DivAssign, /=);

sopasg2!(add_assign, AddAssign, +=);
sopasg2!(sub_assign, SubAssign, -=);
sopasg2!(mul_assign, MulAssign, *=);
sopasg2!(div_assign, DivAssign, /=);

impl<'a, E> Neg for &'a Vec2<E>
where
    E: Number + Neg<Output = E>,
{
    type Output = Vec2<E>;
    fn neg(self) -> Vec2<E> {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T> Vec2<T>
where
    T: Number,
{
    pub fn new(e: T) -> Vec2<T> {
        Vec2 { x: e, y: e }
    }

    pub fn dot(&self, o: &Vec2<T>) -> T {
        self.x * o.x + self.y * o.y
    }

    pub fn cross(&self, o: &Vec2<T>) -> Vec2<T> {
        println!("{:?}", o);
        println!("In 2D we can not have a cross product");
        Vec2 {
            x: T::new(0.0),
            y: T::new(0.0),
        }
    }

    pub fn length(&self) -> T {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn absolute_length(&self) -> T {
        self.x.abs() + self.y.abs()
    }

    pub fn square_length(&self) -> T {
        self.x * self.x + self.y * self.y
    }

    pub fn normalize(&mut self) {
        let len = (self.x * self.x + self.y * self.y).sqrt();
        self.x /= len;
        self.y /= len;
    }

    pub fn normalized(&self) -> Vec2<T> {
        let len = (self.x * self.x + self.y * self.y).sqrt();
        Vec2 {
            x: self.x / len,
            y: self.y / len,
        }
    }
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
unsafe impl<T> objc::Encode for Vec2<T>
where
    T: Number,
{
    fn encode() -> objc::Encoding {
        let encoding = format!("{{?={}{}}}", T::objc_encode(), T::objc_encode());
        unsafe { objc::Encoding::from_str(&encoding) }
    }
}
