use crate::utils::astar::astar;
use crate::utils::direction::Direction;
use crate::utils::grid::Grid;
use crate::utils::node::Node;
use itertools::Itertools;
use rustc_hash::FxHashSet;
use std::collections::BinaryHeap;
use std::fmt::{Display, Formatter, Write};

pub fn solve1(lines: &[String]) -> i64 {
    let grid = Grid::parse(lines);
    let start = grid.start();
    let mut min_cost = None;
    let mut seen = FxHashSet::default();
    let mut heap = BinaryHeap::new();
    heap.push((0, start, Direction::East));
    while let Some((cost, pos, direction)) = heap.pop() {
        if min_cost.is_some_and(|mc| mc < -cost) {
            continue;
        }
        if !seen.insert((pos.clone(), direction.clone())) {
            continue;
        }
        match grid.get(&pos).unwrap_or(&Tile::Wall) {
            Tile::End => {
                min_cost = Some(-cost);
            }
            Tile::Empty | Tile::Reindeer => {
                heap.push((cost - 1000, pos.clone(), direction.turn_left()));
                heap.push((cost - 1000, pos.clone(), direction.turn_right()));
                heap.push((cost - 1, &pos + direction.vector(), direction));
            }
            _ => {}
        }
    }
    min_cost.unwrap_or(0)
}

pub fn solve2(lines: &[String]) -> i64 {
    let mut grid = Grid::parse(lines);
    let start = grid.start();
    let result = astar(
        &(start, Direction::East),
        |(node, dir)| {
            grid.neighbors_cardinal(node)
                .iter()
                .filter_map(|neighbor| {
                    let new_dir = Direction::from(neighbor - node);
                    if new_dir == dir.inverse()
                        || grid
                            .get(neighbor)
                            .is_some_and(|tile| tile != &Tile::Empty && tile != &Tile::End)
                    {
                        return None;
                    }
                    Some((neighbor.clone(), new_dir))
                })
                .collect_vec()
        },
        |prev, cur| {
            if cur.1 == prev.1 {
                1
            } else {
                1001
            }
        },
        |_, _| 0,
        |(node, _)| grid.get(node).is_some_and(|tile| tile == &Tile::End),
    );

    if let Some((cost, end_nodes, paths)) = result {
        println!("Costs: {}", cost);
        let collect = Vec::new();
        for (node, _) in end_nodes {
            let mut vec = Vec::new();
            vec.push(node);
            loop {
                let parents = match vec.last() {
                    Some(node) => paths.get(node).unwrap().parents,
                    _ => break,
                }
            }
        }
    }
    0
}

impl Grid<Tile> {
    fn parse(lines: &[String]) -> Grid<Tile> {
        Grid {
            rows: lines
                .iter()
                .map(|line| line.chars().flat_map(Tile::try_from).collect_vec())
                .collect_vec(),
        }
    }

    fn start(&self) -> Node {
        self.iter_enumerate()
            .find(|&(_, tile)| *tile == Tile::Reindeer)
            .map(|(p, _)| p)
            .unwrap()
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Tile {
    Reindeer,
    End,
    Empty,
    Wall,
    Steps,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.into())
    }
}

impl From<&Tile> for char {
    fn from(tile: &Tile) -> char {
        match tile {
            Tile::Reindeer => 'S',
            Tile::End => 'E',
            Tile::Empty => '.',
            Tile::Wall => '#',
            Tile::Steps => 'O',
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'S' => Ok(Tile::Reindeer),
            'E' => Ok(Tile::End),
            '.' => Ok(Tile::Empty),
            '#' | '^' | '<' | '>' | 'v' => Ok(Tile::Wall),
            _ => Err(()),
        }
    }
}
