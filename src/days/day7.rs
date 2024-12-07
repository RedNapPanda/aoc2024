use std::collections::VecDeque;

pub fn solve1(lines: &Vec<String>) -> i64 {
    solve(lines, false)
}

pub fn solve2(lines: &Vec<String>) -> i64 {
    solve(lines, true)
}

fn solve(lines: &Vec<String>, concat: bool) -> i64 {
    lines
        .iter()
        .map(|line| {
            let split = line
                .split(":")
                .map(|split| split.trim())
                .collect::<Vec<&str>>();
            let test = split[0].parse::<i64>().unwrap();
            let nums = split[1]
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            (test, nums)
        })
        .filter(|(test, nums)| {
            nums[1..]
                .iter()
                .fold(VecDeque::from([nums[0]]), |mut acc, num| {
                    for _ in 0..acc.len() {
                        let prev = acc.pop_front().unwrap();
                        if concat {
                            let ndigits = num.checked_ilog10().unwrap_or(0) + 1;
                            let combined = (prev * 10_i64.pow(ndigits)) + num;
                            acc.push_back(combined);
                        }
                        acc.push_back(prev + num);
                        acc.push_back(prev * num);
                    }
                    acc
                })
                .iter()
                .any(|total| total == test)
        })
        .map(|(test, _)| test)
        .sum()
}
