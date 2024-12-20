use itertools::Itertools;

pub fn solve1(lines: &[String]) -> i64 {
    build_levels(lines)
        .filter(|levels| is_valid_skip(levels.clone(), None))
        .count() as i64
}

pub fn solve2(lines: &[String]) -> i64 {
    let levels = build_levels(lines).collect_vec();
    levels.iter()
        .filter(|&level| {
            (0..levels.len()).any(|i| is_valid_skip(level.clone(), Some(i)))
        })
        .count() as i64
}
fn build_levels(
    lines: &[String],
) -> impl Iterator<Item = impl Iterator<Item = i64> + Clone + '_> + '_ {
    lines
        .iter()
        .map(|line| line.split_whitespace().map(|s| s.parse::<i64>().unwrap()))
}

fn is_valid_skip<I>(levels: I, skip: Option<usize>) -> bool
where
    I: Iterator<Item = i64> + Clone,
{
    let mut iter = levels
        .clone()
        .enumerate()
        .filter_map(|(i, v)| {
            if skip.is_some_and(|x| x == i) {
                return None;
            }
            Some(v)
        })
        .peekable();
    let a = iter.next().unwrap();
    let b = *iter.peek().unwrap();
    let (_, valid) = iter.fold((a, true), |(l, res), r| {
        (r, res && l != r && (l < r) == (a < b) && (l - r).abs() < 4)
    });
    valid
}
