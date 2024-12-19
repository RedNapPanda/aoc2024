use std::ops::Range;
use crate::utils::algo::astar::astar;
use crate::utils::grid::Grid;
use crate::utils::node::Node;
use crate::utils::PathResult;

pub fn solve1(lines: &[String]) -> i64 {
    let size = 71;
    let cutoff = 1024;
    let mut grid = Grid::with_default(size, size, '.');
    grid.set_walls(lines, 0..cutoff);
    if let Some(result) = grid.scan(size) {
        return result.cost as i64
    }
    0
}

pub fn solve2(lines: &[String]) -> i64 {
    let size = 71;
    let cutoff = 1024;
    let mut left = cutoff;
    let mut right = lines.len();
    let mut midpoint = cutoff + ((lines.len() - cutoff) / 2);
    let mut grid = Grid::with_default(size, size, '.');
    let mut first = None;
    while left < midpoint && midpoint < right {
        grid.reset('.');
        grid.set_walls(lines, 0..midpoint);
        if grid.scan(size).is_some() {
            left = midpoint;
            midpoint += (right - midpoint) / 2;
        } else {
            if first.is_none_or(|v| v > midpoint) {
                first = Some(midpoint);
            }
            right = midpoint;
            midpoint = left + (midpoint - left) / 2;
        }
    }
    println!("Solution: {}", lines[first.unwrap()-1]);
    0
}

impl Grid<char> {
    fn set_walls(&mut self, lines: &[String], range: Range<usize>) {
        for line in lines[range].iter() {
            if let Some(pos) = line
                .split_once(',')
                .map(|(c, r)| (r.parse::<i64>().unwrap(), c.parse::<i64>().unwrap()))
            {
                self.set(&Node::from(pos), '#');
            }
        }
    }
    
    fn scan(&self, size: usize) -> Option<PathResult<Node, i32>> {
        astar(
            &Node::new(0, 0),
            |node| {
                self.neighbors_cardinal(node)
                    .into_iter()
                    .filter(|node| self.get(node).is_some_and(|&c| c == '.'))
            },
            |_, _| 1,
            |_, _| 0,
            |node| node.x == (size - 1) as i64 && node.y == (size - 1) as i64,
            false,
        )
    }
}