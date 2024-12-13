use crate::utils::grid::Grid;
use crate::utils::point::Point;
use itertools::Itertools;
use std::collections::HashSet;

const DIRECTIONS: [(i64, i64); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

pub fn solve1(lines: &[String]) -> i64 {
    let grid = &Grid::<char>::from(lines);
    plots(grid)
        .iter()
        .map(|(plant, count, plot)| {
            let mut perimeter = 0;
            for pos in plot {
                for dir in DIRECTIONS {
                    if !plot.contains(&(pos + dir)) {
                        perimeter += 1;
                    }
                }
            }
            perimeter * count
        })
        .sum()
}

pub fn solve2(lines: &[String]) -> i64 {
    let grid = &Grid::<char>::from(lines);
    plots(grid)
        .iter()
        .map(|(plant, count, plot)| {
            let mut corners = 0;
            for pos in plot {
                for i in 0..4 {
                    let dir = DIRECTIONS[i];
                    let next_dir = if i < 3 {
                        DIRECTIONS[i + 1]
                    } else {
                        DIRECTIONS[0]
                    };
                    if grid.get(&(pos + dir)).is_some_and(|c| c == plant) {
                        continue;
                    }
                    if grid.get(&(pos + next_dir)).is_none_or(|c| c != plant)
                        || grid
                            .get(&(pos + dir + next_dir))
                            .is_some_and(|c| c == plant)
                    {
                        corners += 1
                    }
                }
            }
            corners * count
        })
        .sum()
}

fn plots(grid: &Grid<char>) -> Vec<(char, i64, Vec<Point>)> {
    let seen = &mut HashSet::new();
    let mut plots = Vec::new();
    for (pos, &plant) in grid.iter_enumerate() {
        if seen.contains(&pos) {
            continue;
        }
        let area = &mut HashSet::new();
        let count = dfs(grid, plant, pos, area);
        if area.is_empty() {
            continue;
        }
        seen.extend(area.clone());
        plots.push((
            plant,
            count,
            area.iter()
                .cloned()
                .unique()
                .sorted_unstable()
                .collect_vec(),
        ));
    }
    plots
}

fn dfs(grid: &Grid<char>, plant: char, pos: Point, area: &mut HashSet<Point>) -> i64 {
    match grid.get(&pos) {
        Some(&plot) if plot == plant => {
            let mut count = 1;
            area.insert(pos.clone());
            for dir in DIRECTIONS {
                let next = &pos + dir;
                if !area.contains(&next) {
                    count += dfs(grid, plant, next, area)
                }
            }
            count
        }
        _ => 0,
    }
}
