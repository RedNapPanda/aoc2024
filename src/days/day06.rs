use crate::utils::grid::{Contains, Grid};
use crate::utils::node::Node;
use itertools::Itertools;
use std::collections::HashSet;

pub fn solve1(lines: &[String]) -> i64 {
    let (grid, start) = with_start(lines);
    walk(&grid, start)
        .0
        .into_iter()
        .map(|(pos, _)| pos)
        .unique()
        .count() as i64
}

pub fn solve2(lines: &[String]) -> i64 {
    let (mut grid, start) = with_start(lines);
    let unique = walk(&grid, start.clone())
        .0
        .into_iter()
        .map(|(pos, _)| pos)
        .unique()
        .collect_vec();
    unique
        .iter()
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

fn with_start(lines: &[String]) -> (Grid<char>, Node<i64>) {
    let grid = Grid::from(lines);
    let start = grid
        .iter_enumerate()
        .find(|(_, &c)| c == '^')
        .map(|(p, _)| p)
        .unwrap();
    (grid, start)
}

type SeenNodes = HashSet<(Node<i64>, Node<i64>)>;
fn walk(grid: &Grid<char>, pos: Node<i64>) -> (SeenNodes, bool) {
    let dir = Node::from((-1, 0));
    let mut seen = HashSet::new();
    let mut node = (pos.clone(), dir.clone());
    let mut fast = (pos, dir);
    while grid.contains(&node.0) {
        let next = &node.0 + &node.1;
        for _ in 0..2 {
            let contains = seen.contains(&fast);
            if !grid.contains(&fast.0) || contains {
                return (seen, contains);
            }
            seen.insert(fast.clone());
            let fast_next = &fast.0 + &fast.1;
            if grid.contains(&fast_next) && grid[fast_next.x as usize][fast_next.y as usize] == '#'
            {
                fast.1 = fast.1.rot90_cw();
                continue;
            }
            fast.0 = fast_next;
        }
        if grid.contains(&next) && grid[next.x as usize][next.y as usize] == '#' {
            node.1 = node.1.rot90_cw();
        } else {
            node.0 = next;
        }
    }
    (seen, false)
}
