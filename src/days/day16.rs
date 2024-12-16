use crate::utils::grid::Grid;
use crate::utils::point::Point;
use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::fmt::{Display, Formatter, Write};

pub fn solve1(lines: &[String]) -> i64 {
    let grid = &mut parse(lines);
    println!("{}", grid);
    let start = grid
        .iter_enumerate()
        .find(|&(_, tile)| *tile == Tile::Reindeer)
        .map(|(p, _)| p)
        .unwrap();
    let cost = grid.search(start);
    println!("{}", grid);
    cost
}

pub fn solve2(lines: &[String]) -> i64 {
    let grid = parse(lines);
    0
}

fn parse(lines: &[String]) -> Grid<Tile> {
    Grid {
        rows: lines
            .iter()
            .map(|line| line.chars().flat_map(Tile::try_from).collect_vec())
            .collect_vec(),
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn inverse(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
    
    fn turn_left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        }
    }
    
    fn turn_right(&self) -> Self {
        self.turn_left().inverse()
    }
    
    fn vector(&self) -> Point {
        match self {
            Direction::North => Point { x: -1, y: 0 },
            Direction::South => Point { x: 1, y: 0 },
            Direction::East => Point { x: 0, y: 1 },
            Direction::West => Point { x: 0, y: -1 },
        }
    }
}

impl From<&Direction> for char {
    fn from(dir: &Direction) -> Self {
        match dir {
            Direction::North => '^',
            Direction::South => 'v',
            Direction::East => '>',
            Direction::West => '<',
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(char::from(self))
    }
}

impl From<Point> for Direction {
    fn from(value: Point) -> Self {
        match value {
            Point { x: 0, y: 0 } => panic!("Not a vector"),
            Point { x, y: 0 } if x < 0 => Direction::North,
            Point { x, y: 0 } if x > 0 => Direction::South,
            Point { x: 0, y } if y > 0 => Direction::East,
            Point { x: 0, y } if y < 0 => Direction::West,
            _ => panic!("diagonals not implemented"),
        }
    }
}

#[derive(Debug, PartialEq)]
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
            '#' | '^' | '<' | '>' | 'v' => Ok(Tile::Wall),
            _ => Err(()),
        }
    }
}

impl Grid<Tile> {
    fn search(&mut self, origin: Point) -> i64 {
        let mut min_cost = i64::MAX;
        let mut seen = HashSet::new();
        let mut heap = BinaryHeap::new();
        heap.push((0, origin, Direction::East));
        while let Some((cost, pos, direction)) = heap.pop() {
            if !seen.insert((pos.clone(), direction.clone())) {
                continue
            }
            match self.get(&pos).unwrap_or(&Tile::Wall) {
                Tile::End => {
                    min_cost = min_cost.min(-cost);
                },
                Tile::Empty | Tile::Reindeer => {
                    heap.push((cost - 1000, pos.clone(), direction.turn_left()));
                    heap.push((cost - 1000, pos.clone(), direction.turn_right()));
                    heap.push((cost - 1, &pos + direction.vector(), direction));
                },
                _ => {},
            }
        }
        min_cost
    }
}
