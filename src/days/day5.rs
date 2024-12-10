use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

pub fn solve1(lines: &Vec<String>) -> i64 {
    let (graph, updates) = build_graph(lines);
    let mut seen = HashSet::<&str>::new();
    updates
        .iter()
        .map(|update| update.split(",").collect_vec())
        .filter(|update| is_valid(update, &graph, &mut seen))
        .map(|update| update[update.len() / 2].parse::<i64>().unwrap())
        .sum()
}

pub fn solve2(lines: &Vec<String>) -> i64 {
    let (graph, updates) = build_graph(lines);
    let mut seen = HashSet::<&str>::new();
    updates
        .iter()
        .map(|update| update.split(",").collect_vec())
        .filter(|update| !is_valid(update, &graph, &mut seen))
        .map(|mut update| {
            update.sort_unstable_by(|&a, &b| {
                match graph.contains_key(a) && graph[a].contains(b) {
                    true => Ordering::Less,
                    _ => Ordering::Greater
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

fn is_valid<'a>(
    update: &Vec<&'a str>,
    graph: &HashMap<&str, HashSet<&str>>,
    seen: &mut HashSet<&'a str>,
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
