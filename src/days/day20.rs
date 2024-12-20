use std::collections::VecDeque;
use std::fmt::{Display, Formatter, Write};
use itertools::Itertools;
use strum::IntoEnumIterator;
use rustc_hash::{FxHashMap, FxHashSet};
use crate::utils::algo::astar::astar;
use crate::utils::direction::Direction;
use crate::utils::grid::Grid;
use crate::utils::node::Node;
use crate::utils::PathResult;

pub fn solve1(lines: &[String]) -> i64 {
    let grid = &mut Grid::<Tile>::from(lines);
    let start = start(grid);
    if let Some(result) = astar(&start,
                                |node| grid.neighbors_cardinal(node)
                                    .into_iter()
                                    .filter(|n| grid.get(n).is_some_and(|t| t == &Tile::Empty || t == &Tile::End)),
                                |_, _| 1,
                                |_, _| 0,
                                |node| grid.get(node).is_some_and(|t| t == &Tile::End),
                                false,
    ) {
        let mut saves = FxHashMap::default();
        for node in &result.first() {
            let cost = result.visited.get(node).unwrap().cost;
            Direction::iter()
                .for_each(|dir| {
                    let nodes = grid.nodes_in_direction(node, dir, 2);
                    if nodes.len() != 2 {
                        return;
                    }
                    if grid.get(&nodes[0]).is_none_or(|t| t != &Tile::Wall)
                        || grid.get(&nodes[1]).is_none_or(|t| t != &Tile::Empty && t != &Tile::End)
                        || !result.visited.contains_key(&nodes[1]) {
                        return;
                    }

                    let skip_cost = result.visited.get(&nodes[1]).unwrap().cost;
                    let saved = skip_cost - cost - 2;
                    let count = saves.get(&saved).unwrap_or(&0);
                    saves.insert(saved, count + 1);
                })
        }
        return saves.iter().filter_map(|(saved, count)| {
            if *saved >= 100 {
                Some(*count)
            } else {
                None
            }
        }).sum::<i64>();
    }
    0
}

pub fn solve2(lines: &[String]) -> i64 {
    let grid = &mut Grid::<Tile>::from(lines);
    let start = start(grid);
    if let Some(result) = astar(&start,
                                |node| grid.neighbors_cardinal(node)
                                    .into_iter()
                                    .filter(|n| grid.get(n).is_some_and(|t| t == &Tile::Empty || t == &Tile::End)),
                                |_, _| 1i64,
                                |_, _| 0,
                                |node| grid.get(node).is_some_and(|t| t == &Tile::End),
                                false,
    ) {
        let mut count = 0;
        for node in &result.first() {
            let cost = result.visited.get(node).unwrap().cost;
            //todo: bfs to find all empty nodes <= 20 from position
        }
    }
    0
}

fn bfs(grid: &Grid<Tile>, result: PathResult<Node, i64>, start: &Node, cost: i64, limit: i64) -> i64 {
    let mut count = 0;
    let mut pair_seen = FxHashSet::default();
    let mut seen = FxHashSet::default();
    let mut deque = VecDeque::new();
    deque.push_back((start.clone(), 0i64));
    while let Some((node, steps)) = deque.pop_front() {
        if steps > limit {
            continue
        }
        seen.insert(node.clone());
        if let Some(tile) = grid.get(&node) {
            if tile == &Tile::Empty || tile == &Tile::End {
                let pair = (start, node.clone());
                if pair_seen.contains(&pair) {
                    continue
                }
                let skip_cost = result.visited.get(&node).unwrap().cost;
                let saved = skip_cost - cost - steps;
                if saved >= limit {
                    count += 1;
                }
                pair_seen.insert(pair);
            }
            grid.neighbors_cardinal(&node)
                .iter()
                .filter(|&n| !seen.contains(n))
                .for_each(|n| deque.push_back((n.clone(), steps + 1)));
        } else {
            continue
        }
    }
    count
}

fn start(grid: &Grid<Tile>) -> Node {
    grid.iter_enumerate()
        .find(|&(_, tile)| *tile == Tile::Start)
        .map(|(p, _)| p)
        .unwrap()
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
enum Tile {
    Start,
    End,
    Empty,
    #[default]
    Wall,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.into())
    }
}

impl From<&Tile> for char {
    fn from(tile: &Tile) -> char {
        match tile {
            Tile::Start => 'S',
            Tile::End => 'E',
            Tile::Empty => '.',
            Tile::Wall => '#',
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'S' => Ok(Tile::Start),
            'E' => Ok(Tile::End),
            '.' => Ok(Tile::Empty),
            '#' => Ok(Tile::Wall),
            _ => Err(()),
        }
    }
}
