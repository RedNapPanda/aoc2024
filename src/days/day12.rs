use crate::utils::grid::Grid;
use crate::utils::point::Point;
use itertools::Itertools;
use std::collections::HashSet;

const DIRECTIONS: [(i64, i64); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

pub fn solve1(lines: &[String]) -> i64 {
    let grid = &Grid::<char>::from(lines);
    plots(grid)
        .iter()
        .map(|(area, plot)| {
            plot.iter()
                .flat_map(|pos| DIRECTIONS.iter().map(move |&dir| pos + dir))
                .filter(|pos| !plot.contains(pos))
                .count() as i64 * area
        })
        .sum()
}

pub fn solve2(lines: &[String]) -> i64 {
    let grid = &Grid::<char>::from(lines);
    plots(grid)
        .iter()
        .map(|(area, plot)| {
            let plant = grid.get(&plot[0]).unwrap();
            let is_plant = |c: &char| c == plant;
            let is_not_plant = |c: &char| !is_plant(c);
            plot.iter()
                .flat_map(|pos| (0..4).map(|i| (pos.clone(), i)))
                .filter(|(pos, i)| {
                    let dir = DIRECTIONS[*i];
                    let next_dir = if *i < 3 {
                        DIRECTIONS[i + 1]
                    } else {
                        DIRECTIONS[0]
                    };
                    if grid.get(&(pos + dir)).is_some_and(is_plant) {
                        return false
                    }
                    grid.get(&(pos + next_dir)).is_none_or(is_not_plant)
                        || grid
                        .get(&(pos + dir + next_dir))
                        .is_some_and(is_plant)
                }).count() as i64 * area
        }).sum()
}

fn plots(grid: &Grid<char>) -> Vec<(i64, Vec<Point>)> {
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
