use itertools::Itertools;
use crate::utils::point::Point;

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
                .map(move |(col, c)| (c, Point::from((row, col))))
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
                .sorted_by(|a, b| a.cmp(&b))
                .tuple_combinations::<(_, _)>()
                .flat_map(|(a, b)| {
                    let mut vec = vec![];
                    let step = &b - &a;
                    let mut a_node = &a - &step;
                    while is_valid(&a_node, height, width) {
                        vec.push(a_node.clone());
                        a_node = &a_node - &step;
                        if !part2 {
                            break;
                        }
                    }
                    let mut b_node = &b + &step;
                    while is_valid(&b_node, height, width) {
                        vec.push(b_node.clone());
                        b_node = &b_node + &step;
                        if !part2 {
                            break;
                        }
                    }
                    if part2 {
                        vec.push(a);
                        vec.push(b);
                    }
                    vec
                })
        })
        .flatten()
        .unique()
        .count() as i64
}

fn is_valid(node: &Point, height: i64, width: i64) -> bool {
    node.x >= 0 && node.x < height && node.y >= 0 && node.y < width
}
