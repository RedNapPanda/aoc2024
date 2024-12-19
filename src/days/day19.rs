use rustc_hash::FxHashSet;
use std::fmt::{Display, Formatter, Write};

pub fn solve1(lines: &[String]) -> i64 {
    let towel_designs = TowelDesigns::from(lines);
    let mut count = 0;
    for towel_design in towel_designs.designs.iter() {
        if towel_designs.total_ways(towel_design) > 0 {
            count += 1;    
        }
    }
    count
}

pub fn solve2(lines: &[String]) -> i64 {
    let towel_designs = TowelDesigns::from(lines);
    let mut count = 0;
    for towel_design in towel_designs.designs.iter() {
        count += towel_designs.total_ways(towel_design);
    }
    count
}

#[derive(Debug)]
struct TowelDesigns {
    available: FxHashSet<String>,
    designs: Vec<String>,
}

impl TowelDesigns {
    fn total_ways(&self, design: &str) -> i64 {
        let mut matches = vec![0; design.len() + 1];
        matches[0] = 1;
        for i in 1..=design.len() {
            for pattern in &self.available {
                let p_len = pattern.len();
                if i < p_len {
                    continue;
                }
                if let Some(s) = design.get((i - p_len)..i) {
                    if s == pattern {
                        matches[i] += matches[i - p_len];
                    }
                }
            }
        }
        *matches.last().unwrap()
    }
}

impl From<&[String]> for TowelDesigns {
    fn from(value: &[String]) -> Self {
        let available = value[0]
            .split(", ")
            .map(|s| s.to_owned())
            .collect::<FxHashSet<_>>();
        Self {
            available,
            designs: value[2..]
                .iter()
                .filter(|s| !s.is_empty())
                .cloned()
                .collect(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.into())
    }
}

impl From<&Color> for char {
    fn from(tile: &Color) -> char {
        match tile {
            Color::White => 'w',
            Color::Blue => 'u',
            Color::Black => 'b',
            Color::Red => 'r',
            Color::Green => 'g',
        }
    }
}

impl TryFrom<char> for Color {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'w' => Ok(Color::White),
            'u' => Ok(Color::Blue),
            'b' => Ok(Color::Black),
            'r' => Ok(Color::Red),
            'g' => Ok(Color::Green),
            _ => Err(()),
        }
    }
}
