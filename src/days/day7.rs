use itertools::Itertools;
use std::collections::VecDeque;

pub fn solve1(lines: &[String]) -> i64 {
    solve(lines, false)
}

pub fn solve2(lines: &[String]) -> i64 {
    solve(lines, true)
}

fn solve(lines: &[String], concat: bool) -> i64 {
    lines
        .iter()
        .map(|line| {
            let split = line.split(":").map(|split| split.trim()).collect_vec();
            let target = split[0].parse::<i64>().unwrap();
            let nums = split[1]
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            (target, nums)
        })
        .filter(|(target, nums)| is_possible(nums, *target, concat))
        .map(|(target, _)| target)
        .sum()
}

fn is_possible(nums: &[i64], mut target: i64, concat: bool) -> bool {
    for i in (0..nums.len()).rev() {
        let num = nums[i];
        if i == 0 {
            return target == num;
        }
        if target % num == 0 && is_possible(&nums[..i], target / num, concat) {
            return true;
        }
        let ten_pow_digits = 10_i64.pow(num.ilog10() + 1);
        if concat
            && target % ten_pow_digits == num
            && is_possible(&nums[..i], target / ten_pow_digits, concat)
        {
            return true;
        }
        if target <= num {
            break;
        }
        target -= num;
    }
    false
}