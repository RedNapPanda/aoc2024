use itertools::Itertools;
use rustc_hash::FxHashSet;
use std::fmt::{Display, Formatter, Write};

pub fn solve1(lines: &[String]) -> i64 {
    let towel_designs = TowelDesigns::from(lines);
    towel_designs
        .designs
        .iter()
        .filter(|design| towel_designs.total_ways(design) > 0)
        .count() as i64
}

pub fn solve2(lines: &[String]) -> i64 {
    let towel_designs = TowelDesigns::from(lines);
    towel_designs
        .designs
        .iter()
        .map(|design| towel_designs.total_ways(design))
        .sum()
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
            matches[i] = self
                .available
                .iter()
                .map(|s| {
                    let len = s.len();
                    if i >= len && s == &design[(i - len)..i] {
                        return matches[i - len];
                    }
                    0
                })
                .sum::<i64>();
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
