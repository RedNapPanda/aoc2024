use crate::utils::grid::Grid;
use itertools::Itertools;

pub fn solve1(lines: &[String]) -> i64 {
    let mut spaces = lines.iter().enumerate().filter_map(|(i, line)| {
        if line.is_empty() {
            Some(i)
        } else {
            None
        }
    }).collect_vec();
    spaces.push(lines.len());
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for i in 0..spaces.len() {
        let start = if i == 0 {
            0
        } else {
            *spaces.get(i - 1).unwrap() + 1
        };
        let end = *spaces.get(i).unwrap_or(&(lines.len())) - 1;
        let mut grid = Grid::<char>::from(&lines[start..=end]);
        grid.transpose();
        let counts = grid.rows.iter().map(|row| row.iter().filter(|&c| c == &'#').count()).collect_vec();
        if is_key(&grid) {
            keys.push(counts);
        } else {
            locks.push(counts);
        }
    }

    let mut count = 0;
    for lock in &locks {
        'key: for key in &keys {
            for i in 0..key.len() {
                if lock[i] + key[i] >= 8 {
                    continue 'key;
                }
            }
            count += 1;
        }
    }
    count
}

pub fn solve2(lines: &[String]) -> i64 {
    println!("Finished AoC");
    0
}

fn is_key(grid: &Grid<char>) -> bool {
    grid.rows.first()
        .and_then(|row| row.first().map(|c| c == &'.'))
        .unwrap_or_default()
}