use crate::utils::grid::{Contains, Grid};
use crate::utils::node::Node;
use itertools::Itertools;
use std::collections::HashSet;

pub fn solve1(lines: &[String]) -> i64 {
    let (grid, start) = with_start(lines);
    walk_unique(&grid, start.clone()).count() as i64
}

pub fn solve2(lines: &[String]) -> i64 {
    let (mut grid, start) = with_start(lines);
    walk_unique(&grid.clone(), start.clone())
        .filter(|pos| {
            if grid[pos.x as usize][pos.y as usize] == '.' {
                grid[pos.x as usize][pos.y as usize] = '#';
            }
            let (_, looped) = walk(&grid, start.clone());
            grid[pos.x as usize][pos.y as usize] = '.';
            looped
        })
        .count() as i64
}

fn with_start(lines: &[String]) -> (Grid<char>, Node) {
    let grid = &Grid::from(lines);
    let start = grid
        .iter_enumerate()
        .find(|(_, &c)| c == '^')
        .map(|(p, _)| p)
        .unwrap();
    (grid.clone(), start)
}

fn walk_unique(grid: &Grid<char>, start: Node) -> impl Iterator<Item = Node> + '_ {
    walk(grid, start).0.into_iter().map(|(pos, _)| pos).unique()
}

fn walk(grid: &Grid<char>, mut pos: Node) -> (HashSet<(Node, Node)>, bool) {
    let mut dir = Node::from((-1, 0));
    let mut seen = HashSet::new();
    let mut fast = pos.clone();
    let mut fast_dir = dir.clone();
    while grid.contains(&pos) {
        let next = &pos + &dir;
        for _ in 0..2 {
            let contains = seen.contains(&(fast.clone(), fast_dir.clone()));
            if !grid.contains(&fast) || contains {
                return (seen, contains);
            }
            seen.insert((fast.clone(), fast_dir.clone()));
            let fast_next = &fast + &fast_dir;
            if grid.contains(&fast_next)
                && grid[fast_next.x as usize][fast_next.y as usize] == '#'
            {
                fast_dir = fast_dir.rot90_cw();
                continue;
            }
            fast = fast_next;
        }
        if grid.contains(&next) && grid[next.x as usize][next.y as usize] == '#' {
            dir = dir.rot90_cw();
        } else {
            pos = next.clone();
        }
    }
    (seen, false)
}
