use itertools::Itertools;
use std::collections::HashMap;

pub fn solve1(lines: &[String]) -> i64 {
    blink(lines, 25)
}

pub fn solve2(lines: &[String]) -> i64 {
    blink(lines, 75)
}

fn blink(lines: &[String], times: i64) -> i64 {
    let mut state = lines
        .first()
        .iter()
        .flat_map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect_vec()
        })
        .map(|x| (x, 1))
        .collect::<HashMap<_, _>>();
    for _ in 0..times {
        let mut next_state = HashMap::new();
        for (&x, count) in state.iter() {
            if x == 0 {
                let val = next_state.get(&1).unwrap_or(&0) + count;
                next_state.insert(1, val);
                continue;
            }
            let digits = x.ilog10() + 1;
            if digits % 2 == 0 {
                let pow = 10i64.pow(digits / 2);
                let left = x / pow;
                let right = x % pow;
                let val = next_state.get(&left).unwrap_or(&0) + count;
                next_state.insert(left, val);
                let val = next_state.get(&right).unwrap_or(&0) + count;
                next_state.insert(right, val);
                continue;
            }
            let i = x * 2024;
            let val = next_state.get(&i).unwrap_or(&0) + count;
            next_state.insert(i, val);
        }
        state = next_state;
    }
    state.values().sum()
}
