extern crate num;

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

use ::io::file::Stream;
use super::num::Number;

pub enum Axis {
    X,
    Y,
    Z,
    W,
}

pub trait VectorElement:
        Add<Output=Self> +
        Sub<Output=Self> +
        Mul<Output=Self> +
        Div<Output=Self> +
        Neg<Output=Self> +
        AddAssign +
        SubAssign +
        MulAssign +
        DivAssign +
        num::NumCast +
        Number +
        PartialOrd +
        Copy +
        Clone +
        Debug {
}

impl<T> VectorElement for T where
    T:
        Add<Output=T> +
        Sub<Output=T> +
        Mul<Output=T> +
        Div<Output=T> +
        Neg<Output=T> +
        AddAssign +
        SubAssign +
        MulAssign +
        DivAssign +
        num::NumCast +
        Number +
        PartialOrd +
        Copy +
        Clone +
        Debug {

}

pub trait MathVector <ElementType>:
        Sized +
        Add<Output=Self> +
        AddAssign +
        Sub<Output=Self> +
        SubAssign +
        Mul<ElementType, Output=Self> +
        MulAssign<ElementType> +
        Div<ElementType, Output=Self> +
        DivAssign<ElementType> +
        Neg<Output=Self>
    where ElementType: VectorElement {
    fn new(e: ElementType) -> Self;
    fn dot(&self, o: &Self) -> ElementType;
    fn cross(&self, o: &Self) -> Self;
    fn length(&self) -> ElementType;
    fn absolute_length(&self) -> ElementType;
    fn square_length(&self) -> ElementType;
    fn normalize(&mut self);
    fn normalized(&self) -> Self;
    fn read(&mut self, s: &mut Stream);
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3<T> where T: VectorElement, Vec3<T>: MathVector<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

macro_rules! as_expr { ($e:expr) => {$e} }

macro_rules! op3 {
    ($func:ident, $tra:ident, $opt:tt) => (
        impl<T> $tra for Vec3<T> where T: VectorElement {
            type Output = Vec3<T>;
            fn $func(self, other: Vec3<T>) -> Vec3<T> {
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
        impl<T> $tra<T> for Vec3<T> where T: VectorElement {
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
        impl<T> $tra for Vec3<T> where T: VectorElement {
            fn $func(&mut self, other: Vec3<T>) {
                as_expr!(self.x $opt other.x);
                as_expr!(self.y $opt other.y);
                as_expr!(self.z $opt other.z);
            }
        }
    )
}

macro_rules! sopasg3 {
    ($func:ident, $tra:ident, $opt:tt) => (
        impl<T> $tra<T> for Vec3<T> where T: VectorElement {
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

impl<E> Neg for Vec3<E> where E: VectorElement {
    type Output = Vec3<E>;
    fn neg(self) -> Vec3<E> {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T> MathVector<T> for Vec3<T> where T: VectorElement {

    fn new(e: T) -> Vec3<T> {
        Vec3 {
            x: e,
            y: e,
            z: e,
        }
    }

    fn dot(&self, o: &Vec3<T>) -> T {
        self.x * o.x + self.y * o.y + self.z * o.z
    }

    fn cross(&self, o: &Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.y * o.z - self.z * o.y,
            y: self.z * o.x - self.x * o.z,
            z: self.x * o.y - self.y * o.x
        }
    }

    fn length(&self) -> T {
        (self.x * self.x + self.y * self.y + self.z * self.z).square_root()
    }

    fn absolute_length(&self) -> T {
        self.x.absolute() + self.y.absolute() + self.z.absolute()
    }

    fn square_length(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn normalize(&mut self) {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).square_root();
        self.x /= len;
        self.y /= len;
        self.z /= len;
    }

    fn normalized(&self) -> Vec3<T> {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).square_root();
        Vec3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len
        }
    }

    fn read(&mut self, s: &mut Stream) {
        self.x = num::cast::<f32, T>(s.read(&0f32)).unwrap();
        self.y = num::cast::<f32, T>(s.read(&0f32)).unwrap();
        self.z = num::cast::<f32, T>(s.read(&0f32)).unwrap();
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vec2<T> where T: VectorElement, Vec2<T>: MathVector<T> {
    pub x: T,
    pub y: T,
}

macro_rules! op2 {
    ($func:ident, $tra:ident, $opt:tt) => (
        impl<T> $tra for Vec2<T> where T: VectorElement {
            type Output = Vec2<T>;
            fn $func(self, other: Vec2<T>) -> Vec2<T> {
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
        impl<T> $tra<T> for Vec2<T> where T: VectorElement {
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
        impl<T> $tra for Vec2<T> where T: VectorElement {
            fn $func(&mut self, other: Vec2<T>) {
                as_expr!(self.x $opt other.x);
                as_expr!(self.y $opt other.y);
            }
        }
    )
}

macro_rules! sopasg2 {
    ($func:ident, $tra:ident, $opt:tt) => (
        impl<T> $tra<T> for Vec2<T> where T: VectorElement {
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

impl<E> Neg for Vec2<E> where E: VectorElement {
    type Output = Vec2<E>;
    fn neg(self) -> Vec2<E> {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T> MathVector<T> for Vec2<T> where T: VectorElement {

    fn new(e: T) -> Vec2<T> {
        Vec2 {
            x: e,
            y: e,
        }
    }

    fn dot(&self, o: &Vec2<T>) -> T {
        self.x * o.x + self.y * o.y
    }

    fn cross(&self, o: &Vec2<T>) -> Vec2<T> {
        println!("{:?}", o);
        println!("In 2D we can not have a cross product");
        Vec2 {
            x: num::cast(0).unwrap(),
            y: num::cast(0).unwrap(),
        }
    }

    fn length(&self) -> T {
        (self.x * self.x + self.y * self.y).square_root()
    }

    fn absolute_length(&self) -> T {
        self.x.absolute() + self.y.absolute()
    }

    fn square_length(&self) -> T {
        self.x * self.x + self.y * self.y
    }

    fn normalize(&mut self) {
        let len = (self.x * self.x + self.y * self.y).square_root();
        self.x /= len;
        self.y /= len;
    }

    fn normalized(&self) -> Vec2<T> {
        let len = (self.x * self.x + self.y * self.y).square_root();
        Vec2 {
            x: self.x / len,
            y: self.y / len,
        }
    }

    fn read(&mut self, s: &mut Stream) {
        self.x = s.read::<T>(&num::cast(0).unwrap());
        self.y = s.read::<T>(&num::cast(0).unwrap());
    }
}
