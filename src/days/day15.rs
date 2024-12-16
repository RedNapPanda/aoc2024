use crate::utils::grid::Grid;
use crate::utils::point::Point;
use itertools::Itertools;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

pub fn solve1(lines: &[String]) -> i64 {
    let (mut grid, movements) = parse(lines, false);
    grid.walk_robot(&movements, 'O')
}

pub fn solve2(lines: &[String]) -> i64 {
    let (mut grid, movements) = parse(lines, true);
    grid.walk_robot(&movements, '[')
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
    fn walk_robot(&mut self, movements: &Vec<Direction>, box_char: char) -> i64 {
        let mut robot = self.find_robot();
        for movement in movements {
            self.shift_boxes(&mut robot, movement);
        }
        self.iter_enumerate()
            .filter_map(|(point, &c)| if c == box_char { Some(point) } else { None })
            .map(|pos| pos.x * 100 + pos.y)
            .sum()
    }

    fn find_robot(&self) -> Point {
        self.iter_enumerate()
            .find_map(|(point, char)| match char {
                '@' => Some(point),
                _ => None,
            })
            .unwrap()
    }

    fn shift_boxes(&mut self, robot: &mut Point, movement: &Direction) {
        let (can_push, stack_opt) = self.boxes_to_move(movement, robot);
        if !can_push {
            return;
        }
        if let Some(mut stack) = stack_opt {
            stack.insert((robot.clone(), robot.clone() + movement.vector()));
            stack
                .iter()
                .sorted_unstable_by(|a, b| match movement {
                    Direction::Left => a.0.y.cmp(&b.0.y),
                    Direction::Right => b.0.y.cmp(&a.0.y),
                    Direction::Up => a.0.x.cmp(&b.0.x),
                    Direction::Down => b.0.x.cmp(&a.0.x),
                })
                .for_each(|(from, to)| {
                    let tmp = *self.get(to).unwrap();
                    self.set(to, *self.get(from).unwrap());
                    self.set(from, tmp);
                });
            *robot += movement.vector();
        }
    }

    fn boxes_to_move(
        &mut self,
        direction: &Direction,
        pos: &Point,
    ) -> (bool, Option<HashSet<(Point, Point)>>) {
        const INVALID: (bool, Option<HashSet<(Point, Point)>>) = (false, None);
        if self.get(pos).is_none_or(|&c| c == '#') {
            return INVALID;
        }
        let mut stack = HashSet::new();
        let vector = direction.vector();
        let next = pos + &vector;
        let next_char = self.get(&next);
        match next_char {
            Some('O') => {
                let (can_push, next_stack) = self.boxes_to_move(direction, &next);
                if !can_push {
                    return INVALID;
                }
                if let Some(list) = next_stack {
                    stack.extend(list);
                }
                stack.insert((next.clone(), next + &vector));
            }
            Some('[') | Some(']') => match direction {
                Direction::Left | Direction::Right => {
                    let other_half = &next + &vector;
                    let after_box = &other_half + &vector;
                    let (push_half, half_boxes) = self.boxes_to_move(direction, &other_half);
                    if !push_half {
                        return INVALID;
                    }
                    if let Some(list) = half_boxes {
                        stack.extend(list);
                    }
                    stack.insert((other_half.clone(), after_box));
                    stack.insert((next.clone(), other_half.clone()));
                }
                Direction::Up | Direction::Down => {
                    let mut other_half = next.right();
                    if let Some(']') = next_char {
                        other_half = next.left();
                    }
                    let after_half = &next + &vector;
                    let after_other = &other_half + &vector;
                    let (push_half, half_boxes) = self.boxes_to_move(direction, &next);
                    let (push_other, other_boxes) = self.boxes_to_move(direction, &other_half);
                    if !push_half || !push_other {
                        return INVALID;
                    }
                    if let Some(list) = half_boxes {
                        stack.extend(list);
                    }
                    if let Some(list) = other_boxes {
                        stack.extend(list);
                    }
                    stack.insert((other_half.clone(), after_other.clone()));
                    stack.insert((next.clone(), after_half.clone()));
                }
            },
            Some('.') => return (true, Some(stack)),
            _ => return INVALID,
        }
        (true, Some(stack))
    }
}
