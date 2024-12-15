use crate::{forward_ref_binop, impl_ops_assign_ref_copy, impl_ops_ref_copy};
use std::ops::{Add, AddAssign, Mul, Sub};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl<T> From<(T, T)> for Point
where
    T: Into<i64>,
{
    fn from((x, y): (T, T)) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl Point {
    pub fn left(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }
    pub fn right(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }
    pub fn down(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }
    pub fn up(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn left_up(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y - 1,
        }
    }
    pub fn right_up(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y + 1,
        }
    }
    pub fn left_down(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y - 1,
        }
    }
    pub fn right_down(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
    /**
    Rotate clockwise about 90 degree origin
    */
    pub fn rot90_cw(&self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
        }
    }
    /**
    Rotate counter-clockwise about 90 degree origin
    */
    pub fn rot90_ccw(&self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }
    
    pub fn inverse(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl_ops_ref_copy!(Add, add |p: Point, other: Point| Point {
    x: p.x + other.x,
    y: p.y + other.y
});

impl_ops_ref_copy!(Add, add |p: Point, other: (i64, i64)| Point {
    x: p.x + other.0,
    y: p.y + other.1
});

impl_ops_ref_copy!(Sub, sub |p: Point, other: Point| Point {
    x: p.x - other.x,
    y: p.y - other.y
});

impl_ops_ref_copy!(Sub, sub |p: Point, other: (i64, i64)| Point {
    x: p.x - other.0,
    y: p.y - other.1
});

impl_ops_ref_copy!(Mul, mul |p: Point, other: i64| Point {
    x: p.x * other,
    y: p.y * other
});

impl_ops_assign_ref_copy!(AddAssign, add_assign |p: Point, other: Point| {
    p.x += other.x;
    p.y += other.y;
});