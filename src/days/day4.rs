use itertools::Itertools;

type Point = (i64, i64);

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
    let mut x_locs = Vec::new();
    let graph = &lines
        .iter()
        .enumerate()
        .map(|(row, line)| {
            let _x_locs = &line
                .chars()
                .enumerate()
                .filter(|&(_, c)| c == 'X')
                .map(|(col, _)| (row as i64, col as i64))
                .collect_vec();
            x_locs.extend_from_slice(_x_locs);
            line.chars().collect_vec()
        })
        .collect_vec();
    x_locs
        .into_iter()
        .map(|(row, col)| {
            XMAS_ENUM
                .into_iter()
                .map(|x| x.shift())
                .filter(|&((r1, c1), (r2, c2), (r3, c3))| {
                    inbound(graph, (row + r3, col + c3))
                        && graph[(row + r1) as usize][(col + c1) as usize] == 'M'
                        && graph[(row + r2) as usize][(col + c2) as usize] == 'A'
                        && graph[(row + r3) as usize][(col + c3) as usize] == 'S'
                })
                .count() as i64
        })
        .sum()
}

pub fn solve2(lines: &[String]) -> i64 {
    let mut a_locs = Vec::new();
    let graph = &lines
        .iter()
        .enumerate()
        .map(|(row, line)| {
            let _a_locs = &line
                .chars()
                .enumerate()
                .filter(|&(_, c)| c == 'A')
                .map(|(col, _)| (row as i64, col as i64))
                .collect_vec();
            a_locs.extend_from_slice(_a_locs);
            line.chars().collect_vec()
        })
        .collect_vec();
    a_locs
        .into_iter()
        .filter(|&pos| {
            let left_down = Xmas::LeftDown.point(pos);
            let left_up = Xmas::LeftUp.point(pos);
            let right_down = Xmas::RightDown.point(pos);
            let right_up = Xmas::RightUp.point(pos);

            if !inbound(graph, left_down)
                || !inbound(graph, left_up)
                || !inbound(graph, right_down)
                || !inbound(graph, right_up)
            {
                return false;
            }

            let left_down = graph[left_down.0 as usize][left_down.1 as usize];
            let left_up = graph[left_up.0 as usize][left_up.1 as usize];
            let right_down = graph[right_down.0 as usize][right_down.1 as usize];
            let right_up = graph[right_up.0 as usize][right_up.1 as usize];

            left_up != right_down
                && left_down != right_up
                && (left_down == 'M' && right_up == 'S' || left_down == 'S' && right_up == 'M')
                && (left_up == 'M' && right_down == 'S' || left_up == 'S' && right_down == 'M')
        })
        .count() as i64
}

fn inbound(graph: &[Vec<char>], pos: Point) -> bool {
    let (row, col) = pos;
    row >= 0 && row < graph.len() as i64 && col >= 0 && col < graph[0].len() as i64
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
    fn point(&self, pos: Point) -> Point {
        let (row, col) = pos;
        match self {
            Xmas::LeftDown => (row - 1, col + 1),
            Xmas::LeftUp => (row - 1, col - 1),
            Xmas::RightDown => (row + 1, col + 1),
            Xmas::RightUp => (row + 1, col - 1),
            _ => (row, col),
        }
    }

    fn shift(&self) -> (Point, Point, Point) {
        match self {
            Xmas::Right => ((0, 1), (0, 2), (0, 3)),
            Xmas::Left => ((0, -1), (0, -2), (0, -3)),
            Xmas::Down => ((1, 0), (2, 0), (3, 0)),
            Xmas::Up => ((-1, 0), (-2, 0), (-3, 0)),
            Xmas::LeftDown => ((1, -1), (2, -2), (3, -3)),
            Xmas::LeftUp => ((-1, -1), (-2, -2), (-3, -3)),
            Xmas::RightDown => ((1, 1), (2, 2), (3, 3)),
            Xmas::RightUp => ((-1, 1), (-2, 2), (-3, 3)),
        }
    }
}
