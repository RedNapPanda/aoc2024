use regex::Regex;
use itertools::Itertools;

pub fn solve1(lines: &[String]) -> i64 {
    let towel_designs = TowelDesigns::from(lines);
    let regex = Regex::new(&format!("^({})*$", towel_designs.patterns.iter().join("|"))).unwrap();
    towel_designs
        .designs
        .iter()
        .filter(|design| regex.is_match(design))
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
    patterns: Vec<String>,
    designs: Vec<String>,
}

impl TowelDesigns {
    fn total_ways(&self, design: &str ) -> i64 {
        let mut matches = vec![0; design.len() + 1];
        matches[0] = 1;
        for i in 1..=design.len() {
            matches[i] = self
                .patterns
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
        matches[matches.len() - 1]
    }
}

impl From<&[String]> for TowelDesigns {
    fn from(value: &[String]) -> Self {
        let available = value[0]
            .split(", ")
            .map(|s| s.to_owned())
            .collect_vec();
        Self {
            patterns: available,
            designs: value[2..]
                .iter()
                .filter(|s| !s.is_empty())
                .cloned()
                .collect(),
        }
    }
}
