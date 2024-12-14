use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::Ordering;

pub fn solve1(lines: &[String]) -> i64 {
    let (graph, updates) = build_graph(lines);
    let mut seen = FxHashSet::default();
    updates
        .iter()
        .map(|update| {
            let split = update.split(",");
            (split.clone(), split.count())
        })
        .filter(|(update, _)| is_valid(update.clone(), &graph, &mut seen))
        .map(|(mut update, len)| {
            update
                .nth(len / 2)
                .map_or(0, |s| s.parse::<i64>().unwrap_or(0))
        })
        .sum()
}

pub fn solve2(lines: &[String]) -> i64 {
    let (graph, updates) = build_graph(lines);
    let mut seen = FxHashSet::default();
    updates
        .iter()
        .map(|update| {
            let split = update.split(",");
            (split.clone(), split.count())
        })
        .filter(|(update, _)| !is_valid(update.clone(), &graph, &mut seen))
        .map(|(update, len)| {
            update
                .sorted_unstable_by(
                    |&a, &b| match graph.contains_key(a) && graph[a].contains(b) {
                        true => Ordering::Less,
                        _ => Ordering::Greater,
                    },
                )
                .nth(len / 2)
                .map_or(0, |s| s.parse::<i64>().unwrap_or(0))
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
    mut update: impl Iterator<Item = &'a str>,
    graph: &FxHashMap<&str, FxHashSet<&str>>,
    seen: &mut FxHashSet<&'a str>,
) -> bool {
    seen.clear();
    update.all(|page| {
        if !graph.contains_key(page) {
            seen.insert(page);
            return true;
        }
        let result = seen.iter().all(|&prev| !graph[page].contains(prev));
        seen.insert(page);
        result
    })
}
