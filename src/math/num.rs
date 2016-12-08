
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

pub trait Number {
    fn square_root(&self) -> Self;
    fn absolute(&self) -> Self;
}

impl Number for f64 {
    fn square_root(&self) -> f64 {
        self.sqrt()
    }

    fn absolute(&self) -> f64 {
        self.abs()
    }
}

impl Number for f32 {
    fn square_root(&self) -> f32 {
        self.sqrt()
    }

    fn absolute(&self) -> f32 {
        self.abs()
    }
}
