use crate::utils::grid::Grid;
use crate::utils::node::Node;
use itertools::Itertools;
use std::collections::HashSet;

const DIRECTIONS: [(i64, i64); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

pub fn solve1(lines: &[String]) -> i64 {
    let grid = &Grid::<char>::from(lines);
    plots(grid)
        .iter()
        .map(|(area, plot)| {
            let perimeter = plot
                .iter()
                .flat_map(|pos| DIRECTIONS.iter().map(move |&dir| pos + dir))
                .filter(|pos| !plot.contains(pos))
                .count() as i64;
            perimeter * area
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
            let corners = plot
                .iter()
                .map(|pos| {
                    (0..4)
                        .filter(|&i| {
                            let dir = DIRECTIONS[i];
                            let next_dir = DIRECTIONS.get(i + 1).unwrap_or(&DIRECTIONS[0]);
                            if grid.get(&(pos + dir)).is_some_and(is_plant) {
                                return false;
                            }
                            let transposed = grid.get(&(pos + next_dir)).is_none_or(is_not_plant);
                            let corner = grid.get(&(pos + dir + next_dir)).is_some_and(is_plant);
                            transposed || corner
                        })
                        .count() as i64
                })
                .sum::<i64>();
            corners * area
        })
        .sum()
}

fn plots(grid: &Grid<char>) -> Vec<(i64, Vec<Node<i64>>)> {
    let seen = &mut HashSet::new();
    let mut plots = Vec::new();
    for (pos, &plant) in grid.enumerate() {
        if seen.contains(&pos) {
            continue;
        }
        let area = &mut HashSet::new();
        let count = dfs(grid, plant, pos, area);
        if area.is_empty() {
            continue;
        }
        seen.extend(area.clone());
        plots.push((count, area.iter().cloned().unique().collect_vec()));
    }
    plots
}

fn dfs(grid: &Grid<char>, plant: char, pos: Node<i64>, area: &mut HashSet<Node<i64>>) -> i64 {
    match grid.get(&pos) {
        Some(&plot) if plot == plant => {
            area.insert(pos.clone());
            DIRECTIONS
                .iter()
                .map(|dir| {
                    let pos = &pos + dir;
                    if area.contains(&pos) {
                        return 0;
                    }
                    dfs(grid, plant, pos, area)
                })
                .sum::<i64>()
                + 1
        }
        _ => 0,
    }
}
