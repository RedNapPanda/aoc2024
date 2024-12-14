use regex::{Captures, Regex};

pub fn solve1(lines: &[String]) -> i64 {
    let regex = &Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    parse(lines, regex)
        .map(|c| c[1].parse::<i64>().unwrap() * c[2].parse::<i64>().unwrap())
        .sum()
}

pub fn solve2(lines: &[String]) -> i64 {
    let regex = &Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    parse(lines, regex)
        .fold((0, true), |(v, toggle), c| match c.get(1) {
            Some(_) => {
                let r = c[1].parse::<i64>().unwrap() * c[2].parse::<i64>().unwrap();
                (v + r * toggle as i64, toggle)
            },
            _ => (v, c[0].len() == 4),
        })
        .0
}

fn parse<'a>(lines: &'a [String], regex: &'a Regex) -> impl Iterator<Item = Captures<'a>> + 'a {
    lines
        .iter()
        .flat_map(move |l| regex.captures_iter(l))
}
