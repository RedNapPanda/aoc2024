use crate::utils::point::Point;
use crate::utils::traits::Contains;
use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};
use std::slice::Iter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<T> {
    pub rows: Vec<Vec<T>>,
}

impl<T> Display for Grid<T>
where
    T: Display + std::fmt::Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut res = Ok(());
        for row in &self.rows {
            res = res.and_then(|_| writeln!(f, "{:?}", row));
        }
        res
    }
}

impl<T> Grid<T> {
    pub fn get(&self, point: &Point) -> Option<&T> {
        match self.contains(point) {
            true => Some(&self.rows[point.x as usize][point.y as usize]),
            false => None,
        }
    }
    
    pub fn set(&mut self, point: &Point, value: T) {
        self.rows[point.x as usize][point.y as usize] = value;
    }

    pub fn height(&self) -> usize {
        self.rows.len()
    }

    pub fn width(&self) -> usize {
        self.rows.first().map_or(0, |r| r.len())
    }

    pub fn iter(&self) -> Iter<'_, Vec<T>> {
        self.rows.iter()
    }

    pub fn iter_enumerate(&self) -> impl Iterator<Item = (Point, &T)> + '_ {
        self.iter()
            .flatten()
            .enumerate()
            // reindex the flattened enumeration into Points on Grid
            .map(|(i, v)| (Point::from((i / self.height(), i % self.width())), v))
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = Vec<T>;
    type IntoIter = std::vec::IntoIter<Vec<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.rows.into_iter()
    }
}

impl<T> Contains<&Point> for Grid<T> {
    fn contains(&self, other: &Point) -> bool {
        other.x >= 0
            && other.x < self.height() as i64
            && other.y >= 0
            && other.y < self.width() as i64
    }
}

impl<T> Contains<(i64, i64)> for Grid<T> {
    fn contains(&self, other: (i64, i64)) -> bool {
        self.contains(&Point::from(other))
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.rows[index]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Vec<T> {
        &mut self.rows[index]
    }
}

impl From<&[String]> for Grid<char> {
    fn from(vec: &[String]) -> Grid<char> {
        Grid {
            rows: vec
                .iter()
                .map(|line| line.chars().collect_vec())
                .collect_vec(),
        }
    }
}

impl From<&[String]> for Grid<usize> {
    fn from(vec: &[String]) -> Grid<usize> {
        Grid {
            rows: vec
                .iter()
                .map(|line| {
                    line.chars()
                        .flat_map(|c| c.to_digit(10).map(|c| c as usize))
                        .collect_vec()
                })
                .collect_vec(),
        }
    }
}
