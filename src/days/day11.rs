use itertools::Itertools;
use rustc_hash::FxHashMap;

pub fn solve1(lines: &[String]) -> i64 {
    blink(lines, 25)
}

pub fn solve2(lines: &[String]) -> i64 {
    blink(lines, 75)
}

fn blink(lines: &[String], times: i64) -> i64 {
    let state = lines
        .first()
        .iter()
        .flat_map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect_vec()
        })
        .map(|x| (x, 1i64))
        .collect::<FxHashMap<_, _>>();
    (0..times)
        .fold(state, |state, _| {
            state.iter()
                .fold(FxHashMap::default(), |mut next_state, (&x, count)| {
                    if x == 0 {
                        let val = next_state.get(&1).unwrap_or(&0) + count;
                        next_state.insert(1, val);
                        return next_state
                    }
                    let digits = x.ilog10() + 1;
                    if digits % 2 == 0 {
                        let pow = 10i64.pow(digits / 2);
                        let left = x / pow;
                        let left_val = next_state.get(&left).unwrap_or(&0) + count;
                        next_state.insert(left, left_val);
                        let right = x % pow;
                        let right_val = next_state.get(&right).unwrap_or(&0) + count;
                        next_state.insert(right, right_val);
                        return next_state
                    }
                    let i = x * 2024;
                    let val = next_state.get(&i).unwrap_or(&0) + count;
                    next_state.insert(i, val);
                    next_state
                })
        })
        .values()
        .sum::<i64>()
}
