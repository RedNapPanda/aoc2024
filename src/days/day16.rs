use crate::utils::astar::{astar, AStarResult};
use crate::utils::direction::Direction;
use crate::utils::grid::Grid;
use crate::utils::node::Node;
use itertools::Itertools;
use std::fmt::{Display, Formatter, Write};

pub fn solve1(lines: &[String]) -> i64 {
    Grid::parse(lines)
        .find_paths()
        .map_or(0, |result| result.cost)
}

pub fn solve2(lines: &[String]) -> i64 {
    Grid::parse(lines).find_paths().map_or(0, |result| {
        result
            .collect()
            .iter()
            .flatten()
            .map(|n| n.0.clone())
            .unique()
            .count()
    }) as i64
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

    fn find_paths(&self) -> Option<AStarResult<(Node, Direction), i64>> {
        astar(
            &(self.start(), Direction::East),
            |(node, dir)| {
                self.neighbors_cardinal(node)
                    .iter()
                    .filter_map(|neighbor| {
                        let new_dir = Direction::from(neighbor - node);
                        if new_dir == dir.inverse()
                            || self
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
            |(node, _)| self.get(node).is_some_and(|tile| tile == &Tile::End),
        )
    }
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
