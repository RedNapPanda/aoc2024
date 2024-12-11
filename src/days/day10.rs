use crate::utils::grid::Grid;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

pub fn solve1(lines: &[String]) -> i64 {
    let grid = Grid::<usize>::from(lines);
    let trail_heads = &grid
        .iter_enumerate()
        .filter_map(|(p, x)| match x {
            0 => Some(p),
            _ => None,
        })
        .collect_vec();
    trail_heads
        .iter()
        .map(|trail_head| {
            let mut count = 0;
            let mut seen = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back(trail_head.clone());
            while let Some(pos) = queue.pop_front() {
                if seen.contains(&pos) {
                    continue
                }
                let val = *grid.get(&pos).unwrap_or(&0);
                seen.insert(pos.clone());
                if val == 9 {
                    count += 1;
                    continue
                }
                for p in [pos.left(), pos.right(), pos.up(), pos.down()] {
                    if let Some(&v) = grid.get(&p) {
                        if v > val && v - val == 1 {
                            queue.push_back(p);
                        }
                    }
                }
            }
            count
        })
        .sum()
}

pub fn solve2(lines: &[String]) -> i64 {
    let grid = Grid::<usize>::from(lines);
    let trail_heads = &grid
        .iter_enumerate()
        .filter_map(|(p, x)| match x {
            0 => Some(p),
            _ => None,
        })
        .collect_vec();
    trail_heads
        .iter()
        .map(|trail_head| {
            let mut count = 0;
            let mut queue = VecDeque::new();
            queue.push_back(trail_head.clone());
            while let Some(pos) = queue.pop_front() {
                let val = *grid.get(&pos).unwrap_or(&0);
                if val == 9 {
                    count += 1;
                    continue
                }
                for p in [pos.left(), pos.right(), pos.up(), pos.down()] {
                    if let Some(&v) = grid.get(&p) {
                        if v > val && v - val == 1 {
                            queue.push_back(p);
                        }
                    }
                }
            }
            count
        })
        .sum()
}
