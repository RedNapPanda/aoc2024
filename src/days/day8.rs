use itertools::Itertools;

pub fn solve1(lines: &Vec<String>) -> i64 {
    count_nodes(lines, false)
}

pub fn solve2(lines: &Vec<String>) -> i64 {
    count_nodes(lines, true)
}

fn count_nodes(lines: &Vec<String>, part2: bool) -> i64 {
    let (height, width) = (lines.len() as i64, lines[0].len() as i64);
    lines
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.char_indices()
                .filter(|&(_, c)| c != '.')
                .map(move |(col, c)| (c, (row as i64, col as i64)))
                .collect::<Vec<_>>()
        })
        // sort keys as chunk_by doesn't work when keys are not next to each other
        // wasted a ton of time here, example doesn't have different antenna non-sequentially
        .sorted_by(|a, b| a.0.cmp(&b.0))
        .chunk_by(|&(c, _)| c)
        .into_iter()
        .map(|(_, group)| {
            group
                .map(|(_, pos)| pos)
                // sorted top_left -> bottom_right
                .sorted_by(|&a, &b| a.cmp(&b))
                .tuple_combinations::<(_, _)>()
                .flat_map(|(a, b)| {
                    let mut vec = if part2 { vec![a, b] } else { vec![] };
                    let step = (b.0 - a.0, b.1 - a.1);
                    let mut a_node = (a.0 - step.0, a.1 - step.1);
                    while is_valid(a_node, height, width) {
                        vec.push(a_node);
                        a_node = (a_node.0 - step.0, a_node.1 - step.1);
                        if !part2 {
                            break;
                        }
                    }
                    let mut b_node = (b.0 + step.0, b.1 + step.1);
                    while is_valid(b_node, height, width) {
                        vec.push(b_node);
                        b_node = (b_node.0 + step.0, b_node.1 + step.1);
                        if !part2 {
                            break;
                        }
                    }
                    vec
                })
        })
        .flatten()
        .unique()
        .count() as i64
}

fn is_valid(node: (i64, i64), height: i64, width: i64) -> bool {
    node.0 >= 0 && node.0 < height && node.1 >= 0 && node.1 < width
}
