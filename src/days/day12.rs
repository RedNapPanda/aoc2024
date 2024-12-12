use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use crate::utils::grid::Grid;
use crate::utils::point::Point;
use crate::utils::traits::Contains;

pub fn solve1(lines: &[String]) -> i64 {
    let grid = &Grid::<char>::from(lines);
    let seen = &mut HashSet::new();
    let mut plots = Vec::new();
    for (pos, &plant) in grid.iter_enumerate() {
        if seen.contains(&pos) {
            continue
        }
        let area = &mut HashSet::new();
        dfs(grid, plant, pos, area);
        if area.is_empty() {
            continue
        }
        seen.extend(area.clone());
        plots.push((plant, area.iter().cloned().sorted_unstable().collect::<HashSet<_>>()));
    }
    plots.iter()
        .map(|(_, plot)| {
            let mut perimeter = 0;
            for pos in plot {
                for dir in DIRECTIONS {
                    if !plot.contains(&(pos + dir)) {
                        perimeter += 1;
                    }
                }
            }
            perimeter * plot.len() as i64
        })
        .sum()
}

pub fn solve2(lines: &[String]) -> i64 {
    0
}

const DIRECTIONS: [(i64, i64); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];
fn dfs(grid: &Grid<char>, plant: char, pos: Point, area: &mut HashSet<Point>) {
    match grid.get(&pos) {
        Some(&plot) if plot == plant => {
            area.insert(pos.clone());
            for dir in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
                let next = &pos + dir;
                if !area.contains(&next) {
                    dfs(grid, plant, next, area)
                }
            }
        },
        _ => ()
    }
}