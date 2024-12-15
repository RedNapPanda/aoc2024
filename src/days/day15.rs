use crate::utils::grid::Contains;
use crate::utils::grid::Grid;
use crate::utils::point::Point;
use itertools::Itertools;
use std::collections::HashSet;
use std::iter;
use strum_macros::Display;

pub fn solve1(lines: &[String]) -> i64 {
    let check = |&c: &char| c == 'O';
    let (mut grid, movements) = parse(lines, false);
    let mut robot = grid.find_robot();
    for movement in &movements {
        let inverse = &movement.vector().inverse();
        //find next open spot
        let next_open = grid.next_open(robot.clone(), movement, check);
        if next_open.is_none() {
            continue;
        }
        let (dist, mut next_pos) = next_open.clone().unwrap();
        for _ in (0i64..dist) {
            let prev_pos = &next_pos + inverse;
            let tmp = *grid.get(&next_pos).unwrap();
            grid.set(&next_pos,*grid.get(&prev_pos).unwrap());
            grid.set(&prev_pos, tmp);
            next_pos = prev_pos;
        }
        robot += movement.vector();
    }
    grid
        .iter_enumerate()
        .filter_map(|(point, char)| match char {
            'O' => Some(point),
            _ => None,
        })
        .map(|pos| pos.x * 100 + pos.y)
        .sum()
}

pub fn solve2(lines: &[String]) -> i64 {
    let check = |&c: &char| c == '[' || c == ']';
    let (mut grid, movements) = parse(lines, true);
    let mut robot = grid.find_robot();
    let boxes = &mut grid
        .iter_enumerate()
        .filter_map(|(point, char)| match char {
            '[' => Some(point),
            _ => None,
        })
        .collect::<HashSet<_>>();
    println!("{}", grid);
    0
}

fn parse(lines: &[String], part2: bool) -> (Grid<char>, Vec<Direction>) {
    let newline = lines.iter().position(|l| l.is_empty()).unwrap();
    let grid_lines = &lines[..newline];
    let grid = if !part2 {
        Grid::from(grid_lines)
    } else {
        let rows = grid_lines
            .iter()
            .map(|s| {
                s.chars()
                    .flat_map(|c| match c {
                        '#' => ['#', '#'],
                        'O' => ['[', ']'],
                        '@' => ['@', '.'],
                        '.' => ['.', '.'],
                        _ => unimplemented!(),
                    })
                    .collect_vec()
            })
            .collect_vec();
        Grid { rows }
    };
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
    fn find_robot(&self) -> Point {
        self
            .iter_enumerate()
            .find_map(|(point, char)| match char {
                '@' => Some(point),
                _ => None,
            })
            .unwrap()
    }

    fn next_open(&self, start: Point, direction: &Direction, check: fn(&char) -> bool) -> Option<(i64, Point)> {
        let vector = &direction.vector();
        (1i64..)
            .map(|i| {
                let pos = &start + vector * i;
                let c = self.get(&pos);
                (pos, c)
            })
            .enumerate()
            .take_while_inclusive(|(_, (_, c))| c.is_some_and(check))
            .last()
            .and_then(|(i, (p, c))| {
                let dist = (i + 1) as i64;
                match c {
                    Some('#') => None,
                    _ => Some((dist, p)),
                }
            })
    }
}
