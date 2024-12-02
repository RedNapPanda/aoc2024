fn is_valid(levels: &Vec<i64>) -> bool {
    let dir = levels[0] < levels[1];
    for i in 1..levels.len() {
        let (l, r) = (levels[i-1], levels[i]);
        if l == r || (l < r) != dir || (l - r).abs() > 3 {
            return false;
        }
    }
    true
}

fn is_valid_skip(levels: &Vec<i64>, skip: usize) -> bool {
    let mut iter = levels
        .into_iter()
        .enumerate()
        .filter(|&(i, _)| i != skip)
        .map(|(_, l)| l)
        .peekable();
    let a = *iter.next().unwrap();
    let b = **iter.peek().unwrap();
    let dir = a < b;
    iter.fold((a, true), |(l, res), r| {
        (*r, res && l != *r && (l < *r) == dir && (l - r).abs() < 4)
    })
    .1
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
        .filter(|levels| (0..levels.len()).any(|i| is_valid_skip(&levels, i)))
        .count() as i64;
    Some(result)
}
