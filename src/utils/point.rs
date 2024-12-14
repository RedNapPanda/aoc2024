use crate::{forward_ref_binop, impl_ops_ref_copy};
use std::ops::{Add, Sub};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl<T> From<(T, T)> for Point
where T: Into<i64>{
    fn from((x, y): (T, T)) -> Self {
        Self {
            x: x.into() as i64,
            y: y.into() as i64,
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
