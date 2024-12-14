use itertools::Itertools;

pub fn solve1(lines: &[String]) -> i64 {
    let mut files = files(lines);
    let mut right = files.len() - 1;
    for i in 0..files.len() {
        if right <= i || files[i] != usize::MAX {
            continue;
        }
        while files[right] == usize::MAX {
            right -= 1;
        }
        files.swap(i, right);
        right -= 1;
    }
    sum(files)
}

pub fn solve2(lines: &[String]) -> i64 {
    let mut files = files(lines);
    let chunks = files
        .iter()
        .enumerate()
        .chunk_by(|&(_, &x)| x)
        .into_iter()
        .filter_map(|(x, mut chunk)| match x {
            usize::MAX => None,
            _ => chunk.next().map(|(i, _)| (i, chunk.count() + 1)),
        })
        .collect_vec();
    (0..chunks.len()).rev().for_each(|i| {
        let (idx, file_len) = chunks[i];
        let mut width = 0;
        let mut left_ptr = 0;
        while left_ptr < idx && width < file_len {
            let mut width_ptr = left_ptr;
            while width_ptr < idx && files[width_ptr] == usize::MAX {
                width_ptr += 1;
            }
            width = width_ptr - left_ptr;
            if width < file_len {
                left_ptr = width_ptr;
                while left_ptr < idx && files[left_ptr] != usize::MAX {
                    left_ptr += 1;
                }
            }
        }
        if left_ptr < idx {
            for i in 0..file_len {
                files.swap(left_ptr + i, idx + i);
            }
        }
    });
    sum(files)
}

fn files(lines: &[String]) -> Vec<usize> {
    lines
        .first()
        .unwrap_or(&String::from(""))
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .enumerate()
        .flat_map(|(i, x)| {
            (0..x).map(move |_| match i % 2 {
                0 => (i + 1) / 2,
                _ => usize::MAX,
            })
        })
        .collect_vec()
}

fn sum(files: Vec<usize>) -> i64 {
    files
        .iter()
        .enumerate()
        .filter(|&(_, &x)| x != usize::MAX)
        .map(|(i, &x)| i * x)
        .sum::<usize>() as i64
}
