use crate::utils::grid::Grid;
use crate::utils::point::Point;
use itertools::Itertools;
use std::collections::HashSet;

pub fn solve1(lines: &[String]) -> i64 {
    let grid = Grid::<usize>::from(lines);
    let trail_heads = get_trail_heads(&grid);
    trail_heads
        .iter()
        .map(|trail_head| {
            let mut count = 0;
            let mut seen = HashSet::new();
            let mut stack = Vec::new();
            stack.push(trail_head.clone());
            while let Some(pos) = stack.pop() {
                if seen.contains(&pos) {
                    continue;
                }
                let cur = *grid.get(&pos).unwrap_or(&0);
                seen.insert(pos.clone());
                if cur == 9 {
                    count += 1;
                    continue;
                }
                walk(&grid, &mut stack, pos, cur);
            }
            count
        })
        .sum()
}

pub fn solve2(lines: &[String]) -> i64 {
    let grid = Grid::<usize>::from(lines);
    let trail_heads = get_trail_heads(&grid);
    trail_heads
        .iter()
        .map(|trail_head| {
            let mut count = 0;
            let mut stack = Vec::new();
            stack.push(trail_head.clone());
            while let Some(pos) = stack.pop() {
                let prev = *grid.get(&pos).unwrap_or(&0);
                if prev == 9 {
                    count += 1;
                    continue;
                }
                walk(&grid, &mut stack, pos, prev);
            }
            count
        })
        .sum()
}

fn get_trail_heads(grid: &Grid<usize>) -> Vec<Point> {
    grid.iter_enumerate()
        .filter_map(|(p, x)| match x {
            0 => Some(p),
            _ => None,
        })
        .collect_vec()
}

fn walk(grid: &Grid<usize>, stack: &mut Vec<Point>, pos: Point, cur: usize) {
    for p in [pos.left(), pos.right(), pos.up(), pos.down()] {
        if let Some(&next) = grid.get(&p) {
            if next > cur && next - cur == 1 {
                stack.push(p);
            }
        }
    }
}
