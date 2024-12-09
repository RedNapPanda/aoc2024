use std::ops::{Add, Sub};
use crate::{forward_ref_binop, impl_ops_ref_copy};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64
}

impl_ops_ref_copy!(Add, add |p: Point, other: Point| Point {
    x: p.x + other.x,
    y: p.y + other.y
});

impl_ops_ref_copy!(Sub, sub |p: Point, other: Point| Point {
    x: p.x - other.x,
    y: p.y - other.y
});

impl From<(i64, i64)> for Point {
    fn from((x, y): (i64, i64)) -> Self {
        Point { x, y }
    }
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        <Point as From<(i64, i64)>>::from((x as i64, y as i64))
    }
}