use itertools::Itertools;
use std::cmp::Ordering;
use rustc_hash::{FxHashMap, FxHashSet};

pub fn solve1(lines: &[String]) -> i64 {
    let (graph, updates) = build_graph(lines);
    let mut seen = FxHashSet::default();
    updates
        .iter()
        .map(|update| update.split(",").collect_vec())
        .filter(|update| is_valid(update, &graph, &mut seen))
        .map(|update| update[update.len() / 2].parse::<i64>().unwrap())
        .sum()
}

pub fn solve2(lines: &[String]) -> i64 {
    let (graph, updates) = build_graph(lines);
    let mut seen = FxHashSet::default();
    updates
        .iter()
        .map(|update| update.split(",").collect_vec())
        .filter(|update| !is_valid(update, &graph, &mut seen))
        .map(|mut update| {
            update.sort_unstable_by(
                |&a, &b| match graph.contains_key(a) && graph[a].contains(b) {
                    true => Ordering::Less,
                    _ => Ordering::Greater,
                },
            );
            update[update.len() / 2].parse::<i64>().unwrap()
        })
        .sum()
}

fn build_graph(lines: &[String]) -> (FxHashMap<&str, FxHashSet<&str>>, Vec<String>) {
    let break_line = lines.iter().position(|l| l.is_empty()).unwrap();
    let mut graph = FxHashMap::default();
    lines[..break_line]
        .iter()
        .map(|rule| rule.split_once("|").unwrap())
        .for_each(|(first, second)| {
            graph
                .entry(first)
                .or_insert_with(FxHashSet::default)
                .insert(second);
        });
    (graph, lines[(break_line + 1)..].to_vec())
}

fn is_valid<'a>(
    update: &Vec<&'a str>,
    graph: &FxHashMap<&str, FxHashSet<&str>>,
    seen: &mut FxHashSet<&'a str>,
) -> bool {
    seen.clear();
    update.iter().all(|&page| {
        if !graph.contains_key(page) {
            seen.insert(page);
            return true;
        }
        let result = seen.iter().all(|&prev| !graph[page].contains(prev));
        seen.insert(page);
        result
    })
}
