use crate::utils::point::Point;
use crate::utils::traits::Contains;
use std::slice::Iter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<T>
where
    T: Copy,
{
    pub rows: Vec<Vec<T>>,
}

impl<T: Copy> Grid<T> {
    pub fn iter(&self) -> Iter<'_, Vec<T>> {
        self.rows.iter()
    }

    pub fn iter_enumerate(&self) -> impl Iterator<Item = (Point, T)> + '_ {
        self.iter().flatten().enumerate().map(|(i, &ref v)| {
            (
                Point::from((i / self.height() as usize, i % self.width() as usize)),
                v.clone(),
            )
        })
    }
}

impl<T: Copy> IntoIterator for Grid<T> {
    type Item = Vec<T>;
    type IntoIter = std::vec::IntoIter<Vec<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.rows.into_iter()
    }
}

pub trait Height<T> {
    fn height(&self) -> T;
}

pub trait Width<T> {
    fn width(&self) -> T;
}

impl<T: Copy> Height<i64> for Grid<T> {
    fn height(&self) -> i64 {
        self.rows.len() as i64
    }
}

impl<T: Copy> Width<i64> for Grid<T> {
    fn width(&self) -> i64 {
        self.rows.get(0).map_or(0, |r| r.len()) as i64
    }
}

impl<T: Copy> Contains<&Point> for Grid<T> {
    fn contains(&self, other: &Point) -> bool {
        other.x < 0 || other.x >= self.height() || other.y < 0 || other.y >= self.width()
    }
}

impl<T: Copy> Contains<(i64, i64)> for Grid<T> {
    fn contains(&self, other: (i64, i64)) -> bool {
        self.contains(&<Point as From<(i64, i64)>>::from(other))
    }
}

impl From<&Vec<String>> for Grid<char> {
    fn from(vec: &Vec<String>) -> Grid<char> {
        Grid {
            rows: vec
                .iter()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        }
    }
}
