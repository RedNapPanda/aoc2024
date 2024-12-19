use crate::utils::grid::Grid;
use crate::utils::node::Node;
use itertools::Itertools;
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Write};

pub fn solve1(lines: &[String]) -> i64 {
    let (mut grid, movements) = parse(lines, false);
    walk_robot(&mut grid, &movements, 'O')
}

pub fn solve2(lines: &[String]) -> i64 {
    let (mut grid, movements) = parse(lines, true);
    walk_robot(&mut grid, &movements, '[')
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

// TODO: port to direction.rs
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
            Direction::Left => f.write_char('<'),
            Direction::Right => f.write_char('>'),
            Direction::Up => f.write_char('^'),
            Direction::Down => f.write_char('V'),
        }
    }
}

impl Direction {
    fn vector(&self) -> Node {
        match self {
            Direction::Left => Node { x: 0, y: -1 },
            Direction::Right => Node { x: 0, y: 1 },
            Direction::Up => Node { x: -1, y: 0 },
            Direction::Down => Node { x: 1, y: 0 },
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
fn walk_robot(
    grid: &mut Grid<char>, movements: &Vec<Direction>, box_char: char) -> i64 {
    let mut robot = find_robot(grid);
    for movement in movements {
        shift_boxes(grid, &mut robot, movement);
    }
    grid.iter_enumerate()
        .filter_map(|(point, &c)| if c == box_char { Some(point) } else { None })
        .map(|pos| pos.x * 100 + pos.y)
        .sum()
}

fn find_robot(
    grid: &Grid<char>) -> Node {
    grid.iter_enumerate()
        .find_map(|(point, char)| match char {
            '@' => Some(point),
            _ => None,
        })
        .unwrap()
}

fn shift_boxes(
    grid: &mut Grid<char>, robot: &mut Node, movement: &Direction) {
    let (can_push, stack_opt) = boxes_to_move(grid, movement, robot);
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
                let tmp = *grid.get(to).unwrap();
                grid.set(to, *grid.get(from).unwrap());
                grid.set(from, tmp);
            });
        *robot += movement.vector();
    }
}

fn boxes_to_move(
    grid: &mut Grid<char>,
    direction: &Direction,
    pos: &Node,
) -> (bool, Option<HashSet<(Node, Node)>>) {
    const INVALID: (bool, Option<HashSet<(Node, Node)>>) = (false, None);
    if grid.get(pos).is_none_or(|&c| c == '#') {
        return INVALID;
    }
    let mut stack = HashSet::new();
    let vector = direction.vector();
    let next = pos + &vector;
    let next_char = grid.get(&next);
    match next_char {
        Some('O') => {
            let (can_push, next_stack) = boxes_to_move(grid, direction, &next);
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
                let (push_half, half_boxes) = boxes_to_move(grid, direction, &other_half);
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
                let (push_half, half_boxes) = boxes_to_move(grid, direction, &next);
                let (push_other, other_boxes) = boxes_to_move(grid, direction, &other_half);
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
