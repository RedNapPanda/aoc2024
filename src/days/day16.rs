use crate::utils::algo::astar::astar;
use crate::utils::direction::Direction;
use crate::utils::grid::Grid;
use crate::utils::node::Node;
use crate::utils::PathResult;
use itertools::Itertools;
use std::fmt::{Display, Formatter, Write};

pub fn solve1(lines: &[String]) -> i64 {
    let grid = &parse(lines);
    find_paths(grid, false).map_or(0, |result| result.cost)
}

pub fn solve2(lines: &[String]) -> i64 {
    let grid = &parse(lines);
    find_paths(grid, true).map_or(0, |result| {
        result
            .collect()
            .iter()
            .flatten()
            .map(|n| n.0.clone())
            .unique()
            .count()
    }) as i64
}

fn parse(lines: &[String]) -> Grid<Tile> {
    Grid {
        rows: lines
            .iter()
            .map(|line| line.chars().flat_map(Tile::try_from).collect_vec())
            .collect_vec(),
    }
}

fn start(grid: &Grid<Tile>) -> Node {
    grid.iter_enumerate()
        .find(|&(_, tile)| *tile == Tile::Reindeer)
        .map(|(p, _)| p)
        .unwrap()
}

fn find_paths(grid: &Grid<Tile>, part2: bool) -> Option<PathResult<(Node, Direction), i64>> {
    astar(
        &(start(grid), Direction::East),
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
        |prev, cur| 1 + (1000 * (cur.1 != prev.1) as i64),
        |_, _| 0,
        |(node, _)| grid.get(node).is_some_and(|tile| tile == &Tile::End),
        part2,
    )
}

#[derive(Debug, PartialEq, Clone)]
enum Tile {
    Reindeer,
    End,
    Empty,
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
            Tile::Reindeer => 'S',
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
            'S' => Ok(Tile::Reindeer),
            'E' => Ok(Tile::End),
            '.' => Ok(Tile::Empty),
            '#' => Ok(Tile::Wall),
            _ => Err(()),
        }
    }
}
