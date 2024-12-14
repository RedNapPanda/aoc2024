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
            let (target, nums) = line
                .split(":")
                .map(|split| split.trim())
                .collect_tuple()
                .unwrap();
            let target = target.parse::<i64>().unwrap();
            let nums = nums
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect_vec();
            (target, nums)
        })
        .filter(|(target, nums)| is_possible(nums, *target, concat))
        .map(|(target, _)| target)
        .sum()
}

fn is_possible(nums: &[i64], target: i64, concat: bool) -> bool {
    let i = match nums.len() {
        0 => return false,
        1 => return target == nums[0],
        x => x - 1,
    };
    let num = nums[i];
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
        return false;
    }
    is_possible(&nums[..i], target - num, concat)
}
