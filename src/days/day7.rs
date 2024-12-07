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
            let target = split[0].parse::<i64>().unwrap();
            let nums = split[1]
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            (target, nums)
        })
        .filter(|(target, nums)| valid_equation(nums, *target, 1, nums[0], concat))
        .map(|(target, _)| target)
        .sum()
}

fn valid_equation(nums: &Vec<i64>, target: i64, i: usize, val: i64, concat: bool) -> bool {
    if i >= nums.len() {
        return target == val
    }
    let mut result = false;
    let mul = nums[i] * val;
    if !result && mul <= target {
        result |= valid_equation(nums, target, i+1, mul, concat)
    }
    let sum = nums[i] + val;
    if !result && sum <= target {
        result |= valid_equation(nums, target, i+1, sum, concat)
    }
    if !result && concat {
        let combined = (val * 10_i64.pow(nums[i].ilog10() + 1)) + nums[i];
        if combined <= target {
            result |= valid_equation(nums, target, i+1, combined, concat)
        }
    }
    result
}
