use itertools::Itertools;

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
            let split = line
                .split(":")
                .map(|split| split.trim())
                .collect_vec();
            let target = split[0].parse::<i64>().unwrap();
            let nums = split[1]
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            (target, nums)
        })
        .filter(|(target, nums)| is_possible(nums, *target, nums.len(), concat))
        .map(|(target, _)| target)
        .sum()
}

fn ten_pow_digits(val: i64) -> i64 {
    10_i64.pow(val.ilog10() + 1)
}

fn is_possible(nums: &Vec<i64>, mut target: i64, len: usize, concat: bool) -> bool {
    for i in (0..len).rev() {
        if i == 0 {
            return target == nums[0];
        }
        let num = nums[i];
        if (target % num == 0 && is_possible(nums, target / num, i, concat))
            || (concat
            && target % ten_pow_digits(num) == num
            && is_possible(nums, target / ten_pow_digits(num), i, concat))
        {
            return true;
        } else if target <= num {
            return false;
        }
        target -= num;
    }
    false
}
