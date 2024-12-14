use rustc_hash::FxHashMap;
use std::cmp;

pub fn solve1(lines: &[String]) -> i64 {
    let (mut l_list, mut r_list): (Vec<_>, Vec<_>) = parse(lines).unzip();
    l_list.sort_unstable();
    r_list.sort_unstable();
    l_list
        .into_iter()
        .zip(r_list)
        .map(|(l, r)| cmp::max(l, r) - cmp::min(l, r))
        .sum()
}

pub fn solve2(lines: &[String]) -> i64 {
    let mut count_map = FxHashMap::<i64, (bool, i64)>::default();
    parse(lines).for_each(|(l, r)| {
        count_map.entry(l).or_default().0 = true;
        count_map.entry(r).or_default().1 += 1;
    });
    count_map
        .into_iter()
        .filter(|(_, (seen, _))| *seen)
        .map(|(l, (_, c))| l * c)
        .sum()
}

fn parse(lines: &[String]) -> impl Iterator<Item = (i64, i64)> + '_ {
    lines
        .iter()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(l, r)| (l.parse::<i64>().unwrap(), r.parse::<i64>().unwrap()))
}
