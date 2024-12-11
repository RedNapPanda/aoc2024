use crate::utils::point::Point;
use crate::utils::traits::Contains;
use std::ops::{Index, IndexMut};
use std::slice::Iter;
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<T>
where
    T: Copy,
{
    pub rows: Vec<Vec<T>>,
}

impl<T: Copy> Grid<T> {
    pub fn height(&self) -> usize {
        self.rows.len()
    }

    pub fn width(&self) -> usize {
        self.rows.get(0).map_or(0, |r| r.len())
    }

    pub fn iter(&self) -> Iter<'_, Vec<T>> {
        self.rows.iter()
    }

    pub fn iter_enumerate(&self) -> impl Iterator<Item=(Point, T)> + '_ {
        self.iter()
            .flatten()
            .enumerate()
            // reindex the flattened enumeration into Points on Grid
            .map(|(i, v)| (Point::from((i / self.height(), i % self.width())), *v))
    }
}

impl<T: Copy> IntoIterator for Grid<T> {
    type Item = Vec<T>;
    type IntoIter = std::vec::IntoIter<Vec<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.rows.into_iter()
    }
}

impl<T: Copy> Contains<&Point> for Grid<T> {
    fn contains(&self, other: &Point) -> bool {
        other.x >= 0
            && other.x < self.height() as i64
            && other.y >= 0
            && other.y < self.width() as i64
    }
}

impl<T: Copy> Contains<(i64, i64)> for Grid<T> {
    fn contains(&self, other: (i64, i64)) -> bool {
        self.contains(&Point::from(other))
    }
}

impl From<&Vec<String>> for Grid<char> {
    fn from(vec: &Vec<String>) -> Grid<char> {
        Grid {
            rows: vec
                .iter()
                .map(|line| line.chars().collect_vec())
                .collect_vec(),
        }
    }
}

impl<T: Copy> Index<usize> for Grid<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.rows[index]
    }
}

impl<T: Copy> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Vec<T> {
        &mut self.rows[index]
    }
}
