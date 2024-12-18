use crate::utils::node::Node;
use itertools::Itertools;
use std::fmt::{Debug, Display, Formatter, Result};
use std::ops::{Index, IndexMut};
use std::slice::Iter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<T> {
    pub rows: Vec<Vec<T>>,
}

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut res = Ok(());
        for row in &self.rows {
            res = res.and_then(|_| writeln!(f, "{}", row.iter().join("")));
        }
        res
    }
}

impl<T> Grid<T> {
    pub fn get(&self, point: &Node) -> Option<&T> {
        match self.contains(point) {
            true => Some(&self.rows[point.x as usize][point.y as usize]),
            false => None,
        }
    }

    pub fn set(&mut self, point: &Node, value: T) {
        if self.contains(point) {
            self.rows[point.x as usize][point.y as usize] = value;
        }
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

    pub fn neighbors_cardinal(&self, point: &Node) -> [Node; 4] {
        [point.left(), point.up(), point.right(), point.down()]
    }
    pub fn neighbors_all(&self, point: &Node) -> [Node; 8] {
        [
            point.left(),
            point.left_up(),
            point.up(),
            point.right_up(),
            point.right(),
            point.right_down(),
            point.down(),
            point.left_down(),
        ]
    }

    pub fn iter_enumerate(&self) -> impl Iterator<Item = (Node, &T)> + '_ {
        self.iter().enumerate().flat_map(|(x, row)| {
            row.iter()
                .enumerate()
                .map(move |(y, v)| (Node::from((x as i64, y as i64)), v))
        })
    }
}

impl<T> Grid<T>
where
    T: Default + Clone,
{
    pub fn with_dimensions(height: usize, width: usize) -> Self {
        Self {
            rows: vec![vec![T::default(); width]; height],
        }
    }

    pub fn reset_defaults(&mut self) {
        for i in 0..self.height() {
            for j in 0..self.width() {
                self.rows[i][j] = T::default()
            }
        }
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = Vec<T>;
    type IntoIter = std::vec::IntoIter<Vec<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.rows.into_iter()
    }
}

pub trait Contains<T> {
    fn contains(&self, other: T) -> bool;
}

impl<T> Contains<&Node> for Grid<T> {
    fn contains(&self, other: &Node) -> bool {
        other.x >= 0
            && other.x < self.height() as i64
            && other.y >= 0
            && other.y < self.width() as i64
    }
}

impl<T> Contains<(i64, i64)> for Grid<T> {
    fn contains(&self, other: (i64, i64)) -> bool {
        self.contains(&Node::from(other))
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.rows[index]
    }
}

impl<T> IndexMut<usize> for Grid<T>
where
    T: Default,
{
    fn index_mut(&mut self, index: usize) -> &mut Vec<T> {
        &mut self.rows[index]
    }
}

impl From<&[String]> for Grid<char> {
    fn from(vec: &[String]) -> Self {
        Self {
            rows: vec
                .iter()
                .map(|line| line.chars().collect_vec())
                .collect_vec(),
        }
    }
}

impl From<&[String]> for Grid<usize> {
    fn from(vec: &[String]) -> Self {
        Self {
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
