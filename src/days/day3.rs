use regex::Regex;

pub fn solve1(lines: &Vec<String>) -> i64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    lines
        .into_iter()
        .flat_map(|l| re.captures_iter(l.as_str()))
        .map(|c| c[1].parse::<i64>().unwrap() * c[2].parse::<i64>().unwrap())
        .sum()
}

pub fn solve2(lines: &Vec<String>) -> i64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    lines
        .into_iter()
        .flat_map(|l| re.captures_iter(l.as_str()))
        .fold((0, true), |(v, toggle), c| match c.get(1) {
            None => (v, c[0].len() == 4),
            Some(_) => {
                let r = c[1].parse::<i64>().unwrap() * c[2].parse::<i64>().unwrap();
                (v + r * toggle as i64, toggle)
            }
        })
        .0
}
