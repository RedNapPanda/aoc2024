use crate::utils::grid::{Contains, Grid};
use crate::utils::point::Point;
use itertools::Itertools;
use std::collections::HashSet;

pub fn solve1(lines: &[String]) -> i64 {
    let (grid, start) = Grid::with_start(lines);
    grid.walk(start)
        .0
        .iter()
        .map(|(pos, _)| pos)
        .unique()
        .count() as i64
}

pub fn solve2(lines: &[String]) -> i64 {
    let (grid, start) = &mut Grid::with_start(lines);
    grid.walk(start.clone())
        .0
        .into_iter()
        .map(|(pos, _)| pos)
        .unique()
        .filter(|pos| {
            if grid[pos.x as usize][pos.y as usize] == '.' {
                grid[pos.x as usize][pos.y as usize] = '#';
            }
            let (_, looped) = grid.walk(start.clone());
            grid[pos.x as usize][pos.y as usize] = '.';
            looped
        })
        .count() as i64
}

fn rot90_point(pos: Point) -> Point {
    match pos {
        Point { x: 0, y: 1 } => Point { x: 1, y: 0 },
        Point { x: 0, y: -1 } => Point { x: -1, y: 0 },
        Point { x: 1, y: 0 } => Point { x: 0, y: -1 },
        Point { x: -1, y: 0 } => Point { x: 0, y: 1 },
        _ => unreachable!(),
    }
}

impl Grid<char> {
    fn with_start(lines: &[String]) -> (Self, Point) {
        let grid = &Grid::from(lines);
        let start = grid
            .iter_enumerate()
            .find(|(_, &c)| c == '^')
            .map(|(p, _)| p)
            .unwrap();
        (grid.clone(), start)
    }

    fn walk(&self, mut pos: Point) -> (HashSet<(Point, Point)>, bool) {
        let mut dir = <Point as From<(i64, i64)>>::from((-1, 0));
        let mut seen = HashSet::new();
        let mut fast = pos.clone();
        let mut fast_dir = dir.clone();
        while self.contains(&pos) {
            let next = &pos + &dir;
            for _ in 0..2 {
                let contains = seen.contains(&(fast.clone(), fast_dir.clone()));
                if !self.contains(&fast) || contains {
                    return (seen, contains);
                }
                seen.insert((fast.clone(), fast_dir.clone()));
                let fast_next = &fast + &fast_dir;
                if self.contains(&fast_next)
                    && self[fast_next.x as usize][fast_next.y as usize] == '#'
                {
                    fast_dir = rot90_point(fast_dir);
                    continue;
                }
                fast = fast_next;
            }
            if self.contains(&next) && self[next.x as usize][next.y as usize] == '#' {
                dir = rot90_point(dir);
            } else {
                pos = next.clone();
            }
        }
        (seen, false)
    }
}
