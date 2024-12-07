use itertools::Itertools;
use std::collections::HashSet;

pub fn solve1(lines: &Vec<String>) -> i64 {
    let (graph, start) = &build_graph(lines);
    walk(graph, (start.0, start.1))
        .0
        .iter()
        .map(|(p, _)| p)
        .unique()
        .count() as i64
}

pub fn solve2(lines: &Vec<String>) -> i64 {
    let (graph, start) = &mut build_graph(lines);
    let (path, _) = walk(graph, (start.0, start.1));
    path.into_iter()
        .map(|(pos, _)| pos)
        .unique()
        .filter(|&(row, col)| {
            if graph[row as usize][col as usize] == '.' {
                graph[row as usize][col as usize] = '#';
            }
            let (_, looped) = walk(&graph, (start.0, start.1));
            graph[row as usize][col as usize] = '.';
            looped
        })
        .count() as i64
}

fn build_graph(lines: &Vec<String>) -> (Vec<Vec<char>>, (i32, i32)) {
    let mut pos: (i32, i32) = (0, 0);
    let graph = lines
        .iter()
        .enumerate()
        .map(|(row, line)| {
            line.char_indices()
                .inspect(|&(col, c)| {
                    if c == '^' {
                        pos = (row as i32, col as i32)
                    }
                })
                .map(|(_, c)| c)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (graph, pos)
}

fn is_valid(graph: &Vec<Vec<char>>, pos: (i32, i32)) -> bool {
    pos.0 >= 0 && pos.0 < graph.len() as i32 && pos.1 >= 0 && pos.1 < graph[0].len() as i32
}

fn rot90(pos: (i32, i32)) -> (i32, i32) {
    match pos {
        (0, 1) => (1, 0),
        (0, -1) => (-1, 0),
        (1, 0) => (0, -1),
        (-1, 0) => (0, 1),
        _ => panic!(),
    }
}

fn walk(graph: &Vec<Vec<char>>, mut pos: (i32, i32)) -> (HashSet<((i32, i32), (i32, i32))>, bool) {
    let mut dir = (-1, 0);
    let mut seen = HashSet::new();
    let mut fast = pos;
    let mut fast_dir = dir;
    while is_valid(graph, pos) {
        let next = (pos.0 + dir.0, pos.1 + dir.1);
        for _ in 0..2 {
            let contains = seen.contains(&(fast, fast_dir));
            if !is_valid(graph, fast) || contains {
                return (seen, contains);
            }
            seen.insert((fast, fast_dir));
            let fast_next = (fast.0 + fast_dir.0, fast.1 + fast_dir.1);
            if is_valid(graph, fast_next)
                && graph[fast_next.0 as usize][fast_next.1 as usize] == '#'
            {
                fast_dir = rot90(fast_dir);
                continue;
            }
            fast = fast_next;
        }
        if is_valid(graph, next) && graph[next.0 as usize][next.1 as usize] == '#' {
            dir = rot90(dir);
        } else {
            pos = next;
        }
    }
    (seen, false)
}
