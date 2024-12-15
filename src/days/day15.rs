use crate::utils::grid::Grid;
use crate::utils::point::Point;
use itertools::Itertools;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

pub fn solve1(lines: &[String]) -> i64 {
    let check = |&c: &char| c == 'O';
    let (mut grid, movements) = parse(lines, false);
    let mut robot = grid.find_robot();
    for movement in &movements {
        if let Some(open) = grid.next_open(robot.clone(), movement, check) {
            grid.move_boxes(movement, &robot, &open);
            robot += movement.vector();
        }
    }
    grid.iter_enumerate()
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
    for movement in &movements {
        if let Some(open) = grid.next_open(robot.clone(), movement, check) {
            let (can_push, stack_opt) = grid.move_double_boxes(movement, &robot, &open);
            if !can_push {
                continue;
            }
            if let Some(mut stack) = stack_opt {
                stack.insert((robot.clone(), &robot + movement.vector()));
                let mut stack = stack.iter()
                    .sorted_unstable_by(|a, b| match movement {
                        Direction::Left => b.0.y.cmp(&a.0.y),
                        Direction::Up => b.0.x.cmp(&a.0.x),
                        Direction::Right => a.0.y.cmp(&b.0.y),
                        Direction::Down => a.0.x.cmp(&b.0.x),
                    })
                    .collect_vec();
                while let Some((from, to)) = stack.pop() {
                    let tmp = *grid.get(to).unwrap();
                    grid.set(to, *grid.get(from).unwrap());
                    grid.set(from, tmp);
                }
                robot += movement.vector();
            }
        }
    }
    grid.iter_enumerate()
        .filter_map(|(point, char)| match char {
            '[' => Some(point),
            _ => None,
        })
        .map(|pos| pos.x * 100 + pos.y)
        .sum()
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

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Left => write!(f, "<"),
            Direction::Right => write!(f, ">"),
            Direction::Up => write!(f, "^"),
            Direction::Down => write!(f, "V"),
        }
    }
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
        self.iter_enumerate()
            .find_map(|(point, char)| match char {
                '@' => Some(point),
                _ => None,
            })
            .unwrap()
    }

    fn next_open(
        &self,
        start: Point,
        direction: &Direction,
        check: fn(&char) -> bool,
    ) -> Option<Point> {
        let vector = &direction.vector();
        (1..)
            .map(|i| {
                let next = &start + vector * i;
                let c = self.get(&next);
                (next, c)
            })
            .take_while_inclusive(|(_, c)| c.is_some_and(check))
            .last()
            .and_then(|(p, c)| match c {
                Some('#') => None,
                _ => Some(p),
            })
    }

    fn move_boxes(&mut self, direction: &Direction, pos: &Point, end: &Point) {
        if pos == end {
            return;
        }
        let vector = direction.vector();
        let next = pos + &vector;
        if let Some(&c) = self.get(&next) {
            if c == 'O' {
                self.move_boxes(direction, &next, end);
            }
            let tmp = *self.get(&next).unwrap();
            self.set(&next, *self.get(pos).unwrap());
            self.set(pos, tmp);
        }
    }

    fn move_double_boxes(
        &mut self,
        direction: &Direction,
        pos: &Point,
        end: &Point,
    ) -> (bool, Option<HashSet<(Point, Point)>>) {
        if pos == end && self.get(pos).is_none_or(|&c| c == '#') {
            return (true, None);
        }
        let mut stack = HashSet::new();
        let vector = direction.vector();
        let next = pos + &vector;
        let next_char = self.get(&next);
        match next_char {
            Some('[') | Some(']') => match direction {
                Direction::Left | Direction::Right => {
                    let other_half = &next + &vector;
                    let after_box = &other_half + &vector;
                    let (push_half, half_boxes) = self.move_double_boxes(direction, &other_half, end);
                    if let Some(list) = half_boxes {
                        if push_half {
                            stack.extend(list);
                        }
                    }
                    if push_half {
                        stack.insert((other_half.clone(), after_box));
                        stack.insert((next.clone(), other_half.clone()));
                    }
                }
                Direction::Up | Direction::Down => {
                    let other_half = if let Some('[') = next_char {
                        next.right()
                    } else if let Some(']') = next_char {
                        next.left()
                    } else {
                        unimplemented!()
                    };
                    let after_half = &next + &vector;
                    let after_other = &other_half + &vector;
                    let (push_half, half_boxes) = self.move_double_boxes(direction, &next, end);
                    let (push_other, other_boxes) =
                        self.move_double_boxes(direction, &other_half, end);
                    if push_half && push_other {
                        if let Some(list) = half_boxes {
                            stack.extend(list);
                        }
                        if let Some(list) = other_boxes {
                            stack.extend(list);
                        }
                        stack.insert((other_half.clone(), after_other.clone()));
                        stack.insert((next.clone(), after_half.clone()));
                    } else {
                        return (false, None)
                    }
                }
            },
            Some('.') => return (true, Some(stack)),
            _ => return (false, None),
        }
        (true, Some(stack))
    }
}
