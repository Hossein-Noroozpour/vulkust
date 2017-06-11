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
use super::number::Float;

#[derive(Debug, Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
    W,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vec4<T> where T: Float {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vec3<T> where T: Float {
    pub x: T,
    pub y: T,
    pub z: T,
}

macro_rules! as_expr { ($e:expr) => {$e} }

macro_rules! op3 {
    ($func:ident, $tra:ident, $opt:tt) => (
        impl<T> $tra for Vec3<T> where T: Float {
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
        impl<T> $tra<T> for Vec3<T> where T: Float {
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
        impl<T> $tra for Vec3<T> where T: Float {
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
        impl<T> $tra<T> for Vec3<T> where T: Float {
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

impl<E> Neg for Vec3<E> where E: Float {
    type Output = Vec3<E>;
    fn neg(self) -> Vec3<E> {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T> Vec3<T> where T: Float {
    pub fn new(e: T) -> Vec3<T> {
        Vec3 {
            x: e,
            y: e,
            z: e,
        }
    }

    pub fn dot(&self, o: &Vec3<T>) -> T {
        self.x * o.x + self.y * o.y + self.z * o.z
    }

    pub fn cross(&self, o: &Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.y * o.z - self.z * o.y,
            y: self.z * o.x - self.x * o.z,
            z: self.x * o.y - self.y * o.x
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

    pub fn normalized(&self) -> Vec3<T> {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Vec3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vec2<T> where T: Float {
    pub x: T,
    pub y: T,
}

macro_rules! op2 {
    ($func:ident, $tra:ident, $opt:tt) => (
        impl<T> $tra for Vec2<T> where T: Float {
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
        impl<T> $tra<T> for Vec2<T> where T: Float {
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
        impl<T> $tra for Vec2<T> where T: Float {
            fn $func(&mut self, other: Vec2<T>) {
                as_expr!(self.x $opt other.x);
                as_expr!(self.y $opt other.y);
            }
        }
    )
}

macro_rules! sopasg2 {
    ($func:ident, $tra:ident, $opt:tt) => (
        impl<T> $tra<T> for Vec2<T> where T: Float {
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

impl<E> Neg for Vec2<E> where E: Float {
    type Output = Vec2<E>;
    fn neg(self) -> Vec2<E> {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T> Vec2<T> where T: Float {
    pub fn new(e: T) -> Vec2<T> {
        Vec2 {
            x: e,
            y: e,
        }
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
