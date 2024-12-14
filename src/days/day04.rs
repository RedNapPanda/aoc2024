use crate::utils::grid::Contains;
use crate::utils::grid::Grid;
use crate::utils::point::Point;

const XMAS_ENUM: [Xmas; 8] = [
    Xmas::Right,
    Xmas::Left,
    Xmas::Down,
    Xmas::Up,
    Xmas::RightDown,
    Xmas::RightUp,
    Xmas::LeftDown,
    Xmas::LeftUp,
];

pub fn solve1(lines: &[String]) -> i64 {
    let grid = &Grid::<char>::from(lines);
    grid.iter_enumerate()
        .filter_map(|(pos, &x)| {
            if x != 'X' {
                return None;
            }
            let xmas = XMAS_ENUM
                .into_iter()
                .filter(|x| {
                    let (p1, p2, p3) = x.points_in_direction(&pos);
                    grid.contains(&p3)
                        && grid.get(&p1).is_some_and(|&c| c == 'M')
                        && grid.get(&p2).is_some_and(|&c| c == 'A')
                        && grid.get(&p3).is_some_and(|&c| c == 'S')
                })
                .count() as i64;
            Some(xmas)
        })
        .sum()
}

pub fn solve2(lines: &[String]) -> i64 {
    let grid = &Grid::<char>::from(lines);
    grid.iter_enumerate()
        .filter(|(pos, &x)| {
            let left_down = pos.left_down();
            let left_up = pos.left_up();
            let right_down = pos.right_down();
            let right_up = pos.right_up();
            if x != 'A'
                || !grid.contains(&left_down)
                || !grid.contains(&left_up)
                || !grid.contains(&right_down)
                || !grid.contains(&right_up)
            {
                return false;
            }

            let left_down_c = *grid.get(&left_down).unwrap();
            let left_up_c = *grid.get(&left_up).unwrap();
            let right_down_c = *grid.get(&right_down).unwrap();
            let right_up_c = *grid.get(&right_up).unwrap();

            left_up_c != right_down_c
                && left_down_c != right_up_c
                && (left_down_c == 'M' && right_up_c == 'S'
                    || left_down_c == 'S' && right_up_c == 'M')
                && (left_up_c == 'M' && right_down_c == 'S'
                    || left_up_c == 'S' && right_down_c == 'M')
        })
        .count() as i64
}

enum Xmas {
    Right,
    Left,
    Down,
    Up,
    RightDown,
    RightUp,
    LeftDown,
    LeftUp,
}

impl Xmas {
    fn points_in_direction(&self, pos: &Point) -> (Point, Point, Point) {
        match self {
            Xmas::Right => (pos + (0, 1), pos + (0, 2), pos + (0, 3)),
            Xmas::Left => (pos + (0, -1), pos + (0, -2), pos + (0, -3)),
            Xmas::Down => (pos + (1, 0), pos + (2, 0), pos + (3, 0)),
            Xmas::Up => (pos + (-1, 0), pos + (-2, 0), pos + (-3, 0)),
            Xmas::LeftDown => (pos + (1, -1), pos + (2, -2), pos + (3, -3)),
            Xmas::LeftUp => (pos + (-1, -1), pos + (-2, -2), pos + (-3, -3)),
            Xmas::RightDown => (pos + (1, 1), pos + (2, 2), pos + (3, 3)),
            Xmas::RightUp => (pos + (-1, 1), pos + (-2, 2), pos + (-3, 3)),
        }
    }
}
