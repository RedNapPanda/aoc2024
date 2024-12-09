use std::collections::VecDeque;

pub fn solve1(lines: &Vec<String>) -> i64 {
    // todo: doing zero optimization compression for this
    let vec = lines
        .first()
        .unwrap_or(&String::from(""))
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect::<Vec<i64>>();
    let mut count = 0i64;
    let mut files = vec
        .iter()
        .enumerate()
        .flat_map(|(i, &x)| {
            let res = (0..x)
                .into_iter()
                .map(move |_| if i % 2 == 0 { count } else { -1 });
            if i % 2 == 0 {
                count += 1;
            }
            res
        })
        .collect::<Vec<_>>();
    let empty_spaces = &mut files
        .iter()
        .enumerate()
        .filter(|(_, &d)| d == -1)
        .map(|(i, _)| i)
        .collect::<VecDeque<_>>();
    while !empty_spaces.is_empty() {
        let mut right_file = files.len() - 1;
        while right_file > 0 && files[right_file] == -1 {
            right_file -= 1;
        }
        let left_empty = empty_spaces.pop_front().unwrap();
        if left_empty > right_file {
            break;
        }
        files[left_empty] = files[right_file];
        files[right_file] = -1;
        right_file -= 1;
    }
    files
        .iter()
        .filter(|&&x| x != -1)
        .enumerate()
        .map(|(i, &x)| i as i64 * x)
        .sum()
}

pub fn solve2(lines: &Vec<String>) -> i64 {
    0
}
