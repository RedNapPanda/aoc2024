use crate::utils::grid::Grid;
use crate::utils::node::Node;
use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::collections::VecDeque;

type PathMap = FxHashMap<(char, char), String>;

pub fn solve1(lines: &[String]) -> i64 {
    solve(lines, 3)
}

pub fn solve2(lines: &[String]) -> i64 {
    solve(lines, 26)
}

fn solve(lines: &[String], depth: i64) -> i64 {
    let paths = &build_paths();
    let cache = &mut FxHashMap::default();
    let mut sum = 0;
    for input in lines {
        let code = input[..input.len() - 1].parse::<i64>().unwrap();
        let cost = sequence(paths, cache, input, depth);
        sum += code * cost;
    }
    sum
}

fn build_paths() -> FxHashMap<(char, char), String> {
    // literally using the grid struct to make pathing easier to avoid the invalid spot
    let mut paths = Grid::from(&[
        ['7', '8', '9'],
        ['4', '5', '6'],
        ['1', '2', '3'],
        ['_', '0', 'A'],
    ]).npad_paths();
    let dpad_paths = Grid::from(&[
        ['_', '^', 'A'],
        ['<', 'v', '>'],
    ]).dpad_paths();
    paths.extend(dpad_paths);
    paths
}

fn sequence(paths: &PathMap, cache: &mut FxHashMap<(String, i64), i64>, input: &str, depth: i64) -> i64 {
    let key = (input.to_owned(), depth);
    if cache.contains_key(&key) {
        return cache[&key];
    }
    if depth == 0 {
        return input.len() as i64;
    }
    let mut sum = 0;
    let mut queue = VecDeque::from([('A', 0, depth)]);
    while let Some((from, index, depth)) = queue.pop_front() {
        if index == input.len() {
            continue;
        }
        let to = input.chars().nth(index).unwrap();
        if from == to {
            sum += 1;
        } else {
            sum += sequence(paths, cache, &paths[&(from, to)], depth - 1);
        }
        queue.push_back((to, index + 1, depth));
    }
    cache.insert(key, sum);
    sum
}

impl Grid<char> {
    fn npad_paths(&self) -> FxHashMap<(char, char), String> {
        let lookup = self.lookup_table().collect::<FxHashMap<_, _>>();
        let buttons = self.rows.iter().flatten();
        buttons.clone().cartesian_product(buttons)
            .map(|(from, to)| {
                let start = &lookup[&from];
                let end = &lookup[&to];
                let y_diff = end.y - start.y;
                let x_diff = end.x - start.x;
                let left = (x_diff..0).map(|_| "<");
                let right = (0..x_diff).map(|_| ">");
                let down = (0..y_diff).map(|_| "v");
                let up = (y_diff..0).map(|_| "^");

                let path = if self.get(&Node::new(start.x.min(end.x), start.y.max(end.y)))
                    .is_some_and(|t| t == &'_') {
                    up.chain(left)
                        .chain(right)
                        .chain(down)
                        .join("")
                } else {
                    left.chain(up)
                        .chain(down)
                        .chain(right)
                        .join("")
                };
                ((*from, *to), path + "A")
            }).collect::<FxHashMap<_, _>>()
    }

    fn dpad_paths(&self) -> FxHashMap<(char, char), String> {
        let lookup = self.lookup_table().collect::<FxHashMap<_, _>>();
        let buttons = self.rows.iter().flatten();
        buttons.clone().cartesian_product(buttons)
            .map(|(from, to)| {
                let start = &lookup[&from];
                let end = &lookup[&to];
                let x_diff = end.y - start.y;
                let y_diff = end.x - start.x;
                let left = (y_diff..0).map(|_| "<");
                let right = (0..y_diff).map(|_| ">");
                let down = (0..x_diff).map(|_| "v");
                let up = (x_diff..0).map(|_| "^");

                let path = if self.get(&Node::new(start.x.min(end.x), start.y.min(end.y)))
                    .is_some_and(|t| t == &'_') {
                    down.chain(left).chain(right).chain(up).join("")
                } else {
                    left.chain(down).chain(up).chain(right).join("")
                };
                ((*from, *to), path + "A")
            }).collect::<FxHashMap<_, _>>()
    }
}