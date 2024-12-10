use itertools::Itertools;

fn fold_valid<'a, I>(v: i64, dir: bool, iter: I) -> bool
where
    I: Iterator<Item = &'a i64>,
{
    iter.fold((v, true), |(l, res), &r| {
        (r, res && l != r && (l < r) == dir && (l - r).abs() < 4)
    })
    .1
}

fn is_valid_skip(levels: &Vec<i64>, skip: usize) -> bool {
    let mut iter = levels
        .into_iter()
        .enumerate()
        .filter_map(|(i, v)| {
            if i == skip {
                return None;
            }
            Some(v)
        })
        .peekable();
    let a = *iter.next().unwrap();
    let b = **iter.peek().unwrap();
    fold_valid(a, a < b, iter)
}

pub fn solve1(lines: &Vec<String>) -> i64 {
    lines
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect_vec()
        })
        .filter(|levels| {
            let iter = levels.into_iter().skip(1);
            fold_valid(levels[0], levels[0] < levels[1], iter)
        })
        .count() as i64
}

pub fn solve2(lines: &Vec<String>) -> i64 {
    lines
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect_vec()
        })
        .filter(|levels| (0..levels.len()).any(|i| is_valid_skip(&levels, i)))
        .count() as i64
}
