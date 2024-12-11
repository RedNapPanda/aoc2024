use crate::utils::grid::Grid;
use crate::utils::traits::Contains;
use itertools::Itertools;

pub fn solve1(lines: &[String]) -> i64 {
    count_nodes(lines, false)
}

pub fn solve2(lines: &[String]) -> i64 {
    count_nodes(lines, true)
}

fn count_nodes(lines: &[String], part2: bool) -> i64 {
    let grid = &Grid::<char>::from(lines);
    grid.iter_enumerate()
        .filter(|(_, &c)| c != '.')
        // sort keys as chunk_by doesn't work when keys are not next to each other
        // wasted a ton of time here, example doesn't have different antenna non-sequentially
        .sorted_by(|a, b| a.1.cmp(b.1))
        .chunk_by(|(_, &c)| c)
        .into_iter()
        .flat_map(|(_, group)| {
            group
                .map(|(pos, _)| pos)
                // sorted top_left -> bottom_right
                .sorted_by(|a, b| a.cmp(b))
                .tuple_combinations::<(_, _)>()
                .flat_map(|(a, b)| {
                    let mut vec = vec![];
                    let step = &b - &a;
                    let mut a_node = &a - &step;
                    while grid.contains(&a_node) {
                        vec.push(a_node.clone());
                        a_node = a_node - &step;
                        if !part2 {
                            break;
                        }
                    }
                    let mut b_node = &b + &step;
                    while grid.contains(&b_node) {
                        vec.push(b_node.clone());
                        b_node = b_node + &step;
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
        .unique()
        .count() as i64
}
