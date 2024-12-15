use crate::utils::grid::{Contains, Grid};
use crate::utils::point::Point;
use itertools::Itertools;
use std::collections::HashSet;
use strum_macros::Display;

pub fn solve1(lines: &[String]) -> i64 {
    let (mut grid, movements) = parse(lines);
    let mut robot = grid
        .iter_enumerate()
        .find_map(|(point, char)| match char {
            '@' => Some(point),
            _ => None,
        })
        .unwrap();
    let boxes = &mut grid
        .iter_enumerate()
        .filter_map(|(point, char)| match char {
            'O' => Some(point),
            _ => None,
        })
        .collect::<HashSet<_>>();
    println!("Initial Grid");
    println!("{}", grid);
    for (i, movement) in movements.iter().enumerate() {
        let vector = &movement.vector();
        let inverse = &movement.vector().inverse();
        //find next open spot
        let next_open = grid.next_open(robot.clone(), movement);
        if next_open.is_none() {
            continue;
        }
        let mut next_pos = next_open.unwrap();
        while (&next_pos + vector) != robot {
            let pos = &next_pos + inverse;
            if boxes.contains(&pos) {
                grid.set(&next_pos, 'O');
                boxes.insert(next_pos);
                boxes.remove(&pos);
            } else {
                grid.set(&next_pos, '@');
                robot = next_pos.clone();
            }
            grid.set(&pos, '.');
            next_pos = pos;
        }
    }
    println!("Completed");
    println!("{}", grid);
    boxes.iter()
        .map(|pos| pos.x * 100 + pos.y)
        .sum()
}

pub fn solve2(lines: &[String]) -> i64 {
    0
}

fn parse(lines: &[String]) -> (Grid<char>, Vec<Direction>) {
    let newline = lines.iter().position(|l| l.is_empty()).unwrap();
    let grid = Grid::from(&lines[..newline]);
    let movements = lines[(newline + 1)..]
        .iter()
        .flat_map(|s| s.chars().map(Direction::from))
        .collect_vec();
    (grid, movements)
}

#[derive(Debug, Display)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn vector(&self) -> Point {
        match self {
            Direction::Left => Point { x: 0, y: -1 },
            Direction::Right => Point { x: 0, y: 1 },
            Direction::Up => Point { x: -1, y: 0 },
            Direction::Down => Point { x: 1, y: 0 },
        }
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => unimplemented!(),
        }
    }
}
impl Grid<char> {
    fn next_open(&self, start: Point, direction: &Direction) -> Option<Point> {
        let vector = &direction.vector();
        let mut next = start + vector;
        let mut pos = self.get(&next);
        while pos.is_some_and(|&c| c == 'O') {
            next += vector;
            pos = self.get(&next);
        }
        pos.and_then(|&c| match c {
            '#' => None,
            _ => Some(next),
        })
    }
}
