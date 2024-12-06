use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

pub fn solve1(lines: &Vec<String>) -> i64 {
    let (graph, updates) = build_graph(lines);
    let mut seen = HashSet::new();
    updates
        .iter()
        .map(|update| update.split(",").collect::<Vec<_>>())
        .filter(|update| {
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
        })
        .map(|update| update[update.len() / 2].parse::<i64>().unwrap())
        .sum()
}

pub fn solve2(lines: &Vec<String>) -> i64 {
    let (graph, updates) = build_graph(lines);
    let mut seen = HashSet::new();
    updates
        .iter()
        .map(|update| update.split(",").collect::<Vec<_>>())
        .filter(|update| {
            seen.clear();
            update.iter().any(|&page| {
                if graph.contains_key(page) && seen.iter().any(|&prev| graph[page].contains(prev)) {
                    true
                } else {
                    seen.insert(page);
                    false
                }
            })
        })
        .map(|mut update| {
            update.sort_unstable_by(|a, b| {
                if graph.contains_key(a) && graph[a].contains(b) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            update[update.len() / 2].parse::<i64>().unwrap()
        })
        .sum()
}

fn build_graph(lines: &Vec<String>) -> (HashMap<&str, HashSet<&str>>, Vec<String>) {
    let break_line = lines.iter().position(|l| l.is_empty()).unwrap();
    let mut graph = HashMap::new();
    lines[..break_line]
        .iter()
        .map(|rule| rule.split_once("|").unwrap())
        .for_each(|(first, second)| {
            graph
                .entry(first)
                .or_insert_with(HashSet::new)
                .insert(second);
        });
    (graph, lines[(break_line + 1)..].to_vec())
}
