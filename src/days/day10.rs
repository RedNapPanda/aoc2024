use crate::utils::grid::Grid;
use crate::utils::node::Node;
use itertools::Itertools;
use std::collections::HashSet;

pub fn solve1(lines: &[String]) -> i64 {
    let grid = &Grid::usize(lines);
    get_trail_heads(grid)
        .iter()
        .map(|trail_head| solve(grid, trail_head, false))
        .sum()
}

pub fn solve2(lines: &[String]) -> i64 {
    let grid = &Grid::usize(lines);
    get_trail_heads(grid)
        .iter()
        .map(|trail_head| solve(grid, trail_head, true))
        .sum()
}

fn solve(grid: &Grid<usize>, trail_head: &Node<i64>, part2: bool) -> i64 {
    let mut count = 0;
    let mut seen = HashSet::new();
    let mut stack = Vec::new();
    stack.push(trail_head.clone());
    while let Some(pos) = stack.pop() {
        if !part2 && !seen.insert(pos.clone()) {
            continue;
        }
        let cur = *grid.get(&pos).unwrap_or(&0);
        if cur == 9 {
            count += 1;
            continue;
        }
        walk(grid, &mut stack, pos, cur);
    }
    count
}

fn get_trail_heads(grid: &Grid<usize>) -> Vec<Node<i64>> {
    grid.enumerate()
        .filter_map(|(p, x)| match x {
            0 => Some(p),
            _ => None,
        })
        .collect_vec()
}

fn walk(grid: &Grid<usize>, stack: &mut Vec<Node<i64>>, pos: Node<i64>, cur: usize) {
    for p in [pos.left(), pos.right(), pos.up(), pos.down()] {
        if let Some(&next) = grid.get(&p) {
            if next > cur && next - cur == 1 {
                stack.push(p);
            }
        }
    }
}
