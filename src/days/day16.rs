use crate::utils::algo::astar::astar;
use crate::utils::direction::Direction;
use crate::utils::grid::Grid;
use crate::utils::node::Node;
use crate::utils::algo::PathResult;
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

fn parse(lines: &[String]) -> Grid<D16Tile> {
    Grid {
        rows: lines
            .iter()
            .map(|line| line.chars().flat_map(D16Tile::try_from).collect_vec())
            .collect_vec(),
    }
}

fn find_paths(grid: &Grid<D16Tile>, part2: bool) -> Option<PathResult<(Node<i64>, Direction), i64>> {
    astar(
        &(grid.find(D16Tile::Reindeer).unwrap(), Direction::East),
        |(node, dir)| {
            grid.neighbors_cardinal(node)
                .iter()
                .filter_map(|neighbor| {
                    let new_dir = Direction::from(neighbor - node);
                    if new_dir == dir.inverse()
                        || grid
                        .get(neighbor)
                        .is_some_and(|tile| tile != &D16Tile::Empty && tile != &D16Tile::End)
                    {
                        return None;
                    }
                    Some((neighbor.clone(), new_dir))
                })
                .collect_vec()
        },
        |prev, cur| 1 + (1000 * (cur.1 != prev.1) as i64),
        |_, _| 0,
        |(node, _)| grid.get(node).is_some_and(|tile| tile == &D16Tile::End),
        part2,
    )
}

#[derive(Debug, PartialEq, Clone)]
enum D16Tile {
    Reindeer,
    End,
    Empty,
    Wall,
}

impl Display for D16Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.into())
    }
}

impl From<&D16Tile> for char {
    fn from(tile: &D16Tile) -> char {
        match tile {
            D16Tile::Reindeer => 'S',
            D16Tile::End => 'E',
            D16Tile::Empty => '.',
            D16Tile::Wall => '#',
        }
    }
}

impl TryFrom<char> for D16Tile {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'S' => Ok(D16Tile::Reindeer),
            'E' => Ok(D16Tile::End),
            '.' => Ok(D16Tile::Empty),
            '#' => Ok(D16Tile::Wall),
            _ => Err(()),
        }
    }
}
