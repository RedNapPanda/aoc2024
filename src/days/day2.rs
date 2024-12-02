fn is_valid(levels: &Vec<i64>) -> bool {
    let dir = levels[0] < levels[1];
    levels
        .iter()
        .skip(1)
        .fold((levels[0], true), |(l, res), r| {
            (*r, res && l != *r && (l < *r) == dir && (l - r).abs() < 4)
        }).1
}

pub fn solve1(lines: &Vec<String>) -> Option<i64> {
    let result = lines
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|levels| is_valid(&levels))
        .count() as i64;
    Some(result)
}

pub fn solve2(lines: &Vec<String>) -> Option<i64> {
    let result = lines
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|levels| {
            (0..levels.len()).any(|i| {
                let mut clone = levels.clone();
                clone.remove(i);
                is_valid(&clone)
            })
        })
        .count() as i64;
    Some(result)
}
