use crate::utils::grid::Grid;
use crate::utils::node::Node;
use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::collections::VecDeque;
use std::fmt::Display;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

type PathMap = FxHashMap<(char, char), String>;

pub fn solve1(lines: &[String]) -> i64 {
    solve(lines, 3)
}

pub fn solve2(lines: &[String]) -> i64 {
    solve(lines, 26)
}

fn solve(lines: &[String], depth: i64) -> i64 {
    let paths = &build_paths();
    let cache = &mut FxHashMap::default();
    let mut sum = 0;
    for input in lines {
        let code = input[..input.len()-1].parse::<i64>().unwrap();
        let cost = sequence(paths, cache, input, depth);
        sum += code * cost;
    }
    sum
}

fn build_paths() -> FxHashMap<(char, char), String> {
    // literally using the grid struct to make pathing easier to avoid the invalid spot
    let mut paths = Grid::from(&[
        [Npad::Seven, Npad::Eight, Npad::Nine],
        [Npad::Four, Npad::Five, Npad::Six],
        [Npad::One, Npad::Two, Npad::Three],
        [Npad::Invalid, Npad::Zero, Npad::Activate],
    ]).generate_paths();
    let dpad_paths = Grid::from(&[
        [Dpad::Invalid, Dpad::Up, Dpad::Activate],
        [Dpad::Left, Dpad::Down, Dpad::Right],
    ]).generate_paths();
    paths.extend(dpad_paths);
    paths
}

fn sequence(paths: &PathMap, cache: &mut FxHashMap<(String, i64), i64>, input: &str, depth: i64) -> i64 {
    let key = (input.to_owned(), depth);
    if cache.contains_key(&key) {
        return cache[&key]
    }
    if depth == 0 {
        return input.len() as i64;
    }
    let mut sum = 0;
    let mut queue = VecDeque::from([('A', 0, depth)]);
    while let Some((from, index, depth)) = queue.pop_front() {
        if index == input.len() {
            continue
        }
        let to =  input.chars().nth(index).unwrap();
        if from == to {
            sum += 1;
        } else {
            sum += sequence(paths, cache, &paths[&(from, to)], depth - 1);
        }
        queue.push_back((to, index + 1, depth));
    }
    cache.insert(key, sum);
    sum
}

impl Grid<Npad> {
    fn generate_paths(&self) -> FxHashMap<(char, char), String> {
        let npad_lookup = self.lookup_table().collect::<FxHashMap<_, _>>();
        let npad_buttons = Npad::iter().filter(|t| t != &Npad::Invalid);
        npad_buttons.clone().cartesian_product(npad_buttons)
            .map(|(from, to)| {
                let path = self.shortest_path(&npad_lookup[&from], &npad_lookup[&to]);
                ((char::from(&from), char::from(&to)), path)
            }).collect::<FxHashMap<_, _>>()
    }
    fn shortest_path(&self, start: &Node<i64>, end: &Node<i64>) -> String {
        let x_diff = end.x - start.x;
        let y_diff = end.y - start.y;
        let left = (y_diff..0).map(|_| "<");
        let right = (0..y_diff).map(|_| ">");
        let down = (0..x_diff).map(|_| "v");
        let up = (x_diff..0).map(|_| "^");

        if self.get(&Node::new(start.x.max(end.x), start.y.min(end.y)))
            .is_some_and(|t| t == &Npad::Invalid) {
            up.chain(left)
                .chain(right)
                .chain(down)
                .join("") + "A"
        } else {
            left.chain(up)
                .chain(down)
                .chain(right)
                .join("") + "A"
        }
    }
}

impl Grid<Dpad> {
    fn generate_paths(&self) -> FxHashMap<(char, char), String> {
        let dpad_lookup = self.lookup_table().collect::<FxHashMap<_, _>>();
        let dpad_buttons = Dpad::iter().filter(|t| t != &Dpad::Invalid);
        dpad_buttons.clone().cartesian_product(dpad_buttons)
            .map(|(from, to)| {
                let path = self.shortest_path(&dpad_lookup[&from], &dpad_lookup[&to]);
                ((char::from(&from), char::from(&to)), path)
            }).collect::<FxHashMap<_, _>>()
    }

    fn shortest_path(&self, start: &Node<i64>, end: &Node<i64>) -> String {
        let x_diff = end.x - start.x;
        let y_diff = end.y - start.y;
        let left = (y_diff..0).map(|_| "<");
        let right = (0..y_diff).map(|_| ">");
        let down = (0..x_diff).map(|_| "v");
        let up = (x_diff..0).map(|_| "^");

        if self.get(&Node::new(start.x.min(end.x), start.y.min(end.y)))
            .is_some_and(|t| t == &Dpad::Invalid) {
            down.chain(left)
                .chain(right)
                .chain(up)
                .join("") + "A"
        } else {
            left.chain(down)
                .chain(up)
                .chain(right)
                .join("") + "A"
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, EnumIter)]
enum Npad {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Activate,
    Invalid,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, EnumIter)]
enum Dpad {
    Left,
    Right,
    Up,
    Down,
    Activate,
    Invalid,
}

impl Display for Npad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(self))
    }
}

impl From<&Npad> for char {
    fn from(value: &Npad) -> Self {
        match value {
            Npad::Activate => 'A',
            Npad::Zero => '0',
            Npad::One => '1',
            Npad::Two => '2',
            Npad::Three => '3',
            Npad::Four => '4',
            Npad::Five => '5',
            Npad::Six => '6',
            Npad::Seven => '7',
            Npad::Eight => '8',
            Npad::Nine => '9',
            Npad::Invalid => '_',
        }
    }
}

impl TryFrom<char> for Npad {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let button = match value {
            'A' => Npad::Activate,
            '0' => Npad::Zero,
            '1' => Npad::One,
            '2' => Npad::Two,
            '3' => Npad::Three,
            '4' => Npad::Four,
            '5' => Npad::Five,
            '6' => Npad::Six,
            '7' => Npad::Seven,
            '8' => Npad::Eight,
            '9' => Npad::Nine,
            _ =>   Npad::Invalid,
        };
        Ok(button)
    }
}

impl Display for Dpad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(self))
    }
}

impl From<&Dpad> for char {
    fn from(value: &Dpad) -> Self {
        match value {
            Dpad::Left => '<',
            Dpad::Right => '>',
            Dpad::Up => '^',
            Dpad::Down => 'v',
            Dpad::Activate => 'A',
            Dpad::Invalid => '_',
        }
    }
}

impl TryFrom<char> for Dpad {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let button = match value {
            '<' => Dpad::Left,
            '>' => Dpad::Right,
            '^' => Dpad::Up,
            'v' => Dpad::Down,
            'A' => Dpad::Activate,
            _ => Dpad::Invalid,
        };
        Ok(button)
    }
}