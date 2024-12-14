use crate::utils::grid::Grid;
use crate::utils::point::Point;
use itertools::Itertools;
use std::collections::HashSet;

pub fn solve1(lines: &[String]) -> i64 {
    let grid = Grid::<usize>::from(lines);
    grid.get_trail_heads()
        .iter()
        .map(|trail_head| grid.solve(trail_head, false))
        .sum()
}

pub fn solve2(lines: &[String]) -> i64 {
    let grid = Grid::<usize>::from(lines);
    grid.get_trail_heads()
        .iter()
        .map(|trail_head| grid.solve(trail_head, true))
        .sum()
}

impl Grid<usize> {
    fn solve(&self, trail_head: &Point, part2: bool) -> i64 {
        let mut count = 0;
        let mut seen = HashSet::new();
        let mut stack = Vec::new();
        stack.push(trail_head.clone());
        while let Some(pos) = stack.pop() {
            if !part2 && !seen.insert(pos.clone()) {
                continue;
            }
            let cur = *self.get(&pos).unwrap_or(&0);
            if cur == 9 {
                count += 1;
                continue;
            }
            self.walk(&mut stack, pos, cur);
        }
        count
    }

    fn get_trail_heads(&self) -> Vec<Point> {
        self.iter_enumerate()
            .filter_map(|(p, x)| match x {
                0 => Some(p),
                _ => None,
            })
            .collect_vec()
    }

    fn walk(&self, stack: &mut Vec<Point>, pos: Point, cur: usize) {
        for p in [pos.left(), pos.right(), pos.up(), pos.down()] {
            if let Some(&next) = self.get(&p) {
                if next > cur && next - cur == 1 {
                    stack.push(p);
                }
            }
        }
    }
}
