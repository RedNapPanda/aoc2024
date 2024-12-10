use itertools::Itertools;

pub fn solve1(lines: &Vec<String>) -> i64 {
    let mut files = lines
        .first()
        .unwrap_or(&String::from(""))
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .enumerate()
        .flat_map(|(i, x)| {
            let res = (0..x)
                .into_iter()
                .map(move |_| if i % 2 == 0 { (i + 1) / 2 } else { usize::MAX });
            res
        })
        .collect_vec();
    let empty_spaces = files
        .iter()
        .enumerate()
        .filter(|(_, &d)| d == usize::MAX)
        .map(|(i, _)| i)
        .collect_vec();
    let mut count = 0;
    let mut right_file = files.len() - 1;
    while count < empty_spaces.len() && empty_spaces[count] <= right_file {
        while right_file > 0 && files[right_file] == usize::MAX {
            right_file -= 1;
        }
        files[empty_spaces[count]] = files[right_file];
        files[right_file] = usize::MAX;
        right_file -= 1;
        count += 1;
    }
    let file_sys = files
        .iter()
        .map(|&x| match x {
            usize::MAX => String::from("."),
            _ => x.to_string(),
        })
        .join("");
    println!("FileSys: {:?}", file_sys);
    files
        .iter()
        .enumerate()
        .filter(|&(_, &x)| x != usize::MAX)
        .map(|(i, &x)| i * x)
        .sum::<usize>() as i64
}

pub fn solve2(lines: &Vec<String>) -> i64 {
    let mut files = lines
        .first()
        .unwrap_or(&String::from(""))
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .enumerate()
        .flat_map(|(i, x)| {
            let res = (0..x)
                .into_iter()
                .map(move |_| if i % 2 == 0 { (i + 1) / 2 } else { usize::MAX });
            res
        })
        .collect_vec();

    // [(idx, len),...]
    let file_loc = files
        .iter()
        .enumerate()
        .filter(|&(_, &x)| x != usize::MAX)
        .dedup_by_with_count(|(_, x), (_, y)| x == y)
        .into_iter()
        .map(|(c, (i, x))| (i, c))
        .collect::<Vec<_>>();
    // [(idx, len),...]
    let empty_blocks = &mut files
        .iter()
        .enumerate()
        .dedup_by_with_count(|(_, x), (_, y)| x == y)
        .filter(|&(_, (_, &x))| x == usize::MAX)
        .into_iter()
        .map(|(c, (i, x))| (i, c))
        .collect::<Vec<_>>();
    for file_num in (0..file_loc.len()).rev() {
        let (idx, len) = file_loc[file_num];
        let (i, e_idx, e_len) = empty_blocks
            .iter()
            .enumerate()
            .filter(|(i, &(_, e_len))| e_len >= len)
            .next()
            .map(|(i, &(e_idx, e_len))| {
                for i in 0..len {
                    files.swap(e_idx + i, idx + i);
                }
                (i, e_idx + len, e_len - len)
            }).unwrap_or((usize::MAX, 0, 0));
        if i != usize::MAX {
            empty_blocks[i] = (e_idx, e_len);
        }
    }

    let file_sys = files
        .iter()
        .map(|&x| match x {
            usize::MAX => String::from("."),
            _ => x.to_string(),
        })
        .join("");
    println!("FileSys: {:?}", file_sys);

    files
        .iter()
        .enumerate()
        .filter(|&(_, &x)| x != usize::MAX)
        .map(|(i, &x)| i * x)
        .sum::<usize>() as i64
}
